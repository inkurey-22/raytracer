use plane::Plane;
use sphere::Sphere;

#[derive(Debug, Clone)]
pub enum Object {
    Plane(Plane),
    Sphere(Sphere),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Plane(plane) => write!(f, "{}", plane),
            Object::Sphere(sphere) => write!(f, "{}", sphere),
        }
    }
}
