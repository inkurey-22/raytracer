use crate::utilities::value_reading::{
    get_bool, get_color, get_f64, get_orientation, get_percentage, get_vec3,
};
use color::Color;
use orientation::{Orientation, Vec3OrientationExt};
use vec3::Vec3;

pub struct ObjectBuilder {
    color: Color,
    position: Vec3,
    orientation: Orientation,
    radius: f64,
    angle: f64,
    apex: Vec3,
    limited: bool,
    reflectiveness: f64,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            orientation: Orientation::new(0.0, 0.0, 0.0),
            radius: 1.0,
            angle: 0.0,
            apex: Vec3::new(0.0, 0.0, 0.0),
            limited: false,
            reflectiveness: 0.0,
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(0.0, 0.0, 0.0),
            v2: Vec3::new(0.0, 0.0, 0.0),
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
            "angle" => {
                self.angle = get_f64(value)?;
            }
            "apex" => {
                self.apex = get_vec3(value)?;
            }
            "limited" => {
                self.limited = get_bool(value)?;
            }
            "reflectiveness" | "reflectivity" | "reflective" => {
                self.reflectiveness = get_percentage(value)?;
            }
            "normal" | "orientation" | "direction" => match get_vec3(value.clone()) {
                Ok(vec) => self.orientation = vec.into_orientation(),
                Err(_) => {
                    self.orientation = get_orientation(value)?;
                }
            },
            "v0" | "vertex0" => {
                self.v0 = get_vec3(value)?;
            }
            "v1" | "vertex1" => {
                self.v1 = get_vec3(value)?;
            }
            "v2" | "vertex2" => {
                self.v2 = get_vec3(value)?;
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
                reflectiveness: self.reflectiveness,
            })),
            "cylinder" => Ok(object_interface::IObject::Cylinder(cylinder::Cylinder {
                color: self.color,
                center: self.position,
                radius: self.radius,
                normal: self.orientation.into_vec3(1.0),
                limited: self.limited,
                reflectiveness: self.reflectiveness,
            })),
            "cone" => Ok(object_interface::IObject::Cone(cone::Cone {
                color: self.color,
                apex: self.apex,
                angle: self.angle,
                normal: self.orientation.into_vec3(1.0),
                limited: self.limited,
                reflectiveness: self.reflectiveness,
            })),
            "plane" => Ok(object_interface::IObject::Plane(plane::Plane {
                color: self.color,
                point: self.position,
                normal: self.orientation.into_vec3(1.0),
                reflectiveness: self.reflectiveness,
            })),
            "triangle" => Ok(object_interface::IObject::Triangle(triangle::Triangle {
                color: self.color,
                v0: self.v0,
                v1: self.v1,
                v2: self.v2,
                reflectiveness: self.reflectiveness,
            })),
            other => Err(config::ConfigError::Message(format!(
                "Unknown object type: {other}"
            ))),
        }
    }
}
