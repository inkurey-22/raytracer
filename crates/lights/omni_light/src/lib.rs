use std::fmt;

use color::Color;
use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct OmniLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f64,
}

impl Default for OmniLight {
    fn default() -> Self {
        OmniLight {
            position: Vec3::default(),
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            intensity: 1.0,
        }
    }
}

impl fmt::Display for OmniLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "OmniLight")?;
        writeln!(f, "  position: {}", self.position)?;
        writeln!(f, "  color: {}", self.color)?;
        write!(f, "  intensity: {:.3}", self.intensity)
    }
}
