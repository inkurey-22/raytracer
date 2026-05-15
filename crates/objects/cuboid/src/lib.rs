use std::fmt;

use color::Color;
use ray::{HitRecord, Ray};
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    pub color: Color,
    pub position: Vec3,
    pub dimensions: Vec3,
    pub orientation: Vec3,
    pub reflectiveness: f64,
}

impl Cuboid {
    #[inline(always)]
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        let half_dims = self.dimensions * 0.5;

        let (sx, cx) = self.orientation.x.sin_cos();
        let (sy, cy) = self.orientation.y.sin_cos();
        let (sz, cz) = self.orientation.z.sin_cos();

        let to_local = |v: Vec3| {
            let (mut x, mut y, mut z) = (v.x, v.y, v.z);

            let y1 = y * cx + z * sx;
            let z1 = -y * sx + z * cx;
            y = y1;
            z = z1;

            let x1 = x * cy - z * sy;
            let z2 = x * sy + z * cy;
            x = x1;
            z = z2;

            let x2 = x * cz + y * sz;
            let y2 = -x * sz + y * cz;
            Vec3::new(x2, y2, z)
        };

        let to_world = |v: Vec3| {
            let (mut x, mut y, mut z) = (v.x, v.y, v.z);

            let x1 = x * cz - y * sz;
            let y1 = x * sz + y * cz;
            x = x1;
            y = y1;

            let x2 = x * cy + z * sy;
            let z2 = -x * sy + z * cy;
            x = x2;
            z = z2;

            let y2 = y * cx - z * sx;
            let z3 = y * sx + z * cx;
            Vec3::new(x, y2, z3)
        };

        let origin = to_local(ray.origin - self.position);
        let direction = to_local(ray.direction);

        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;

        for (o, d, h) in [
            (origin.x, direction.x, half_dims.x),
            (origin.y, direction.y, half_dims.y),
            (origin.z, direction.z, half_dims.z),
        ] {
            if d.abs() < epsilon {
                if o.abs() > h {
                    return None;
                }
                continue;
            }

            let inv_d = 1.0 / d;
            let mut t1 = (-h - o) * inv_d;
            let mut t2 = (h - o) * inv_d;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }

            tmin = tmin.max(t1);
            tmax = tmax.min(t2);

            if tmin > tmax {
                return None;
            }
        }

        let distance = if tmin >= epsilon {
            tmin
        } else if tmax >= epsilon {
            tmax
        } else {
            return None;
        };

        let local_hit = origin + direction * distance;
        let world_hit = self.position + to_world(local_hit);
        let local_normal = {
            let delta = 1e-6;

            if (local_hit.x - half_dims.x).abs() < delta {
                Vec3::new(1.0, 0.0, 0.0)
            } else if (local_hit.x + half_dims.x).abs() < delta {
                Vec3::new(-1.0, 0.0, 0.0)
            } else if (local_hit.y - half_dims.y).abs() < delta {
                Vec3::new(0.0, 1.0, 0.0)
            } else if (local_hit.y + half_dims.y).abs() < delta {
                Vec3::new(0.0, -1.0, 0.0)
            } else if (local_hit.z - half_dims.z).abs() < delta {
                Vec3::new(0.0, 0.0, 1.0)
            } else {
                Vec3::new(0.0, 0.0, -1.0)
            }
        };
        let world_normal = to_world(local_normal);

        Some(HitRecord {
            point: world_hit,
            normal: world_normal,
            t: distance,
        })
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cuboid")?;
        writeln!(f, "      position: {}", self.position)?;
        writeln!(f, "      dimensions: {}", self.dimensions)?;
        writeln!(f, "      orientation: {}", self.orientation)?;
        writeln!(f, "      reflectiveness: {:.3}", self.reflectiveness)?;
        writeln!(f, "      color: {}", self.color)
    }
}
