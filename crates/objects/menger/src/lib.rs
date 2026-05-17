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
        let half = self.size / 2.0;
        let min = self.position - Vec3::new(half, half, half);
        let max = self.position + Vec3::new(half, half, half);
        self.intersect_recursive(min, max, capped_level, ray, epsilon, f64::INFINITY)
    }

    fn intersect_recursive(
        &self,
        min: Vec3,
        max: Vec3,
        level: usize,
        ray: &Ray,
        epsilon: f64,
        t_max: f64,
    ) -> Option<HitRecord> {
        let (t_enter, t_exit) = ray_aabb_intersect(ray, min, max, epsilon)?;
        if t_enter > t_max {
            return None;
        }

        if level == 0 {
            let t = if t_enter > epsilon { t_enter } else { t_exit };
            if t <= epsilon || t > t_max {
                return None;
            }
            let point = ray.at(t);
            let normal = aabb_normal(&point, &min, &max, epsilon);
            return Some(HitRecord { point, normal, t });
        }

        let child_size = (max.x - min.x) / 3.0;
        let mut candidates = Vec::with_capacity(20);

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let count = [i, j, k].iter().filter(|&&x| x == 1).count();
                    if count >= 2 {
                        continue;
                    }

                    let offset = Vec3::new(
                        i as f64 * child_size,
                        j as f64 * child_size,
                        k as f64 * child_size,
                    );
                    let child_min = min + offset;
                    let child_max = child_min + Vec3::new(child_size, child_size, child_size);

                    if let Some((child_enter, _)) =
                        ray_aabb_intersect(ray, child_min, child_max, epsilon)
                        && child_enter <= t_max
                    {
                        candidates.push(ChildCandidate {
                            entry: child_enter,
                            min: child_min,
                            max: child_max,
                        });
                    }
                }
            }
        }

        candidates.sort_unstable_by(|a, b| {
            a.entry
                .partial_cmp(&b.entry)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut closest: Option<HitRecord> = None;
        let mut best_t = t_max;

        for candidate in candidates {
            if candidate.entry > best_t {
                break;
            }

            if let Some(hit) = self.intersect_recursive(
                candidate.min,
                candidate.max,
                level - 1,
                ray,
                epsilon,
                best_t,
            ) {
                best_t = hit.t;
                closest = Some(hit);
            }
        }

        closest
    }
}

#[derive(Clone, Copy)]
struct ChildCandidate {
    entry: f64,
    min: Vec3,
    max: Vec3,
}

fn ray_aabb_intersect(ray: &Ray, min: Vec3, max: Vec3, epsilon: f64) -> Option<(f64, f64)> {
    let (mut tmin, mut tmax) = {
        if ray.direction.x.abs() < epsilon {
            if ray.origin.x < min.x || ray.origin.x > max.x {
                return None;
            }
            (f64::NEG_INFINITY, f64::INFINITY)
        } else {
            let inv_dx = 1.0 / ray.direction.x;
            let mut t1 = (min.x - ray.origin.x) * inv_dx;
            let mut t2 = (max.x - ray.origin.x) * inv_dx;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            (t1, t2)
        }
    };

    let (tymin, tymax) = {
        if ray.direction.y.abs() < epsilon {
            if ray.origin.y < min.y || ray.origin.y > max.y {
                return None;
            }
            (f64::NEG_INFINITY, f64::INFINITY)
        } else {
            let inv_dy = 1.0 / ray.direction.y;
            let mut t1 = (min.y - ray.origin.y) * inv_dy;
            let mut t2 = (max.y - ray.origin.y) * inv_dy;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            (t1, t2)
        }
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
        if ray.direction.z.abs() < epsilon {
            if ray.origin.z < min.z || ray.origin.z > max.z {
                return None;
            }
            (f64::NEG_INFINITY, f64::INFINITY)
        } else {
            let inv_dz = 1.0 / ray.direction.z;
            let mut t1 = (min.z - ray.origin.z) * inv_dz;
            let mut t2 = (max.z - ray.origin.z) * inv_dz;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            (t1, t2)
        }
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

    if tmax > epsilon {
        Some((tmin, tmax))
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
