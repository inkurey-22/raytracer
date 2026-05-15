use std::fmt;

use color::Color;
use ray::{Ray, EPSILON};
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
            direction: Vec3::new(0.0, 0.0, -1.0),
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            intensity: 1.0,
        }
    }
}

fn is_obstructed(ray: &Ray, objects: &[object_interface::IObject]) -> bool {
    for object in objects {
        if object.get_transparency() > 0.0 {
            continue;
        }
        if object.intersect(ray, EPSILON).is_some() {
            return true;
        }
    }
    false
}

fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
    direction - normal * (2.0 * direction.dot(&normal))
}

impl DirectionalLight {
    pub fn compute_contribution(
        &self,
        hit_point: Vec3,
        normal: Vec3,
        view_dir: Vec3,
        surface_color: Color,
        reflectiveness: f64,
        objects: &[object_interface::IObject],
    ) -> Color {
        let light_dir = -self.direction.normalize();
        let shadow_ray = Ray::new(hit_point + normal * EPSILON, light_dir);
        if is_obstructed(&shadow_ray, objects) {
            return Color::new(0.0, 0.0, 0.0);
        }

        let diffuse_intensity = normal.dot(&light_dir).max(0.0);
        let diffuse = self.color * self.intensity * diffuse_intensity * surface_color;

        let reflected_light = reflect(-light_dir, normal).normalize();
        let specular_intensity = reflected_light
            .dot(&view_dir.normalize())
            .max(0.0)
            .powf(32.0)
            * reflectiveness;
        let specular = self.color * self.intensity * specular_intensity;

        diffuse + specular
    }
}

impl fmt::Display for DirectionalLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DirectionalLight")?;
        writeln!(f, "      direction: {}", self.direction)?;
        writeln!(f, "      color: {}", self.color)?;
        writeln!(f, "      intensity: {:.3}", self.intensity)
    }
}
