mod args;
mod config_parse;
mod raytracing;
mod utilities;

use args::Args;
use raytracing::{render, write_ppm};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse().map_err(|msg| {
        eprintln!("{}", msg);
        std::process::exit(1);
    })?;

    let scene = config_parse::load_scene(&args.config)?;
    println!("{}", scene);

    let output_path = args.output.unwrap_or_else(|| {
        let config_path = Path::new(&args.config);
        let stem = config_path.file_stem().unwrap().to_string_lossy();
        format!("{}.ppm", stem)
    });

    let width = args.width.unwrap_or(scene.width);
    let height = args.height.unwrap_or(scene.height);

    println!("Rendering {}x{}...", width, height);
    let image = render(
        &scene.camera,
        &scene.omni_lights,
        &scene.spheres,
        &scene.planes,
        width,
        height,
    );

    println!("Writing to {}...", output_path);
    write_ppm(&output_path, &image)?;
    println!("Done!");

    Ok(())
}
