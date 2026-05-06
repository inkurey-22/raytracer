use std::env;

#[derive(Debug, Clone)]
pub struct Args {
    pub config: String,
    pub output: Option<String>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub samples_per_pixel: Option<usize>,
    pub variance_threshold: Option<f64>,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err(Self::help());
        }

        let mut config = String::new();
        let mut output = None;
        let mut width = None;
        let mut height = None;
        let mut samples_per_pixel = None;
        let mut variance_threshold = None;

        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "-h" | "--help" => return Err(Self::help()),
                "-o" | "--output" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(format!("{} requires a value", arg));
                    }
                    output = Some(args[i].clone());
                }
                "--width" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(format!("{} requires a value", arg));
                    }
                    width = Some(
                        args[i]
                            .parse()
                            .map_err(|_| format!("Invalid width: {}", args[i]))?,
                    );
                }
                "--height" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(format!("{} requires a value", arg));
                    }
                    height = Some(
                        args[i]
                            .parse()
                            .map_err(|_| format!("Invalid height: {}", args[i]))?,
                    );
                }
                "--samples" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(format!("{} requires a value", arg));
                    }
                    samples_per_pixel = Some(
                        args[i]
                            .parse()
                            .map_err(|_| format!("Invalid samples: {}", args[i]))?,
                    );
                }
                "--variance-threshold" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(format!("{} requires a value", arg));
                    }
                    variance_threshold = Some(
                        args[i]
                            .parse()
                            .map_err(|_| format!("Invalid variance threshold: {}", args[i]))?,
                    );
                }
                _ => {
                    if arg.starts_with('-') {
                        return Err(format!("Unknown option: {}", arg));
                    }
                    if config.is_empty() {
                        config = arg.clone();
                    } else {
                        return Err("Multiple config files specified".to_string());
                    }
                }
            }

            i += 1;
        }

        if config.is_empty() {
            return Err("Config file is required".to_string());
        }

        Ok(Args {
            config,
            output,
            width,
            height,
            samples_per_pixel,
            variance_threshold,
        })
    }

    fn help() -> String {
        r#"Usage: raytracer [OPTIONS] <CONFIG>

Arguments:
  <CONFIG>              Path to scene configuration file (TOML)

Options:
  -o, --output <FILE>           Output PPM filename (default: <CONFIG_NAME>.ppm)
  --width <SIZE>                Image width in pixels (default: from config)
  --height <SIZE>               Image height in pixels (default: from config)
  --samples <N>                 Samples per pixel for adaptive supersampling (default: 16)
  --variance-threshold <VAL>    Variance threshold for adaptive sampling (default: 0.01)
  -h, --help                    Show this help message

Examples:
  raytracer config/simple.toml
  raytracer config/example.toml -o render.ppm
  raytracer config/simple.toml --samples 32 --variance-threshold 0.005
  raytracer config/simple.toml --width 400 --height 300"#
            .to_string()
    }
}
