use color::Color;
use cone::Cone;
use cylinder::Cylinder;
use plane::Plane;
use ray::{HitRecord, Ray};
use sphere::Sphere;
use triangle::Triangle;

#[derive(Debug, Clone)]
pub enum IObject {
    Plane(Plane),
    Sphere(Sphere),
    Cylinder(Cylinder),
    Cone(Cone),
    Triangle(Triangle),
}

impl IObject {
    pub fn intersect(&self, ray: &Ray, t_min: f64) -> Option<HitRecord> {
        match self {
            IObject::Plane(plane) => plane.intersect(ray, t_min),
            IObject::Sphere(sphere) => sphere.intersect(ray, t_min),
            IObject::Cylinder(cylinder) => cylinder.intersect(ray, t_min),
            IObject::Cone(cone) => cone.intersect(ray, t_min),
            IObject::Triangle(triangle) => triangle.intersect(ray, t_min),
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            IObject::Plane(plane) => plane.color,
            IObject::Sphere(sphere) => sphere.color,
            IObject::Cylinder(cylinder) => cylinder.color,
            IObject::Cone(cone) => cone.color,
            IObject::Triangle(triangle) => triangle.color,
        }
    }

    pub fn get_reflectiveness(&self) -> f64 {
        match self {
            IObject::Plane(plane) => plane.reflectiveness,
            IObject::Sphere(sphere) => sphere.reflectiveness,
            IObject::Cylinder(cylinder) => cylinder.reflectiveness,
            IObject::Cone(cone) => cone.reflectiveness,
            IObject::Triangle(triangle) => triangle.reflectiveness,
        }
    }
}

impl std::fmt::Display for IObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IObject::Plane(plane) => write!(f, "{}", plane),
            IObject::Sphere(sphere) => write!(f, "{}", sphere),
            IObject::Cylinder(cylinder) => write!(f, "{}", cylinder),
            IObject::Cone(cone) => write!(f, "{}", cone),
            IObject::Triangle(triangle) => write!(f, "{}", triangle),
        }
    }
}
