use std::env;

mod config_parse;
mod utilities;

fn main() -> Result<(), config::ConfigError> {
    let config_path = env::args().nth(1).ok_or_else(|| {
        config::ConfigError::Message("Usage: raytracer <path/to/config.toml>".to_string())
    })?;

    let scene = config_parse::load_scene(&config_path)?;
    println!("{}", scene);
    Ok(())
}
