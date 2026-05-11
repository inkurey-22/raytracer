use std::fmt;

use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct AmbiantLight {
    pub color: Color,
    pub intensity: f64,
}

impl Default for AmbiantLight {
    fn default() -> Self {
        AmbiantLight {
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            intensity: 0.2,
        }
    }
}

impl fmt::Display for AmbiantLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AmbiantLight")?;
        writeln!(f, "      color: {}", self.color)?;
        writeln!(f, "      intensity: {:.3}", self.intensity)
    }
}
