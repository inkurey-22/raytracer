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
        if object.intersect(ray, EPSILON).is_some() {
            return true;
        }
    }
    false
}

impl DirectionalLight {
    pub fn compute_contribution(
        &self,
        hit_point: Vec3,
        normal: Vec3,
        objects: &[object_interface::IObject],
    ) -> Color {
        let light_dir = -self.direction.normalize();
        let shadow_ray = Ray::new(hit_point + normal * EPSILON, light_dir);
        if is_obstructed(&shadow_ray, objects) {
            return Color::new(0.0, 0.0, 0.0);
        }

        let diffuse_intensity = normal.dot(&light_dir).max(0.0);
        self.color * self.intensity * diffuse_intensity
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
