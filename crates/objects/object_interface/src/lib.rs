use std::cmp::Ordering;

use color::Color;
use cone::Cone;
use cuboid::Cuboid;
use cylinder::Cylinder;
use menger::Menger;
use obj_file::ObjFile;
use plane::Plane;
use ray::{HitRecord, Ray};
use sphere::Sphere;
use triangle::Triangle;
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn surrounding_box(a: Aabb, b: Aabb) -> Aabb {
        Aabb {
            min: Vec3::new(
                a.min.x.min(b.min.x),
                a.min.y.min(b.min.y),
                a.min.z.min(b.min.z),
            ),
            max: Vec3::new(
                a.max.x.max(b.max.x),
                a.max.y.max(b.max.y),
                a.max.z.max(b.max.z),
            ),
        }
    }

    pub fn centroid(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        self.hit_interval(ray, t_min, t_max).is_some()
    }

    pub fn hit_interval(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(f64, f64)> {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for axis in 0..3 {
            let origin = match axis {
                0 => ray.origin.x,
                1 => ray.origin.y,
                _ => ray.origin.z,
            };
            let direction = match axis {
                0 => ray.direction.x,
                1 => ray.direction.y,
                _ => ray.direction.z,
            };
            let min = match axis {
                0 => self.min.x,
                1 => self.min.y,
                _ => self.min.z,
            };
            let max = match axis {
                0 => self.max.x,
                1 => self.max.y,
                _ => self.max.z,
            };

            if direction.abs() < 1e-12 {
                if origin < min || origin > max {
                    return None;
                }
                continue;
            }

            let inv_d = 1.0 / direction;
            let mut t0 = (min - origin) * inv_d;
            let mut t1 = (max - origin) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max < t_min {
                return None;
            }
        }

        Some((t_min, t_max))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub reflectiveness: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

#[derive(Debug, Clone)]
pub enum IObject {
    Plane(Plane),
    Sphere(Sphere),
    Cylinder(Cylinder),
    Cone(Cone),
    Menger(Menger),
    Triangle(Triangle),
    Cuboid(Cuboid),
    ObjFile(ObjFile),
}

impl IObject {
    pub fn intersect(&self, ray: &Ray, t_min: f64) -> Option<HitRecord> {
        match self {
            IObject::Plane(plane) => plane.intersect(ray, t_min),
            IObject::Sphere(sphere) => sphere.intersect(ray, t_min),
            IObject::Cylinder(cylinder) => cylinder.intersect(ray, t_min),
            IObject::Cone(cone) => cone.intersect(ray, t_min),
            IObject::Menger(m) => m.intersect(ray, t_min),
            IObject::Triangle(triangle) => triangle.intersect(ray, t_min),
            IObject::Cuboid(cuboid) => cuboid.intersect(ray, t_min),
            IObject::ObjFile(_) => panic!("should not be reached"),
        }
    }

    pub fn bounding_box(&self) -> Option<Aabb> {
        match self {
            IObject::Plane(_) => None,
            IObject::Sphere(sphere) => Some(Aabb::new(
                sphere.center - Vec3::new(sphere.radius, sphere.radius, sphere.radius),
                sphere.center + Vec3::new(sphere.radius, sphere.radius, sphere.radius),
            )),
            IObject::Cylinder(cylinder) => bounded_cylinder_aabb(cylinder),
            IObject::Cone(cone) => bounded_cone_aabb(cone),
            IObject::Menger(menger) => Some(Aabb::new(
                menger.position
                    - Vec3::new(menger.size * 0.5, menger.size * 0.5, menger.size * 0.5),
                menger.position
                    + Vec3::new(menger.size * 0.5, menger.size * 0.5, menger.size * 0.5),
            )),
            IObject::Triangle(triangle) => Some(triangle_aabb(triangle)),
            IObject::Cuboid(cuboid) => Some(cuboid_aabb(cuboid)),
            IObject::ObjFile(_) => None,
        }
    }

    pub fn get_color(&self) -> Color {
        self.material().color
    }

    pub fn get_reflectiveness(&self) -> f64 {
        self.material().reflectiveness
    }

    pub fn get_transparency(&self) -> f64 {
        self.material().transparency
    }

    pub fn get_refractive_index(&self) -> f64 {
        self.material().refractive_index
    }

    pub fn split_into_triangles(&self) -> Option<Vec<IObject>> {
        match self {
            IObject::ObjFile(obj_file) => obj_file
                .split_into_triangles()
                .map(|triangles| triangles.into_iter().map(IObject::Triangle).collect()),
            _ => panic!("should not be reached"),
        }
    }

    pub fn material(&self) -> Material {
        match self {
            IObject::Plane(plane) => Material {
                color: plane.color,
                reflectiveness: plane.reflectiveness,
                transparency: plane.transparency,
                refractive_index: plane.refractive_index,
            },
            IObject::Sphere(sphere) => Material {
                color: sphere.color,
                reflectiveness: sphere.reflectiveness,
                transparency: sphere.transparency,
                refractive_index: sphere.refractive_index,
            },
            IObject::Cylinder(cylinder) => Material {
                color: cylinder.color,
                reflectiveness: cylinder.reflectiveness,
                transparency: cylinder.transparency,
                refractive_index: cylinder.refractive_index,
            },
            IObject::Cone(cone) => Material {
                color: cone.color,
                reflectiveness: cone.reflectiveness,
                transparency: cone.transparency,
                refractive_index: cone.refractive_index,
            },
            IObject::Menger(m) => Material {
                color: m.color,
                reflectiveness: m.reflectiveness,
                transparency: m.transparency,
                refractive_index: m.refractive_index,
            },
            IObject::Triangle(triangle) => Material {
                color: triangle.color,
                reflectiveness: triangle.reflectiveness,
                transparency: triangle.transparency,
                refractive_index: triangle.refractive_index,
            },
            IObject::Cuboid(cuboid) => Material {
                color: cuboid.color,
                reflectiveness: cuboid.reflectiveness,
                transparency: cuboid.transparency,
                refractive_index: cuboid.refractive_index,
            },
            IObject::ObjFile(_) => panic!("should not be reached"),
        }
    }
}

pub trait ObjectQuery {
    fn closest_hit(&self, ray: &Ray, t_min: f64) -> Option<(HitRecord, Material)>;
    fn is_occluded(&self, ray: &Ray, max_distance: f64) -> bool;
}

#[derive(Debug, Clone)]
pub struct BvhScene {
    root: Option<Box<BvhNode>>,
    linear_objects: Option<Vec<IObject>>,
    unbounded_objects: Vec<IObject>,
}

impl BvhScene {
    pub fn new(objects: Vec<IObject>) -> Self {
        let mut bounded = Vec::new();
        let mut unbounded_objects = Vec::new();

        for object in objects {
            match object.bounding_box() {
                Some(bounds) => bounded.push(BvhItem {
                    bounds,
                    centroid: bounds.centroid(),
                    object,
                }),
                None => unbounded_objects.push(object),
            }
        }

        const BVH_MIN_OBJECTS: usize = 32;
        if bounded.len() <= BVH_MIN_OBJECTS {
            return Self {
                root: None,
                linear_objects: Some(
                    bounded
                        .into_iter()
                        .map(|item| item.object)
                        .chain(unbounded_objects.into_iter())
                        .collect(),
                ),
                unbounded_objects: Vec::new(),
            };
        }

        Self {
            root: BvhNode::build(bounded),
            linear_objects: None,
            unbounded_objects,
        }
    }
}

impl ObjectQuery for BvhScene {
    fn closest_hit(&self, ray: &Ray, t_min: f64) -> Option<(HitRecord, Material)> {
        if let Some(objects) = &self.linear_objects {
            return objects.as_slice().closest_hit(ray, t_min);
        }

        let mut closest_t = f64::INFINITY;
        let mut closest_hit = self
            .root
            .as_ref()
            .and_then(|node| node.closest_hit(ray, t_min, &mut closest_t));

        for object in &self.unbounded_objects {
            if let Some(hit) = object.intersect(ray, t_min)
                && hit.t < closest_t
            {
                closest_t = hit.t;
                closest_hit = Some((hit, object.material()));
            }
        }

        closest_hit
    }

    fn is_occluded(&self, ray: &Ray, max_distance: f64) -> bool {
        if let Some(objects) = &self.linear_objects {
            return objects.as_slice().is_occluded(ray, max_distance);
        }

        if self
            .root
            .as_ref()
            .is_some_and(|node| node.is_occluded(ray, max_distance))
        {
            return true;
        }

        self.unbounded_objects.iter().any(|object| {
            object.get_transparency() <= 0.0
                && object
                    .intersect(ray, 1e-6)
                    .is_some_and(|hit| hit.t < max_distance)
        })
    }
}

impl ObjectQuery for [IObject] {
    fn closest_hit(&self, ray: &Ray, t_min: f64) -> Option<(HitRecord, Material)> {
        let mut closest_t = f64::INFINITY;
        let mut closest_hit = None;

        for object in self {
            if let Some(hit) = object.intersect(ray, t_min)
                && hit.t < closest_t
            {
                closest_t = hit.t;
                closest_hit = Some((hit, object.material()));
            }
        }

        closest_hit
    }

    fn is_occluded(&self, ray: &Ray, max_distance: f64) -> bool {
        self.iter().any(|object| {
            object.get_transparency() <= 0.0
                && object
                    .intersect(ray, 1e-6)
                    .is_some_and(|hit| hit.t < max_distance)
        })
    }
}

#[derive(Debug, Clone)]
struct BvhItem {
    bounds: Aabb,
    centroid: Vec3,
    object: IObject,
}

#[derive(Debug, Clone)]
enum BvhNode {
    Leaf {
        bounds: Aabb,
        items: Vec<BvhItem>,
    },
    Branch {
        bounds: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl BvhNode {
    fn build(mut items: Vec<BvhItem>) -> Option<Box<BvhNode>> {
        if items.is_empty() {
            return None;
        }

        if items.len() <= 4 {
            let bounds = items.iter().skip(1).fold(items[0].bounds, |acc, item| {
                Aabb::surrounding_box(acc, item.bounds)
            });
            return Some(Box::new(BvhNode::Leaf { bounds, items }));
        }

        let centroid_bounds = items.iter().skip(1).fold(
            Aabb::new(items[0].centroid, items[0].centroid),
            |acc, item| Aabb {
                min: Vec3::new(
                    acc.min.x.min(item.centroid.x),
                    acc.min.y.min(item.centroid.y),
                    acc.min.z.min(item.centroid.z),
                ),
                max: Vec3::new(
                    acc.max.x.max(item.centroid.x),
                    acc.max.y.max(item.centroid.y),
                    acc.max.z.max(item.centroid.z),
                ),
            },
        );
        let extent = centroid_bounds.max - centroid_bounds.min;
        let axis = if extent.x >= extent.y && extent.x >= extent.z {
            0
        } else if extent.y >= extent.z {
            1
        } else {
            2
        };

        items.sort_unstable_by(|a, b| {
            compare_f64(axis_value(a.centroid, axis), axis_value(b.centroid, axis))
        });
        let mid = items.len() / 2;
        let right_items = items.split_off(mid);
        let left = BvhNode::build(items)?;
        let right = BvhNode::build(right_items)?;
        let bounds = Aabb::surrounding_box(left.bounds(), right.bounds());

        Some(Box::new(BvhNode::Branch {
            bounds,
            left,
            right,
        }))
    }

    fn bounds(&self) -> Aabb {
        match self {
            BvhNode::Leaf { bounds, .. } | BvhNode::Branch { bounds, .. } => *bounds,
        }
    }

    fn closest_hit(
        &self,
        ray: &Ray,
        t_min: f64,
        closest_t: &mut f64,
    ) -> Option<(HitRecord, Material)> {
        if !self.bounds().hit(ray, t_min, *closest_t) {
            return None;
        }

        match self {
            BvhNode::Leaf { items, .. } => {
                let mut best_hit = None;

                for item in items {
                    if let Some(hit) = item.object.intersect(ray, t_min)
                        && hit.t < *closest_t
                    {
                        *closest_t = hit.t;
                        best_hit = Some((hit, item.object.material()));
                    }
                }

                best_hit
            }
            BvhNode::Branch { left, right, .. } => {
                let left_interval = left.bounds().hit_interval(ray, t_min, *closest_t);
                let right_interval = right.bounds().hit_interval(ray, t_min, *closest_t);

                match (left_interval, right_interval) {
                    (Some((left_t, _)), Some((right_t, _))) => {
                        if left_t <= right_t {
                            let left_hit = left.closest_hit(ray, t_min, closest_t);
                            let right_hit = right.closest_hit(ray, t_min, closest_t);
                            match (left_hit, right_hit) {
                                (Some(left_hit), Some(right_hit)) => {
                                    if left_hit.0.t <= right_hit.0.t {
                                        Some(left_hit)
                                    } else {
                                        Some(right_hit)
                                    }
                                }
                                (Some(hit), None) | (None, Some(hit)) => Some(hit),
                                (None, None) => None,
                            }
                        } else {
                            let right_hit = right.closest_hit(ray, t_min, closest_t);
                            let left_hit = left.closest_hit(ray, t_min, closest_t);
                            match (right_hit, left_hit) {
                                (Some(right_hit), Some(left_hit)) => {
                                    if right_hit.0.t <= left_hit.0.t {
                                        Some(right_hit)
                                    } else {
                                        Some(left_hit)
                                    }
                                }
                                (Some(hit), None) | (None, Some(hit)) => Some(hit),
                                (None, None) => None,
                            }
                        }
                    }
                    (Some(_), None) => left.closest_hit(ray, t_min, closest_t),
                    (None, Some(_)) => right.closest_hit(ray, t_min, closest_t),
                    (None, None) => None,
                }
            }
        }
    }

    fn is_occluded(&self, ray: &Ray, max_distance: f64) -> bool {
        if !self.bounds().hit(ray, 1e-6, max_distance) {
            return false;
        }

        match self {
            BvhNode::Leaf { items, .. } => items.iter().any(|item| {
                item.object.get_transparency() <= 0.0
                    && item
                        .object
                        .intersect(ray, 1e-6)
                        .is_some_and(|hit| hit.t < max_distance)
            }),
            BvhNode::Branch { left, right, .. } => {
                let left_interval = left.bounds().hit_interval(ray, 1e-6, max_distance);
                let right_interval = right.bounds().hit_interval(ray, 1e-6, max_distance);

                match (left_interval, right_interval) {
                    (Some((left_t, _)), Some((right_t, _))) => {
                        if left_t <= right_t {
                            left.is_occluded(ray, max_distance)
                                || right.is_occluded(ray, max_distance)
                        } else {
                            right.is_occluded(ray, max_distance)
                                || left.is_occluded(ray, max_distance)
                        }
                    }
                    (Some(_), None) => left.is_occluded(ray, max_distance),
                    (None, Some(_)) => right.is_occluded(ray, max_distance),
                    (None, None) => false,
                }
            }
        }
    }
}

fn axis_value(v: Vec3, axis: usize) -> f64 {
    match axis {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    }
}

fn compare_f64(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

fn triangle_aabb(triangle: &Triangle) -> Aabb {
    Aabb::new(
        Vec3::new(
            triangle.v0.x.min(triangle.v1.x).min(triangle.v2.x),
            triangle.v0.y.min(triangle.v1.y).min(triangle.v2.y),
            triangle.v0.z.min(triangle.v1.z).min(triangle.v2.z),
        ),
        Vec3::new(
            triangle.v0.x.max(triangle.v1.x).max(triangle.v2.x),
            triangle.v0.y.max(triangle.v1.y).max(triangle.v2.y),
            triangle.v0.z.max(triangle.v1.z).max(triangle.v2.z),
        ),
    )
}

fn cuboid_aabb(cuboid: &Cuboid) -> Aabb {
    let half_dims = cuboid.dimensions * 0.5;
    let (sx, cx) = cuboid.orientation.x.sin_cos();
    let (sy, cy) = cuboid.orientation.y.sin_cos();
    let (sz, cz) = cuboid.orientation.z.sin_cos();

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

    let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

    for corner in [
        Vec3::new(-half_dims.x, -half_dims.y, -half_dims.z),
        Vec3::new(-half_dims.x, -half_dims.y, half_dims.z),
        Vec3::new(-half_dims.x, half_dims.y, -half_dims.z),
        Vec3::new(-half_dims.x, half_dims.y, half_dims.z),
        Vec3::new(half_dims.x, -half_dims.y, -half_dims.z),
        Vec3::new(half_dims.x, -half_dims.y, half_dims.z),
        Vec3::new(half_dims.x, half_dims.y, -half_dims.z),
        Vec3::new(half_dims.x, half_dims.y, half_dims.z),
    ] {
        let world_corner = cuboid.position + to_world(corner);
        min.x = min.x.min(world_corner.x);
        min.y = min.y.min(world_corner.y);
        min.z = min.z.min(world_corner.z);
        max.x = max.x.max(world_corner.x);
        max.y = max.y.max(world_corner.y);
        max.z = max.z.max(world_corner.z);
    }

    Aabb::new(min, max)
}

fn bounded_cylinder_aabb(cylinder: &Cylinder) -> Option<Aabb> {
    if !cylinder.limited {
        return None;
    }

    let axis = cylinder.normal.normalize();
    let top = cylinder.center + axis;
    let radius = cylinder.radius.abs();

    Some(Aabb::new(
        Vec3::new(
            cylinder.center.x.min(top.x) - radius,
            cylinder.center.y.min(top.y) - radius,
            cylinder.center.z.min(top.z) - radius,
        ),
        Vec3::new(
            cylinder.center.x.max(top.x) + radius,
            cylinder.center.y.max(top.y) + radius,
            cylinder.center.z.max(top.z) + radius,
        ),
    ))
}

fn bounded_cone_aabb(cone: &Cone) -> Option<Aabb> {
    if !cone.limited {
        return None;
    }

    let axis = cone.normal.normalize();
    let height = cone.normal.length();
    let base_center = cone.apex + axis * height;
    let base_radius = (height * cone.angle.tan()).abs();

    Some(Aabb::new(
        Vec3::new(
            cone.apex.x.min(base_center.x) - base_radius,
            cone.apex.y.min(base_center.y) - base_radius,
            cone.apex.z.min(base_center.z) - base_radius,
        ),
        Vec3::new(
            cone.apex.x.max(base_center.x) + base_radius,
            cone.apex.y.max(base_center.y) + base_radius,
            cone.apex.z.max(base_center.z) + base_radius,
        ),
    ))
}

impl std::fmt::Display for IObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IObject::Plane(plane) => write!(f, "{}", plane),
            IObject::Sphere(sphere) => write!(f, "{}", sphere),
            IObject::Cylinder(cylinder) => write!(f, "{}", cylinder),
            IObject::Cone(cone) => write!(f, "{}", cone),
            IObject::Menger(m) => write!(f, "{}", m),
            IObject::Triangle(triangle) => write!(f, "{}", triangle),
            IObject::Cuboid(cuboid) => write!(f, "{}", cuboid),
            IObject::ObjFile(obj_file) => write!(f, "{}", obj_file),
        }
    }
}
