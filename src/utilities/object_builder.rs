use crate::utilities::value_reading::{
    get_bool, get_color, get_f64, get_orientation, get_percentage, get_string, get_vec3,
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
    transparency: f64,
    refractive_index: f64,
    level: usize,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    dimensions: Vec3,
    path: String,
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
            transparency: 0.0,
            refractive_index: 1.5,
            level: 0,
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(0.0, 0.0, 0.0),
            v2: Vec3::new(0.0, 0.0, 0.0),
            dimensions: Vec3::new(1.0, 1.0, 1.0),
            path: String::new(),
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
            "radius" | "size" => {
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
            "transparency" => {
                self.transparency = get_percentage(value)?;
            }
            "ior" | "refractive_index" | "index_of_refraction" => {
                self.refractive_index = get_f64(value)?;
            }
            "level" => {
                // level is an integer but parsed as number
                self.level = get_f64(value)? as usize;
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
            "dimensions" | "dimension" => {
                self.dimensions = get_vec3(value)?;
            "path" => {
                self.path = get_string(value)?;
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
                transparency: self.transparency,
                refractive_index: self.refractive_index,
            })),
            "cylinder" => Ok(object_interface::IObject::Cylinder(cylinder::Cylinder {
                color: self.color,
                center: self.position,
                radius: self.radius,
                normal: self.orientation.into_vec3(1.0),
                limited: self.limited,
                reflectiveness: self.reflectiveness,
                transparency: self.transparency,
                refractive_index: self.refractive_index,
            })),
            "cone" => Ok(object_interface::IObject::Cone(cone::Cone {
                color: self.color,
                apex: self.apex,
                angle: self.angle,
                normal: self.orientation.into_vec3(1.0),
                limited: self.limited,
                reflectiveness: self.reflectiveness,
                transparency: self.transparency,
                refractive_index: self.refractive_index,
            })),
            "plane" => Ok(object_interface::IObject::Plane(plane::Plane {
                color: self.color,
                point: self.position,
                normal: self.orientation.into_vec3(1.0),
                reflectiveness: self.reflectiveness,
                transparency: self.transparency,
                refractive_index: self.refractive_index,
            })),
            "menger" => Ok(object_interface::IObject::Menger(menger::Menger {
                color: self.color,
                position: self.position,
                size: self.radius,
                level: self.level,
                reflectiveness: self.reflectiveness,
                transparency: self.transparency,
                refractive_index: self.refractive_index,
            })),
            "triangle" => Ok(object_interface::IObject::Triangle(triangle::Triangle {
                color: self.color,
                v0: self.v0,
                v1: self.v1,
                v2: self.v2,
                reflectiveness: self.reflectiveness,
                transparency: self.transparency,
                refractive_index: self.refractive_index,
            })),
            "cuboid" => Ok(object_interface::IObject::Cuboid(cuboid::Cuboid {
                color: self.color,
                position: self.position,
                dimensions: self.dimensions,
                orientation: self.orientation.into_vec3(1.0),
                reflectiveness: self.reflectiveness,
            })),
            "obj_file" => Ok(object_interface::IObject::ObjFile(obj_file::ObjFile {
                path: self.path.clone(),
                center: self.position,
                orientation: self.orientation.into_vec3(1.0),
                reflectiveness: self.reflectiveness,
                transparency: self.transparency,
                refractive_index: self.refractive_index,
            })),

            other => Err(config::ConfigError::Message(format!(
                "Unknown object type: {other}"
            ))),
        }
    }
}
