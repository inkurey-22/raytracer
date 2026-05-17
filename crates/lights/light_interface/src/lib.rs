use ambiant::AmbiantLight;
use directional_light::DirectionalLight;
use omni_light::OmniLight;

use color::Color;
use object_interface::ObjectQuery;
use vec3::Vec3;

#[derive(Debug, Clone)]
pub enum ILight {
    OmniLight(OmniLight),
    DirectionalLight(DirectionalLight),
    AmbiantLight(AmbiantLight),
}

impl ILight {
    pub fn compute_contribution(
        &self,
        hit_point: Vec3,
        normal: Vec3,
        view_dir: Vec3,
        surface_color: Color,
        reflectiveness: f64,
        objects: &dyn ObjectQuery,
    ) -> Color {
        match self {
            ILight::OmniLight(omni_light) => omni_light.compute_contribution(
                hit_point,
                normal,
                view_dir,
                surface_color,
                reflectiveness,
                objects,
            ),
            ILight::DirectionalLight(directional_light) => directional_light.compute_contribution(
                hit_point,
                normal,
                view_dir,
                surface_color,
                reflectiveness,
                objects,
            ),
            ILight::AmbiantLight(ambiant_light) => {
                ambiant_light.color * ambiant_light.intensity * surface_color
            }
        }
    }
}

impl std::fmt::Display for ILight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ILight::OmniLight(omni_light) => write!(f, "{}", omni_light),
            ILight::DirectionalLight(directional_light) => write!(f, "{}", directional_light),
            ILight::AmbiantLight(ambiant_light) => write!(f, "{}", ambiant_light),
        }
    }
}
