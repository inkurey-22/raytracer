use std::fmt;

use color::Color;
use ray::{HitRecord, Ray};
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Cone {
    pub apex: Vec3,
    pub angle: f64,
    pub color: Color,
    pub normal: Vec3,
    pub limited: bool,
    pub reflectiveness: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl fmt::Display for Cone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cone")?;
        writeln!(f, "      apex: {}", self.apex)?;
        writeln!(f, "      angle: {:.3}", self.angle)?;
        writeln!(f, "      normal: {}", self.normal)?;
        writeln!(f, "      reflectiveness: {:.3}", self.reflectiveness)?;
        writeln!(f, "      transparency: {:.3}", self.transparency)?;
        writeln!(f, "      refractive_index: {:.3}", self.refractive_index)?;
        writeln!(f, "      color: {}", self.color)
    }
}

impl Cone {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let height = self.normal.dot(&self.normal).sqrt();
        if height <= epsilon {
            return None;
        }
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

        let mut t_valid = None;
        if a.abs() > epsilon {
            let mut discriminant = half_b * half_b - a * c;
            if discriminant >= -epsilon {
                if discriminant < 0.0 {
                    discriminant = 0.0;
                }
                let sqrt_d = discriminant.sqrt();
                let t1 = (-half_b - sqrt_d) / a;
                let t2 = (-half_b + sqrt_d) / a;
                for &t_candidate in &[t1, t2] {
                    if !t_candidate.is_finite() || t_candidate <= epsilon {
                        continue;
                    }
                    let point = ray.at(t_candidate);
                    let height_along_axis = (point - self.apex).dot(&normal_normalized);
                    if height_along_axis > epsilon && (!self.limited || height_along_axis < height)
                    {
                        t_valid = Some(t_candidate);
                        break;
                    }
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
                    let radial = v - normal_normalized * v.dot(&normal_normalized);
                    let dist_from_axis = radial.length();
                    let base_radius = height * self.angle.tan();

                    if dist_from_axis <= base_radius {
                        return Some(HitRecord {
                            point,
                            normal: normal_normalized,
                            t: t_base,
                        });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_axis_parallel_ray_returns_stable_result() {
        let cone = Cone {
            apex: Vec3::new(0.0, 0.0, 0.0),
            angle: std::f64::consts::FRAC_PI_4,
            color: Color::new(1.0, 1.0, 1.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            limited: false,
            reflectiveness: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        };
        let ray = Ray::new(Vec3::new(3.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        if let Some(hit) = cone.intersect(&ray, 1e-6) {
            assert!(hit.t.is_finite());
            assert!(hit.normal.x.is_finite());
            assert!(hit.normal.y.is_finite());
            assert!(hit.normal.z.is_finite());
        }
    }
}
