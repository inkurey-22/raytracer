use omni_light::OmniLight;

use color::Color;
use vec3::Vec3;

#[derive(Debug, Clone)]
pub enum ILight {
    OmniLight(OmniLight),
}

impl ILight {
    pub fn compute_contribution(
        &self,
        hit_point: Vec3,
        normal: Vec3,
        objects: &[object_interface::IObject],
    ) -> Color {
        match self {
            ILight::OmniLight(omni_light) => {
                omni_light.compute_contribution(hit_point, normal, objects)
            }
        }
    }
}

impl std::fmt::Display for ILight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ILight::OmniLight(omni_light) => write!(f, "{}", omni_light),
        }
    }
}
