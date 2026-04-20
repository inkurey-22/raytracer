use std::fmt;

use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sphere")?;
        writeln!(f, "  center: {}", self.center)?;
        write!(f, "  radius: {:.3}", self.radius)
    }
}
