use std::fmt;

use color::Color;
use ray::{HitRecord, Ray};
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub color: Color,
    pub center: Vec3,
    pub radius: f64,
    pub reflectiveness: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Sphere {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let denom = a;
        let t1 = (-half_b - sqrt_d) / denom;
        let t2 = (-half_b + sqrt_d) / denom;
        let t = if t1 > epsilon {
            t1
        } else if t2 > epsilon {
            t2
        } else {
            return None;
        };
        let point = ray.at(t);
        let normal = (point - self.center).normalize();
        Some(HitRecord { point, normal, t })
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sphere")?;
        writeln!(f, "      center: {}", self.center)?;
        writeln!(f, "      radius: {:.3}", self.radius)?;
        writeln!(f, "      reflectiveness: {:.3}", self.reflectiveness)?;
        writeln!(f, "      transparency: {:.3}", self.transparency)?;
        writeln!(f, "      refractive_index: {:.3}", self.refractive_index)?;
        writeln!(f, "      color: {}", self.color)
    }
}
