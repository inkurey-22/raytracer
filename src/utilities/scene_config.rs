use std::fmt;

use super::{Camera, Light, Sphere};

#[derive(Debug, Clone)]
pub struct SceneConfig {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub spheres: Vec<Sphere>,
}

impl fmt::Display for SceneConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Scene")?;
        writeln!(f, "{}", self.camera)?;

        if self.lights.is_empty() {
            writeln!(f, "Lights: none")?;
        } else {
            writeln!(f, "Lights: {}", self.lights.len())?;
            for (index, light) in self.lights.iter().enumerate() {
                if index > 0 {
                    writeln!(f)?;
                }
                writeln!(f, "  #{}", index)?;
                writeln!(f, "    position: {}", light.position)?;
                writeln!(f, "    color: {}", light.color)?;
                write!(f, "    intensity: {:.3}", light.intensity)?;
            }
        }

        writeln!(f)?;
        if self.spheres.is_empty() {
            writeln!(f, "Spheres: none")?;
        } else {
            writeln!(f, "Spheres: {}", self.spheres.len())?;
            for (index, sphere) in self.spheres.iter().enumerate() {
                writeln!(f, "  #{}", index)?;
                writeln!(f, "    center: {}", sphere.center)?;
                write!(f, "    radius: {:.3}", sphere.radius)?;

                if index + 1 < self.spheres.len() {
                    writeln!(f)?;
                }
            }
        }

        Ok(())
    }
}
