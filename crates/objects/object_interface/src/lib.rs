use color::Color;
use plane::Plane;
use ray::{HitRecord, Ray};
use sphere::Sphere;

#[derive(Debug, Clone)]
pub enum IObject {
    Plane(Plane),
    Sphere(Sphere),
}

impl IObject {
    pub fn intersect(&self, ray: &Ray, t_min: f64) -> Option<HitRecord> {
        match self {
            IObject::Plane(plane) => plane.intersect(ray, t_min),
            IObject::Sphere(sphere) => sphere.intersect(ray, t_min),
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            IObject::Plane(plane) => plane.color,
            IObject::Sphere(sphere) => sphere.color,
        }
    }
}

impl std::fmt::Display for IObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IObject::Plane(plane) => write!(f, "{}", plane),
            IObject::Sphere(sphere) => write!(f, "{}", sphere),
        }
    }
}
