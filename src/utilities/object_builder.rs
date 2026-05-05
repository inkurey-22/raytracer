use crate::utilities::value_reading::{get_color, get_f64, get_vec3};
use color::Color;
use vec3::Vec3;

pub struct ObjectBuilder {
    color: Color,
    position: Vec3,
    normal: Vec3,
    radius: f64,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
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
            "normal" => {
                self.normal = get_vec3(value)?;
            }
            "radius" => {
                self.radius = get_f64(value)?;
            }

            _ => {
                return Err(config::ConfigError::Message(format!(
                    "Unknown object attribute: {key}"
                )));
            }
        }
        Ok(())
    }

    pub fn build(
        &self,
        object_type: &str,
    ) -> Result<object_interface::IObject, config::ConfigError> {
        match object_type {
            "sphere" => Ok(object_interface::IObject::Sphere(sphere::Sphere {
                color: self.color,
                center: self.position,
                radius: self.radius,
            })),
            "plane" => Ok(object_interface::IObject::Plane(plane::Plane {
                color: self.color,
                point: self.position,
                normal: self.normal,
            })),
            other => Err(config::ConfigError::Message(format!(
                "Unknown object type: {other}"
            ))),
        }
    }
}
