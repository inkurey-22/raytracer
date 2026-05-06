use color::Color;
use orientation::Orientation;
use vec3::Vec3;

pub fn get_value_at(
    settings: &config::Config,
    key: &str,
) -> Result<config::Value, config::ConfigError> {
    settings
        .get(key)
        .map_err(|_| config::ConfigError::Message(format!("Missing required key: {key}")))
}

pub fn get_f64(value: config::Value) -> Result<f64, config::ConfigError> {
    value
        .into_float()
        .map_err(|_| config::ConfigError::Message("Expected a floating-point number".to_string()))
}

pub fn get_vec3(value: config::Value) -> Result<Vec3, config::ConfigError> {
    let table = value
        .into_table()
        .map_err(|_| config::ConfigError::Message("Expected a table for Vec3".to_string()))?;

    Ok(Vec3 {
        x: table
            .get("x")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message("Missing Vec3.x".to_string()))?
            .into_float()
            .map_err(|_| config::ConfigError::Message("Invalid Vec3.x".to_string()))?,
        y: table
            .get("y")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message("Missing Vec3.y".to_string()))?
            .into_float()
            .map_err(|_| config::ConfigError::Message("Invalid Vec3.y".to_string()))?,
        z: table
            .get("z")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message("Missing Vec3.z".to_string()))?
            .into_float()
            .map_err(|_| config::ConfigError::Message("Invalid Vec3.z".to_string()))?,
    })
}

pub fn get_color(value: config::Value) -> Result<Color, config::ConfigError> {
    fn to_channel(value: f64) -> f64 {
        if value > 1.0 { value / 255.0 } else { value }
    }

    let table = value
        .into_table()
        .map_err(|_| config::ConfigError::Message("Expected a table for Color".to_string()))?;

    Ok(Color {
        r: to_channel(
            table
                .get("r")
                .cloned()
                .ok_or_else(|| config::ConfigError::Message("Missing Color.r".to_string()))?
                .into_float()
                .map_err(|_| config::ConfigError::Message("Invalid Color.r".to_string()))?,
        ),
        g: to_channel(
            table
                .get("g")
                .cloned()
                .ok_or_else(|| config::ConfigError::Message("Missing Color.g".to_string()))?
                .into_float()
                .map_err(|_| config::ConfigError::Message("Invalid Color.g".to_string()))?,
        ),
        b: to_channel(
            table
                .get("b")
                .cloned()
                .ok_or_else(|| config::ConfigError::Message("Missing Color.b".to_string()))?
                .into_float()
                .map_err(|_| config::ConfigError::Message("Invalid Color.b".to_string()))?,
        ),
    })
}

pub fn get_orientation(value: config::Value) -> Result<Orientation, config::ConfigError> {
    let table = value.into_table().map_err(|_| {
        config::ConfigError::Message("Expected a table for Orientation".to_string())
    })?;

    Ok(Orientation {
        p: table
            .get("p")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message("Missing Orientation.p".to_string()))?
            .into_float()
            .map_err(|_| config::ConfigError::Message("Invalid Orientation.p".to_string()))?,
        y: table
            .get("y")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message("Missing Orientation.y".to_string()))?
            .into_float()
            .map_err(|_| config::ConfigError::Message("Invalid Orientation.y".to_string()))?,
        r: table
            .get("r")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message("Missing Orientation.r".to_string()))?
            .into_float()
            .map_err(|_| config::ConfigError::Message("Invalid Orientation.r".to_string()))?,
    })
}
