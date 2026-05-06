
use std::fmt;

use ray::{Ray, HitRecord};
use vec3::Vec3;
use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub color: Color,
    pub normal: Vec3,
    pub limited: bool,
}

impl fmt::Display for Cylinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cylinder")?;
        writeln!(f, "      center: {}", self.center)?;
        write!(f, "      radius: {:.3}", self.radius)?;
        writeln!(f, "      limited: {}", self.limited)
    }
}

impl Cylinder {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let axis = self.normal.normalize();
        let oc = ray.origin - self.center;
        let d = ray.direction;
        
        let a = d.dot(&d) - (d.dot(&axis) * d.dot(&axis));
        let b = 2.0 * (oc.dot(&d) - (oc.dot(&axis) * d.dot(&axis)));
        let c = oc.dot(&oc) - (oc.dot(&axis) * oc.dot(&axis)) - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrt_d = discriminant.sqrt();
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);
        let t = if t1 > epsilon {
            t1
        } else if t2 > epsilon {
            t2
        } else {
            return None;
        };
        
        let point = ray.at(t);
        let oc_hit = point - self.center;
        if self.limited {
            let height_on_axis = oc_hit.dot(&axis);
            if height_on_axis < 0.0 || height_on_axis > 1.0 {
                return None;
            }
        }
        let normal = (oc_hit - (axis * oc_hit.dot(&axis))).normalize();
        Some(HitRecord { point, normal, t })
    }
}
