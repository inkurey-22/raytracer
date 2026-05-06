use crate::utilities::value_reading::{get_color, get_f64, get_orientation, get_vec3};
use color::Color;
use orientation::{Orientation, Vec3OrientationExt};
use vec3::Vec3;

pub struct ObjectBuilder {
    color: Color,
    position: Vec3,
    orientation: Orientation,
    radius: f64,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            orientation: Orientation::new(0.0, 0.0, 0.0),
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
            "radius" => {
                self.radius = get_f64(value)?;
            }
            "normal" | "orientation" | "direction" => match get_vec3(value.clone()) {
                Ok(vec) => self.orientation = vec.into_orientation(),
                Err(_) => {
                    self.orientation = get_orientation(value)?;
                }
            },

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
            "cylinder" => Ok(object_interface::IObject::Cylinder(cylinder::Cylinder {
                color: self.color,
                center: self.position,
                radius: self.radius,
                normal: self.orientation.into_vec3(1.0),
            })),
            "plane" => Ok(object_interface::IObject::Plane(plane::Plane {
                color: self.color,
                point: self.position,
                normal: self.orientation.into_vec3(1.0),
            })),
            other => Err(config::ConfigError::Message(format!(
                "Unknown object type: {other}"
            ))),
        }
    }
}
