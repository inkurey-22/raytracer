use std::fmt;

use crate::utilities::Camera;

#[derive(Debug, Clone)]
pub struct SceneConfig {
    pub camera: Camera,
    pub lights: Vec<light_interface::ILight>,
    pub objects: Vec<object_interface::IObject>,
    pub width: usize,
    pub height: usize,
}

impl fmt::Display for SceneConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Scene:")?;
        writeln!(f, "  width: {}, height: {}", self.width, self.height)?;
        writeln!(f, "{}", self.camera)?;

        if self.lights.is_empty() {
            writeln!(f, "  Lights: none")?;
        } else {
            writeln!(f, "  Lights: {}", self.lights.len())?;
            for (index, light) in self.lights.iter().enumerate() {
                write!(f, "    #{} {}", index + 1, light)?;
            }
        }

        /*writeln!(f)?;
        if let Some(light_interface::ILight::AmbiantLight(ambiant_light)) = self
            .lights
            .iter()
            .find(|light| matches!(light, light_interface::ILight::AmbiantLight(_)))
        {
            writeln!(f, "  Ambiant Light:")?;
            writeln!(f, "    color: {}", ambiant_light.color)?;
            writeln!(f, "    intensity: {:.3}", ambiant_light.intensity)?;
        } else {
            writeln!(f, "  Ambiant Light: none")?;
        }*/

        if self.objects.is_empty() {
            writeln!(f, "  Objects: none")?;
        } else {
            writeln!(f, "  Objects: {}", self.objects.len())?;
            for (index, object) in self.objects.iter().enumerate() {
                write!(f, "    #{} {}", index + 1, object)?;
            }
        }

        Ok(())
    }
}
