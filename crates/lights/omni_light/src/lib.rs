use std::fmt;

use color::Color;
use ray::{EPSILON, Ray};
use vec3::Vec3;

use object_interface::IObject;

#[derive(Debug, Clone, Copy)]
pub struct OmniLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f64,
}

fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
    direction - normal * (2.0 * direction.dot(&normal))
}

fn is_obstructed(ray: &Ray, objects: &[object_interface::IObject], max_distance: f64) -> bool {
    for object in objects {
        if let Some(hit) = object.intersect(ray, EPSILON) {
            if hit.t < max_distance {
                return true;
            }
        }
    }
    false
}

impl OmniLight {
    pub fn compute_contribution(
        &self,
        hit_point: Vec3,
        normal: Vec3,
        view_dir: Vec3,
        surface_color: Color,
        reflectiveness: f64,
        objects: &[IObject],
    ) -> Color {
        let light_dir = (self.position - hit_point).normalize();
        let distance = (self.position - hit_point).length();

        let shadow_ray: Ray = Ray::new(hit_point + normal * EPSILON, light_dir);
        let in_shadow = is_obstructed(&shadow_ray, objects, distance);

        if in_shadow {
            return Color::new(0.0, 0.0, 0.0);
        }

        let diffuse_intensity = normal.dot(&light_dir).max(0.0);
        let attenuation = self.intensity / (distance * distance);
        let diffuse = self.color * attenuation * diffuse_intensity * surface_color;

        let reflected_light = reflect(-light_dir, normal).normalize();
        let specular_intensity = reflected_light
            .dot(&view_dir.normalize())
            .max(0.0)
            .powf(32.0)
            * reflectiveness;
        let specular = self.color * attenuation * specular_intensity;

        diffuse + specular
    }
}

impl fmt::Display for OmniLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "OmniLight")?;
        writeln!(f, "      position: {}", self.position)?;
        writeln!(f, "      color: {}", self.color)?;
        writeln!(f, "      intensity: {:.3}", self.intensity)
    }
}
