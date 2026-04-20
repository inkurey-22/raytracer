use std::fmt;

use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub fov: f64,
    pub position: Vec3,
    pub direction: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            fov: 60.0,
            position: Vec3 {
                x: 0.0,
                y: 1.0,
                z: -5.0,
            },
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        }
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Camera")?;
        writeln!(f, "  fov: {:.3}", self.fov)?;
        writeln!(f, "  position: {}", self.position)?;
        write!(f, "  direction: {}", self.direction)
    }
}
