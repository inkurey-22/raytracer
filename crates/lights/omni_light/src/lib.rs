use std::fmt;

use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct OmniLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f64,
}

impl Default for OmniLight {
    fn default() -> Self {
        OmniLight {
            position: Vec3::default(),
            color: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
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
