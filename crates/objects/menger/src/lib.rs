use std::fmt;

use color::Color;
use ray::{HitRecord, Ray};
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Menger {
    pub color: Color,
    pub position: Vec3,
    pub size: f64,
    pub level: usize,
    pub reflectiveness: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl fmt::Display for Menger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Menger")?;
        writeln!(f, "      position: {}", self.position)?;
        writeln!(f, "      size: {:.3}", self.size)?;
        writeln!(f, "      level: {}", self.level)?;
        writeln!(f, "      reflectiveness: {:.3}", self.reflectiveness)?;
        writeln!(f, "      transparency: {:.3}", self.transparency)?;
        writeln!(f, "      refractive_index: {:.3}", self.refractive_index)?;
        writeln!(f, "      color: {}", self.color)
    }
}

impl Menger {
    pub fn intersect(&self, ray: &Ray, epsilon: f64) -> Option<HitRecord> {
        // Cap level to avoid explosion from overly large levels
        let capped_level = self.level.min(5);
        self.intersect_recursive(self.position, self.size, capped_level, ray, epsilon)
    }

    fn intersect_recursive(
        &self,
        center: Vec3,
        size: f64,
        level: usize,
        ray: &Ray,
        epsilon: f64,
    ) -> Option<HitRecord> {
        let half = size / 2.0;
        let min = center - Vec3::new(half, half, half);
        let max = center + Vec3::new(half, half, half);

        // Quick reject using parent AABB
        if ray_aabb_intersect(ray, min, max, epsilon).is_none() {
            return None;
        }

        if level == 0 {
            if let Some(t) = ray_aabb_intersect(ray, min, max, epsilon) {
                let point = ray.at(t);
                let normal = aabb_normal(&point, &min, &max, epsilon);
                return Some(HitRecord { point, normal, t });
            }
            return None;
        }

        let mut closest: Option<HitRecord> = None;
        let mut closest_t = f64::INFINITY;
        let new_size = size / 3.0;

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let count = [i, j, k].iter().filter(|&&x| x == 1).count();
                    if count >= 2 {
                        continue; // remove center and face centers
                    }

                    let offset = Vec3::new(
                        (i as f64 - 1.0) * new_size,
                        (j as f64 - 1.0) * new_size,
                        (k as f64 - 1.0) * new_size,
                    );
                    let sub_center = center + offset;

                    if let Some(hit) =
                        self.intersect_recursive(sub_center, new_size, level - 1, ray, epsilon)
                    {
                        if hit.t < closest_t {
                            closest_t = hit.t;
                            closest = Some(hit);
                        }
                    }
                }
            }
        }

        closest
    }
}

fn ray_aabb_intersect(ray: &Ray, min: Vec3, max: Vec3, epsilon: f64) -> Option<f64> {
    let (mut tmin, mut tmax) = {
        let inv_dx = 1.0 / ray.direction.x;
        let mut t1 = (min.x - ray.origin.x) * inv_dx;
        let mut t2 = (max.x - ray.origin.x) * inv_dx;
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }
        (t1, t2)
    };

    let (tymin, tymax) = {
        let inv_dy = 1.0 / ray.direction.y;
        let mut t1 = (min.y - ray.origin.y) * inv_dy;
        let mut t2 = (max.y - ray.origin.y) * inv_dy;
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }
        (t1, t2)
    };

    if (tmin > tymax) || (tymin > tmax) {
        return None;
    }

    if tymin > tmin {
        tmin = tymin;
    }
    if tymax < tmax {
        tmax = tymax;
    }

    let (tzmin, tzmax) = {
        let inv_dz = 1.0 / ray.direction.z;
        let mut t1 = (min.z - ray.origin.z) * inv_dz;
        let mut t2 = (max.z - ray.origin.z) * inv_dz;
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }
        (t1, t2)
    };

    if (tmin > tzmax) || (tzmin > tmax) {
        return None;
    }

    if tzmin > tmin {
        tmin = tzmin;
    }
    if tzmax < tmax {
        tmax = tzmax;
    }

    let t_candidate = if tmin > epsilon { tmin } else { tmax };
    if t_candidate > epsilon {
        Some(t_candidate)
    } else {
        None
    }
}

fn aabb_normal(point: &Vec3, min: &Vec3, max: &Vec3, _epsilon: f64) -> Vec3 {
    let dx_min = (point.x - min.x).abs();
    let dx_max = (point.x - max.x).abs();
    let dy_min = (point.y - min.y).abs();
    let dy_max = (point.y - max.y).abs();
    let dz_min = (point.z - min.z).abs();
    let dz_max = (point.z - max.z).abs();

    let min_dist = dx_min
        .min(dx_max)
        .min(dy_min)
        .min(dy_max)
        .min(dz_min)
        .min(dz_max);

    if (dx_min - min_dist).abs() < 1e-9 {
        return Vec3::new(-1.0, 0.0, 0.0);
    }
    if (dx_max - min_dist).abs() < 1e-9 {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    if (dy_min - min_dist).abs() < 1e-9 {
        return Vec3::new(0.0, -1.0, 0.0);
    }
    if (dy_max - min_dist).abs() < 1e-9 {
        return Vec3::new(0.0, 1.0, 0.0);
    }
    if (dz_min - min_dist).abs() < 1e-9 {
        return Vec3::new(0.0, 0.0, -1.0);
    }
    // dz_max
    Vec3::new(0.0, 0.0, 1.0)
}
