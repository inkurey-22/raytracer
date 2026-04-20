use std::path::Path;

use crate::utilities::{Camera, Light, SceneConfig, Sphere, Vec3};

fn required_float(settings: &config::Config, key: &str) -> Result<f64, config::ConfigError> {
    settings.get_float(key).map_err(|_| {
        config::ConfigError::Message(format!("Missing or invalid required key: {key}"))
    })
}

fn required_vec3(settings: &config::Config, key: &str) -> Result<Vec3, config::ConfigError> {
    settings.get_table(key).map_err(|_| {
        config::ConfigError::Message(format!("Missing or invalid required key: {key}"))
    })?;

    Ok(Vec3 {
        x: required_float(settings, &format!("{key}.x"))?,
        y: required_float(settings, &format!("{key}.y"))?,
        z: required_float(settings, &format!("{key}.z"))?,
    })
}

fn value_to_vec3(value: config::Value, context: &str) -> Result<Vec3, config::ConfigError> {
    let table = value.into_table().map_err(|_| {
        config::ConfigError::Message(format!("Invalid {context}: expected a table"))
    })?;

    Ok(Vec3 {
        x: table
            .get("x")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message(format!("Missing {context}.x")))?
            .into_float()
            .map_err(|_| config::ConfigError::Message(format!("Invalid {context}.x")))?,
        y: table
            .get("y")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message(format!("Missing {context}.y")))?
            .into_float()
            .map_err(|_| config::ConfigError::Message(format!("Invalid {context}.y")))?,
        z: table
            .get("z")
            .cloned()
            .ok_or_else(|| config::ConfigError::Message(format!("Missing {context}.z")))?
            .into_float()
            .map_err(|_| config::ConfigError::Message(format!("Invalid {context}.z")))?,
    })
}

fn parse_light(light_value: config::Value, index: usize) -> Result<Light, config::ConfigError> {
    let light_table = light_value.into_table().map_err(|_| {
        config::ConfigError::Message(format!(
            "Invalid light entry at index {index}: expected a table"
        ))
    })?;

    let position = light_table
        .get("position")
        .cloned()
        .ok_or_else(|| config::ConfigError::Message(format!("Missing light[{index}].position")))?;

    let color = if let Some(color) = light_table.get("color") {
        value_to_vec3(color.clone(), &format!("light[{index}].color"))?
    } else {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    };

    let intensity = if let Some(intensity) = light_table.get("intensity") {
        intensity.clone().into_float().map_err(|_| {
            config::ConfigError::Message(format!("Invalid light[{index}].intensity"))
        })?
    } else {
        1000.0
    };

    Ok(Light {
        position: value_to_vec3(position, &format!("light[{index}].position"))?,
        color,
        intensity,
    })
}

fn parse_sphere(sphere_value: config::Value, index: usize) -> Result<Sphere, config::ConfigError> {
    let sphere_table = sphere_value.into_table().map_err(|_| {
        config::ConfigError::Message(format!(
            "Invalid sphere entry at index {index}: expected a table"
        ))
    })?;

    let center = sphere_table
        .get("center")
        .cloned()
        .ok_or_else(|| config::ConfigError::Message(format!("Missing sphere[{index}].center")))?;

    let radius = sphere_table
        .get("radius")
        .cloned()
        .ok_or_else(|| config::ConfigError::Message(format!("Missing sphere[{index}].radius")))?
        .into_float()
        .map_err(|_| config::ConfigError::Message(format!("Invalid sphere[{index}].radius")))?;

    Ok(Sphere {
        center: value_to_vec3(center, &format!("sphere[{index}].center"))?,
        radius,
    })
}

pub fn load_scene(config_path: &str) -> Result<SceneConfig, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::from(Path::new(config_path)))
        .build()?;

    settings.get_table("camera").map_err(|_| {
        config::ConfigError::Message("Missing required [camera] section".to_string())
    })?;

    let camera = Camera {
        fov: required_float(&settings, "camera.fov")?,
        position: required_vec3(&settings, "camera.position")?,
        direction: required_vec3(&settings, "camera.direction")?,
    };

    let lights = match settings.get_array("lights") {
        Ok(light_values) => light_values
            .into_iter()
            .enumerate()
            .map(|(index, light_value)| parse_light(light_value, index))
            .collect::<Result<Vec<_>, _>>()?,
        Err(_) => Vec::new(),
    };

    let spheres = match settings.get_array("spheres") {
        Ok(sphere_values) => sphere_values
            .into_iter()
            .enumerate()
            .map(|(index, sphere_value)| parse_sphere(sphere_value, index))
            .collect::<Result<Vec<_>, _>>()?,
        Err(_) => Vec::new(),
    };

    Ok(SceneConfig {
        camera,
        lights,
        spheres,
    })
}
