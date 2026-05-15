use std::fmt;

use color::Color;
use ray::{HitRecord, Ray};
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub color: Color,
    pub reflectiveness: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Triangle {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > epsilon {
            Some(HitRecord {
                point: ray.origin + ray.direction * t,
                normal: -edge1.cross(&edge2).normalize(),
                t: t,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Triangle")?;
        writeln!(f, "      v0: {}", self.v0)?;
        writeln!(f, "      v1: {}", self.v1)?;
        writeln!(f, "      v2: {}", self.v2)?;
        writeln!(f, "      reflectiveness: {:.3}", self.reflectiveness)?;
        writeln!(f, "      transparency: {:.3}", self.transparency)?;
        writeln!(f, "      refractive_index: {:.3}", self.refractive_index)?;
        writeln!(f, "      color: {}", self.color)
    }
}
