use std::path::Path;

use crate::utilities::{Camera, OmniLight, Plane, SceneConfig, Sphere, Vec3};

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

fn value_to_color(value: config::Value, context: &str) -> Result<Color, config::ConfigError> {
    let table = value.into_table().map_err(|_| {
        config::ConfigError::Message(format!("Invalid {context}: expected a table"))
    })?;

    let r = table
        .get("r")
        .cloned()
        .ok_or_else(|| config::ConfigError::Message(format!("Missing {context}.r")))?
        .into_float()
        .map_err(|_| config::ConfigError::Message(format!("Invalid {context}.r")))?
        / 255.0;

    let g = table
        .get("g")
        .cloned()
        .ok_or_else(|| config::ConfigError::Message(format!("Missing {context}.g")))?
        .into_float()
        .map_err(|_| config::ConfigError::Message(format!("Invalid {context}.g")))?
        / 255.0;

    let b = table
        .get("b")
        .cloned()
        .ok_or_else(|| config::ConfigError::Message(format!("Missing {context}.b")))?
        .into_float()
        .map_err(|_| config::ConfigError::Message(format!("Invalid {context}.b")))?
        / 255.0;

    Ok(Color { r, g, b })
}

fn parse_omni_light(
    light_value: config::Value,
    index: usize,
) -> Result<OmniLight, config::ConfigError> {
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
        value_to_color(color.clone(), &format!("light[{index}].color"))?
    } else {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    };

    let intensity = if let Some(intensity) = light_table.get("intensity") {
        intensity.clone().into_float().map_err(|_| {
            config::ConfigError::Message(format!("Invalid light[{index}].intensity"))
        })?
    } else {
        1000.0
    };

    Ok(OmniLight {
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

fn parse_plane(plane_value: config::Value, index: usize) -> Result<Plane, config::ConfigError> {
    let plane_table = plane_value.into_table().map_err(|_| {
        config::ConfigError::Message(format!(
            "Invalid plane entry at index {index}: expected a table"
        ))
    })?;

    let point = plane_table
        .get("point")
        .cloned()
        .ok_or_else(|| config::ConfigError::Message(format!("Missing plane[{index}].point")))?;

    let normal = plane_table
        .get("normal")
        .cloned()
        .ok_or_else(|| config::ConfigError::Message(format!("Missing plane[{index}].normal")))?;

    Ok(Plane::new(
        value_to_vec3(point, &format!("plane[{index}].point"))?,
        value_to_vec3(normal, &format!("plane[{index}].normal"))?,
    ))
}

pub fn load_scene(config_path: &str) -> Result<SceneConfig, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::from(Path::new(config_path)))
        .build()?;

    if settings.get_table("camera").is_err() {
        return Err(config::ConfigError::Message(
            "Missing required [camera] section".to_string(),
        ));
    }

    let camera: Camera = Camera {
        fov: value_reading::get_f64(get_value_at(&settings, "camera.fov")?)?,
        position: value_reading::get_vec3(get_value_at(&settings, "camera.position")?)?,
        direction: value_reading::get_vec3(get_value_at(&settings, "camera.direction")?)?,
    };

    let width = settings
        .get_int("render.width")
        .ok()
        .and_then(|w| usize::try_from(w).ok())
        .unwrap_or(800);
    let height = settings
        .get_int("render.height")
        .ok()
        .and_then(|h| usize::try_from(h).ok())
        .unwrap_or(600);

    let mut objects: Vec<object::Object> = Vec::new();

    if let Ok(objects_table) = settings.get_table("objects") {
        for (object_type, object_attributes) in objects_table.into_iter() {
            let object_entries = match object_attributes.clone().into_array() {
                Ok(entries) => entries,
                Err(_) => vec![object_attributes],
            };
            for object_entry in object_entries {
                let mut builder = object_builder::ObjectBuilder::new();
                for (attribute_key, attribute_value) in object_entry.into_table()?.into_iter() {
                    builder.set_attribute(&attribute_key, attribute_value)?;
                }
                match builder.build(&object_type) {
                    Err(e) => {
                        return Err(config::ConfigError::Message(format!(
                            "Error building object of type '{}': {}",
                            object_type, e
                        )));
                    }
                    Ok(obj) => objects.push(obj),
                }
            }
        }
    }

    let mut lights: Vec<light::Light> = Vec::new();

    if let Ok(lights_table) = settings.get_table("lights") {
        for (light_type, light_attributes) in lights_table.into_iter() {
            let light_entries = match light_attributes.clone().into_array() {
                Ok(entries) => entries,
                Err(_) => vec![light_attributes],
            };
            for light_entry in light_entries {
                let mut builder = light_builder::LightBuilder::new();
                for (attribute_key, attribute_value) in light_entry.into_table()?.into_iter() {
                    builder.set_attribute(&attribute_key, attribute_value)?;
                }
                match builder.build(&light_type) {
                    Err(e) => {
                        return Err(config::ConfigError::Message(format!(
                            "Error building light of type '{}': {}",
                            light_type, e
                        )));
                    }
                    Ok(obj) => lights.push(obj),
                }
            }
        }
    }

    return Ok(SceneConfig {
        camera: camera,
        lights: lights,
        objects: objects,
        width: width,
        height: height,
    });
}
