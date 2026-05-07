
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
    pub limited: bool,
}

impl fmt::Display for Cone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cone")?;
        writeln!(f, "      apex: {}", self.apex)?;
        write!(f, "      angle: {:.3}", self.angle)
    }
}

impl Cone {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let height = self.normal.dot(&self.normal).sqrt();
        let normal_normalized = self.normal / height;
        
        let cos_theta = self.angle.cos();
        let cos_theta_sq = cos_theta * cos_theta;

        let s = ray.origin - self.apex;

        let s_dot_d = s.dot(&ray.direction);
        let d_dot_d = ray.direction.dot(&ray.direction);
        let s_dot_n = s.dot(&normal_normalized);
        let d_dot_n = ray.direction.dot(&normal_normalized);

        let a = d_dot_d * cos_theta_sq - d_dot_n * d_dot_n;
        let half_b = s_dot_d * cos_theta_sq - s_dot_n * d_dot_n;
        let c = s.dot(&s) * cos_theta_sq - s_dot_n * s_dot_n;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrt_d = discriminant.sqrt();
        let t1 = (-half_b - sqrt_d) / a;
        let t2 = (-half_b + sqrt_d) / a;

        let mut t_valid = None;
        for t_candidate in &[t1, t2] {
            if *t_candidate > epsilon {
                let point = ray.at(*t_candidate);
                let height_along_axis = (point - self.apex).dot(&normal_normalized);
                if height_along_axis > 0.0 && (!self.limited || height_along_axis < height) {
                    t_valid = Some(*t_candidate);
                    break;
                }
            }
        }

        if t_valid.is_none() && self.limited {
            let base_point = self.apex + normal_normalized * height;
            let denominator = ray.direction.dot(&normal_normalized);
            
            if denominator.abs() > epsilon {
                let t_base = (base_point - ray.origin).dot(&normal_normalized) / denominator;
                if t_base > epsilon {
                    let point = ray.at(t_base);
                    let v = point - base_point;
                    let dist_from_axis = (v.dot(&v)).sqrt();
                    let base_radius = height * self.angle.tan();
                    
                    if dist_from_axis <= base_radius {
                        return Some(HitRecord { point, normal: normal_normalized, t: t_base });
                    }
                }
            }
            return None;
        }

        let t = t_valid?;
        let point = ray.at(t);
        let v = point - self.apex;

        let v_dot_n = v.dot(&normal_normalized);

        let scale = v_dot_n / cos_theta_sq;
        let raw_normal = v - normal_normalized * scale;

        let normal = raw_normal.normalize();
        Some(HitRecord { point, normal, t })
    }
}
