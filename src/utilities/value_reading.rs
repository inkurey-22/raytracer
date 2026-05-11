use color::Color;
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

pub fn get_bool(value: config::Value) -> Result<bool, config::ConfigError> {
    value
        .into_bool()
        .map_err(|_| config::ConfigError::Message("Expected a boolean value".to_string()))
}

pub fn get_vec3(value: config::Value) -> Result<Vec3, config::ConfigError> {
    let table = value
        .into_table()
        .map_err(|_| config::ConfigError::Message("Expected a table for Vec3".to_string()))?;

    let has_domain_axes =
        table.contains_key("depth") || table.contains_key("right") || table.contains_key("up");
    let has_xyz_axes =
        table.contains_key("x") || table.contains_key("y") || table.contains_key("z");

    if has_domain_axes && has_xyz_axes {
        return Err(config::ConfigError::Message(
            "Vec3 must use either {depth,right,up} or {x,y,z}, not both".to_string(),
        ));
    }

    let (x_key, y_key, z_key) = if has_domain_axes {
        ("depth", "right", "up")
    } else {
        ("x", "y", "z")
    };

    fn read_component(
        table: &config::Map<String, config::Value>,
        key: &str,
    ) -> Result<f64, config::ConfigError> {
        table
            .get(key)
            .cloned()
            .ok_or_else(|| config::ConfigError::Message(format!("Missing Vec3.{key}")))?
            .into_float()
            .map_err(|_| config::ConfigError::Message(format!("Invalid Vec3.{key}")))
    }

    Ok(Vec3 {
        x: read_component(&table, x_key)?,
        y: read_component(&table, y_key)?,
        z: read_component(&table, z_key)?,
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
