
use std::fmt;

use ray::{Ray, HitRecord};
use vec3::Vec3;
use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub color: Color,
}

impl fmt::Display for Cylinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cylinder")?;
        writeln!(f, "  center: {}", self.center)?;
        write!(f, "  radius: {:.3}", self.radius)
    }
}

impl Cylinder {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let vector = self.center + (Vec3{x : self.radius, y : 0.0, z : 0.0});
        let a = 1.0 - (vector.dot(&ray.direction) * vector.dot(&ray.direction));
        let half_b = 2.0 * ((ray.direction.dot(&(ray.origin - self.center))) - (ray.direction.dot(&vector) * (ray.origin - self.center).dot(&vector)));
        let c = (ray.origin - self.center).dot(&(ray.origin - self.center)) - ((ray.origin - self.center).dot(&vector) * ((ray.origin - self.center).dot(&vector))) - self.radius * self.radius;
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
