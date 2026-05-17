mod args;
mod config_parse;
mod raytracing;
mod utilities;

use args::Args;
use color::Color;
use raytracing::{SamplingConfig, render_rows_multithreaded, write_ppm};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

fn perform_local_render(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let scene = config_parse::load_scene(&args.config)?;
    println!("{}", scene);

    let output_path = args.output.clone().unwrap_or_else(|| {
        let config_path = Path::new(&args.config);
        let stem = config_path.file_stem().unwrap().to_string_lossy();
        format!("{}.ppm", stem)
    });

    let width = args.width.unwrap_or(scene.width);
    let height = args.height.unwrap_or(scene.height);

    let sampling_config = SamplingConfig {
        samples_per_pixel: args.samples_per_pixel.unwrap_or(16),
        variance_threshold: args.variance_threshold.unwrap_or(0.01),
    };

    println!(
        "Rendering {}x{} with adaptive supersampling...",
        width, height
    );
    println!("  Samples per pixel: {}", sampling_config.samples_per_pixel);
    println!(
        "  Variance threshold: {}",
        sampling_config.variance_threshold
    );

    let image = render_rows_multithreaded(
        &scene.camera,
        width,
        height,
        &scene.lights,
        &scene.objects,
        sampling_config,
        0,
        height,
    );

    println!("Writing to {}...", output_path);
    write_ppm(&output_path, &image)?;
    println!("Done!");

    Ok(())
}

fn start_server(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let bind_addr = "0.0.0.0:25565";
    let listener = TcpListener::bind(bind_addr)?;
    listener.set_nonblocking(true).ok();
    println!("Server listening on {}", bind_addr);

    let clients = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
    let clients_accept = clients.clone();

    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_accept = shutdown.clone();

    let listener_clone = listener.try_clone()?;
    let accept_thread = thread::spawn(move || {
        while !shutdown_accept.load(Ordering::SeqCst) {
            match listener_clone.accept() {
                Ok((s, addr)) => {
                    if let Ok(peer) = s.peer_addr() {
                        println!("Client connected: {}", peer);
                    } else {
                        println!("Client connected: {}", addr);
                    }
                    s.set_nodelay(true).ok();
                    if let Ok(clone) = s.try_clone() {
                        clients_accept.lock().unwrap().push(clone);
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }
                    eprintln!("Accept error: {}", e);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    });

    println!("Commands: start  -> divide image into row-tasks and dispatch to clients");
    println!("          clients -> show connected clients");
    println!("          quit   -> exit server");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let cmd = line.unwrap_or_default();
        match cmd.trim() {
            "clients" => {
                let guard = clients.lock().unwrap();
                println!("Connected clients: {}", guard.len());
            }
            "start" => {
                // load scene
                if args.config.is_empty() {
                    eprintln!("No config file provided to server; cannot render.");
                    continue;
                }
                let scene = match config_parse::load_scene(&args.config) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Failed to load scene: {}", e);
                        continue;
                    }
                };

                let width = args.width.unwrap_or(scene.width);
                let height = args.height.unwrap_or(scene.height);
                let sampling_config = SamplingConfig {
                    samples_per_pixel: args.samples_per_pixel.unwrap_or(16),
                    variance_threshold: args.variance_threshold.unwrap_or(0.01),
                };

                let guard = clients.lock().unwrap();
                let client_count = guard.len();
                if client_count == 0 {
                    println!("No clients connected; performing local render");
                    if let Err(e) = perform_local_render(&args) {
                        eprintln!("Local render failed: {}", e);
                    }
                    println!("Render complete; shutting down server...");
                    break;
                }

                println!("Dispatching tasks to {} clients...", client_count);

                // prepare shared image buffer
                let image: Arc<Mutex<Vec<Vec<Color>>>> =
                    Arc::new(Mutex::new(vec![vec![Color::default(); width]; height]));

                // partition rows: include server as one worker (last chunk)
                let workers = client_count + 1;
                let rows_per = height.div_ceil(workers);

                let mut handles = Vec::new();

                for (i, s) in guard.iter().enumerate() {
                    let task_id = i as u32;
                    let start_row = i * rows_per;
                    let end_row = ((i + 1) * rows_per).min(height);
                    if start_row >= end_row {
                        continue;
                    }

                    // clone stream for this task
                    let mut stream = match s.try_clone() {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("Failed to clone client stream: {}", e);
                            continue;
                        }
                    };

                    // send TASK header: TASK <id> <start> <end> <width> <height>\n
                    let header = format!(
                        "TASK {} {} {} {} {}\n",
                        task_id, start_row, end_row, width, height
                    );
                    if let Err(e) = stream.write_all(header.as_bytes()) {
                        eprintln!("Failed to send TASK to client {}: {}", task_id, e);
                        continue;
                    }
                    if let Err(e) = stream.flush() {
                        eprintln!("Flush error to client {}: {}", task_id, e);
                        continue;
                    }

                    // spawn thread to wait for RESULT
                    let image_clone = image.clone();
                    let mut reader = BufReader::new(stream.try_clone()?);

                    let handle = thread::spawn(move || {
                        let mut line = String::new();
                        if let Err(e) = reader.read_line(&mut line) {
                            eprintln!("Error reading RESULT header: {}", e);
                            return;
                        }
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() != 3 || parts[0] != "RESULT" {
                            eprintln!("Invalid RESULT header: {}", line);
                            return;
                        }
                        let tid: u32 = parts[1].parse().unwrap_or(0);
                        let bytes_len: usize = parts[2].parse().unwrap_or(0);
                        if bytes_len == 0 {
                            eprintln!("Client {} reported zero-length result", tid);
                            return;
                        }

                        let mut buf = vec![0u8; bytes_len];
                        if let Err(e) = reader.read_exact(&mut buf) {
                            eprintln!(
                                "Failed to read {} bytes from client {}: {}",
                                bytes_len, tid, e
                            );
                            return;
                        }

                        // decode bytes into pixels and write into image
                        let mut offset = 0usize;
                        let mut img = image_clone.lock().unwrap();
                        let start = (tid as usize) * rows_per;
                        let end = ((tid as usize + 1) * rows_per).min(height);
                        for y in start..end {
                            for x in 0..width {
                                if offset + 3 > buf.len() {
                                    break;
                                }
                                let r = buf[offset] as f64 / 255.0;
                                let g = buf[offset + 1] as f64 / 255.0;
                                let b = buf[offset + 2] as f64 / 255.0;
                                offset += 3;
                                img[y][x] = Color::new(r, g, b);
                            }
                        }
                    });

                    handles.push(handle);
                }

                drop(guard); // release client lock

                // server's own chunk
                let server_start = client_count * rows_per;
                let server_end = height;
                if server_start < server_end {
                    println!(
                        "Server rendering rows {}..{} using multithreading",
                        server_start, server_end
                    );
                    let rows = raytracing::render_rows_multithreaded(
                        &scene.camera,
                        width,
                        height,
                        &scene.lights,
                        &scene.objects,
                        sampling_config.clone(),
                        server_start,
                        server_end,
                    );
                    let mut img = image.lock().unwrap();
                    for (i, row) in rows.into_iter().enumerate() {
                        img[server_start + i] = row;
                    }
                }

                // wait for clients
                for h in handles {
                    let _ = h.join();
                }

                // write final image
                let out = args.output.clone().unwrap_or_else(|| {
                    let config_path = Path::new(&args.config);
                    let stem = config_path.file_stem().unwrap().to_string_lossy();
                    format!("{}.ppm", stem)
                });
                let final_img = Arc::try_unwrap(image).unwrap().into_inner().unwrap();
                println!("Writing to {}...", out);
                if let Err(e) = raytracing::write_ppm(&out, &final_img) {
                    eprintln!("Failed to write image: {}", e);
                } else {
                    println!("Done!");

                    // notify clients to shutdown
                    if let Ok(mut guard) = clients.lock() {
                        for s in guard.iter_mut() {
                            let _ = s.write_all(b"SHUTDOWN\n");
                            let _ = s.flush();
                        }
                    }

                    // signal accept thread to stop
                    shutdown.store(true, Ordering::SeqCst);

                    break;
                }
            }
            "quit" | "exit" => {
                println!("Shutting down server...");
                break;
            }
            "" => {}
            other => println!(
                "Unknown command: {} (available: start, clients, quit)",
                other
            ),
        }
    }

    drop(clients);
    let _ = accept_thread.join();
    Ok(())
}

fn run_client(server_addr: &str, args: Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to server at {}...", server_addr);
    let mut stream = TcpStream::connect(server_addr)?;
    println!("Connected to server.");
    stream.set_nodelay(true).ok();

    if args.config.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No config provided to client; cannot render tasks",
        )
        .into());
    }

    let scene = config_parse::load_scene(&args.config)?;

    let mut reader = BufReader::new(stream.try_clone()?);
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            println!("Server closed connection");
            break;
        }
        let cmd = line.trim();
        if cmd.starts_with("TASK") {
            // TASK <id> <start> <end> <width> <height>
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.len() != 6 {
                eprintln!("Invalid TASK header: {}", cmd);
                continue;
            }
            let task_id: u32 = parts[1].parse().unwrap_or(0);
            let start_row: usize = parts[2].parse().unwrap_or(0);
            let end_row: usize = parts[3].parse().unwrap_or(0);
            let width: usize = parts[4].parse().unwrap_or(0);
            let height: usize = parts[5].parse().unwrap_or(0);

            println!("Received TASK {} rows {}..{}", task_id, start_row, end_row);

            let sampling_config = SamplingConfig {
                samples_per_pixel: args.samples_per_pixel.unwrap_or(16),
                variance_threshold: args.variance_threshold.unwrap_or(0.01),
            };

            let rows = raytracing::render_rows_multithreaded(
                &scene.camera,
                width,
                height,
                &scene.lights,
                &scene.objects,
                sampling_config,
                start_row,
                end_row,
            );

            // serialize rows as raw RGB bytes
            let mut buf: Vec<u8> = Vec::new();
            for row in rows.iter() {
                for pixel in row.iter() {
                    let c = pixel.saturate();
                    let r = (c.r * 255.0) as u8;
                    let g = (c.g * 255.0) as u8;
                    let b = (c.b * 255.0) as u8;
                    buf.push(r);
                    buf.push(g);
                    buf.push(b);
                }
            }

            // send RESULT <task_id> <len>\n<raw bytes>
            let header = format!("RESULT {} {}\n", task_id, buf.len());
            if let Err(e) = stream.write_all(header.as_bytes()) {
                eprintln!("Failed to send RESULT header: {}", e);
                continue;
            }
            if let Err(e) = stream.write_all(&buf) {
                eprintln!("Failed to send RESULT payload: {}", e);
                continue;
            }
            if let Err(e) = stream.flush() {
                eprintln!("Flush error: {}", e);
            }
        } else if cmd == "SHUTDOWN" {
            println!("Server requested shutdown; exiting client.");
            break;
        } else {
            println!("Unknown command from server: {}", cmd);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse().map_err(|msg| {
        eprintln!("{}", msg);
        std::process::exit(1);
    })?;

    if args.server {
        return start_server(args);
    }

    if let Some(server_addr) = args.client.clone() {
        return run_client(&server_addr, args);
    }

    // Default: local render as before
    perform_local_render(&args)
}
