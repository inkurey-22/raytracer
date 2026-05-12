use std::fmt;

use color::Color;
use ray::{HitRecord, Ray};
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub color: Color,
    pub normal: Vec3,
    pub limited: bool,
    pub reflectiveness: f64,
}

impl fmt::Display for Cylinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cylinder")?;
        writeln!(f, "      center: {}", self.center)?;
        writeln!(f, "      radius: {:.3}", self.radius)?;
        writeln!(f, "      limited: {}", self.limited)?;
        writeln!(f, "      normal: {}", self.normal)?;
        writeln!(f, "      reflectiveness: {:.3}", self.reflectiveness)?;
        writeln!(f, "      color: {}", self.color)
    }
}

impl Cylinder {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let axis = self.normal.normalize();
        let oc = ray.origin - self.center;
        let d = ray.direction;

        let mut closest_hit: Option<(f64, Vec3)> = None;

        let a = d.dot(&d) - (d.dot(&axis) * d.dot(&axis));
        let half_b = oc.dot(&d) - (oc.dot(&axis) * d.dot(&axis));
        let c = oc.dot(&oc) - (oc.dot(&axis) * oc.dot(&axis)) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            let t1 = (-half_b - sqrt_d) / a;
            let t2 = (-half_b + sqrt_d) / a;
            let t = if t1 > epsilon {
                t1
            } else if t2 > epsilon {
                t2
            } else {
                f64::INFINITY
            };

            if t != f64::INFINITY {
                let point = ray.at(t);
                let oc_hit = point - self.center;
                let height = oc_hit.dot(&axis);
                if !self.limited || (height >= 0.0 && height <= 1.0) {
                    closest_hit = Some((t, point));
                }
            }
        }

        if self.limited {
            let denom = d.dot(&axis);
            if denom.abs() > 1e-6 {
                let t = -oc.dot(&axis) / denom;
                if t > epsilon {
                    let point = ray.at(t);
                    let radial_dist =
                        (point - self.center - axis * (point - self.center).dot(&axis)).length();
                    if radial_dist <= self.radius {
                        if closest_hit.is_none() || t < closest_hit.unwrap().0 {
                            closest_hit = Some((t, point));
                        }
                    }
                }
            }

            if denom.abs() > 1e-6 {
                let t = (axis - oc).dot(&axis) / denom;
                if t > epsilon {
                    let point = ray.at(t);
                    let radial_dist =
                        (point - self.center - axis * (point - self.center).dot(&axis)).length();
                    if radial_dist <= self.radius {
                        if closest_hit.is_none() || t < closest_hit.unwrap().0 {
                            closest_hit = Some((t, point));
                        }
                    }
                }
            }
        }

        closest_hit.map(|(t, point)| {
            let oc_hit = point - self.center;
            let height = oc_hit.dot(&axis);
            let normal = if height <= 0.0 {
                -axis
            } else if height >= 1.0 {
                axis
            } else {
                (oc_hit - axis * height).normalize()
            };
            HitRecord { point, normal, t }
        })
    }
}
