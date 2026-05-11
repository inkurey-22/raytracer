use crate::utilities::value_reading::{get_color, get_f64, get_orientation, get_vec3};
use color::Color;
use orientation::{Orientation, Vec3OrientationExt};
use vec3::Vec3;

pub struct LightBuilder {
    color: Color,
    position: Vec3,
    orientation: Orientation,
    intensity: f64,
}

impl LightBuilder {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            orientation: Orientation::new(-90.0, 0.0, 0.0),
            intensity: 1.0,
        }
    }

    pub fn set_attribute(
        &mut self,
        key: &str,
        value: config::Value,
    ) -> Result<(), config::ConfigError> {
        match key {
            "color" => {
                self.color = get_color(value)?;
            }
            "position" => {
                self.position = get_vec3(value)?;
            }
            "intensity" => {
                self.intensity = get_f64(value)?;
            }
            "normal" | "orientation" | "direction" => match get_vec3(value.clone()) {
                Ok(vec) => self.orientation = vec.into_orientation(),
                Err(_) => {
                    self.orientation = get_orientation(value)?;
                }
            },

            _ => {
                return Err(config::ConfigError::Message(format!(
                    "Unknown light attribute: {key}"
                )));
            }
        }
        Ok(())
    }

    pub fn build(&self, light_type: &str) -> Result<light_interface::ILight, config::ConfigError> {
        match light_type {
            "omni" => Ok(light_interface::ILight::OmniLight(omni_light::OmniLight {
                color: self.color,
                position: self.position,
                intensity: self.intensity,
            })),
            "ambiant" => Ok(light_interface::ILight::AmbiantLight(
                ambiant::AmbiantLight {
                    color: self.color,
                    intensity: self.intensity,
                },
            )),
            "directional" => Ok(light_interface::ILight::DirectionalLight(
                directional_light::DirectionalLight {
                    color: self.color,
                    direction: self.orientation.into_vec3(1.0),
                    intensity: self.intensity,
                },
            )),
            other => Err(config::ConfigError::Message(format!(
                "Unknown light type: {other}"
            ))),
        }
    }
}
