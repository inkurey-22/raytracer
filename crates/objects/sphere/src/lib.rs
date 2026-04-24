
use std::fmt;

use ray::{Ray, HitRecord};
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sphere")?;
        writeln!(f, "  center: {}", self.center)?;
        write!(f, "  radius: {:.3}", self.radius)
    }
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
