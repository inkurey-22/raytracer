use std::path::Path;

use crate::utilities::{
    Camera, SceneConfig, light_builder, object_builder,
    value_reading::{self, get_value_at},
};

pub fn load_scene(config_path: &str) -> Result<SceneConfig, config::ConfigError> {
    let settings = load_settings(config_path)?;

    if settings.get_table("camera").is_err() {
        return Err(config::ConfigError::Message(
            "Missing required [camera] section".to_string(),
        ));
    }

    let camera = Camera {
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

    let mut objects: Vec<object_interface::IObject> = Vec::new();
    if let Ok(objects_table) = settings.get_table("objects") {
        for (object_type, object_attributes) in objects_table {
            let object_entries = match object_attributes.clone().into_array() {
                Ok(entries) => entries,
                Err(_) => vec![object_attributes],
            };
            for object_entry in object_entries {
                let mut builder = object_builder::ObjectBuilder::new();
                for (attribute_key, attribute_value) in object_entry.into_table()? {
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

    let mut lights: Vec<light_interface::ILight> = Vec::new();
    if let Ok(lights_table) = settings.get_table("lights") {
        for (light_type, light_attributes) in lights_table {
            let light_entries = match light_attributes.clone().into_array() {
                Ok(entries) => entries,
                Err(_) => vec![light_attributes],
            };
            for light_entry in light_entries {
                let mut builder = light_builder::LightBuilder::new();
                for (attribute_key, attribute_value) in light_entry.into_table()? {
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

    let ambiant_count = lights
        .iter()
        .filter(|light| matches!(light, light_interface::ILight::AmbiantLight(_)))
        .count();
    if ambiant_count > 1 {
        return Err(config::ConfigError::Message(
            "Only one ambient light is allowed.".to_string(),
        ));
    }

    Ok(SceneConfig {
        camera,
        lights,
        objects,
        width,
        height,
    })
}

fn load_settings(config_path: &str) -> Result<config::Config, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()?;

    let mut complete_settings_builder = config::Config::builder();
    complete_settings_builder =
        complete_settings_builder.add_source(config::File::from(Path::new(config_path)));

    if let Ok(scene_list) = get_value_at(&settings, "scenes.list") {
        for scene_path in scene_list.into_array()? {
            let scene_path_str = scene_path.into_string().map_err(|e| {
                config::ConfigError::Message(format!("Scene path '{:?}' is not a string", e))
            })?;
            let full_scene_path = Path::new(config_path)
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .join(scene_path_str);
            println!("Loading scene from: {}", full_scene_path.display());
            complete_settings_builder = complete_settings_builder.add_source(config::File::from(
                Path::new(full_scene_path.to_str().unwrap()),
            ));
        }
    }
    complete_settings_builder.build()
}
