
use std::fmt;

use ray::{Ray, HitRecord};
use vec3::Vec3;
use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Cone {
    pub apex: Vec3,
    pub angle: f64,
    pub color: Color,
    pub normal: Vec3,
}

impl fmt::Display for Cone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cone")?;
        writeln!(f, "  apex: {}", self.apex)?;
        write!(f, "  angle: {:.3}", self.angle)
    }
}

impl Cone {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let cos_theta = self.angle.cos();
        let cos_theta_sq = cos_theta * cos_theta;

        let s = ray.origin - self.apex;

        let s_dot_d = s.dot(&ray.direction);
        let d_dot_d = ray.direction.dot(&ray.direction);
        let s_dot_n = s.dot(&self.normal);
        let d_dot_n = ray.direction.dot(&self.normal);

        let a = d_dot_d * cos_theta_sq - d_dot_n * d_dot_n;
        let b = 2.0 * (s_dot_d * cos_theta_sq - s_dot_n * d_dot_n);
        let c = s.dot(&s) * cos_theta_sq - s_dot_n * s_dot_n;

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
        if (point - self.apex).dot(&self.normal) <= 0.0 {
            return None;
        }
        let v = point - self.apex;
        let cos_theta = self.angle.cos();
        let cos_theta_sq = cos_theta * cos_theta;

        let v_dot_n = v.dot(&self.normal);

        let scale = v_dot_n / cos_theta_sq;
        let raw_normal = v - self.normal * scale;

        let normal = raw_normal.normalize();
        Some(HitRecord { point, normal, t })
    }
}
