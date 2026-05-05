use std::fmt;

use color::Color;
use ray::{HitRecord, Ray};
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub color: Color,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < epsilon {
            return None;
        }
        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        if t > epsilon {
            let point = ray.at(t);
            let normal = self.normal;
            Some(HitRecord { point, normal, t })
        } else {
            None
        }
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Plane")?;
        writeln!(f, "      point: {}", self.point)?;
        writeln!(f, "      normal: {}", self.normal)?;
        writeln!(f, "      color: {}", self.color)
    }
}
