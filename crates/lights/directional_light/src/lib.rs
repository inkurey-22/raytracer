use std::fmt;

use color::Color;
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f64,
}

impl Default for DirectionalLight {
    fn default() -> Self {
        DirectionalLight {
            direction: Vec3::new(0.0, -1.0, 0.0),
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            intensity: 1.0,
        }
    }
}

impl fmt::Display for DirectionalLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DirectionalLight")?;
        writeln!(f, "  direction: {}", self.direction)?;
        writeln!(f, "  color: {}", self.color)?;
        write!(f, "  intensity: {:.3}", self.intensity)
    }
}
