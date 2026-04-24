use std::f64;
use std::sync::{Arc, Mutex};
use std::thread;

use vec3::Vec3;

use camera::Camera;
use omni_light::OmniLight;
use plane::Plane;
use ray::{EPSILON, Ray};
use sphere::Sphere;

const MAX_RECURSION: i32 = 22;

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub object_type: ObjectType,
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Sphere,
    Plane,
}

pub fn find_closest_hit(ray: &Ray, spheres: &[Sphere], planes: &[Plane]) -> Option<HitInfo> {
    let mut closest_t = f64::INFINITY;
    let mut hit_info: Option<HitInfo> = None;

    for sphere in spheres {
        if let Some(hit) = sphere.intersect(ray, EPSILON)
            && hit.t < closest_t
        {
            closest_t = hit.t;
            hit_info = Some(HitInfo {
                t: hit.t,
                point: hit.point,
                normal: hit.normal,
                object_type: ObjectType::Sphere,
            });
        }
    }

    for plane in planes {
        if let Some(hit) = plane.intersect(ray, EPSILON)
            && hit.t < closest_t
        {
            closest_t = hit.t;
            hit_info = Some(HitInfo {
                t: hit.t,
                point: hit.point,
                normal: hit.normal,
                object_type: ObjectType::Plane,
            });
        }
    }

    hit_info
}

pub fn compute_lighting(
    hit_point: Vec3,
    normal: Vec3,
    omni_lights: &[OmniLight],
    spheres: &[Sphere],
    planes: &[Plane],
) -> Vec3 {
    let mut color = Vec3::new(0.0, 0.0, 0.0);

    for light in omni_lights {
        let light_dir = (light.position - hit_point).normalize();
        let distance = (light.position - hit_point).length();

        let shadow_ray: Ray = Ray::new(hit_point + normal * EPSILON, light_dir);
        let in_shadow =
            find_closest_hit(&shadow_ray, spheres, planes).is_some_and(|hit| hit.t < distance);

        if in_shadow {
            continue;
        }

        let diffuse_intensity = normal.dot(&light_dir).max(0.0);
        let diffuse = light.color * (light.intensity / (distance * distance)) * diffuse_intensity;

        color += diffuse;
    }

    color
}

pub fn trace_ray(
    ray: &Ray,
    omni_lights: &[OmniLight],
    spheres: &[Sphere],
    planes: &[Plane],
    depth: i32,
) -> Vec3 {
    if depth > MAX_RECURSION {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match find_closest_hit(ray, spheres, planes) {
        Some(hit) => {
            let lighting = compute_lighting(hit.point, hit.normal, omni_lights, spheres, planes);

            let object_color = match hit.object_type {
                ObjectType::Sphere => Vec3::new(0.9, 0.9, 0.9),
                ObjectType::Plane => Vec3::new(1.0, 1.0, 1.0),
            };

            object_color * lighting.normalize_max()
        }
        None => {
            let t = 0.5 * (ray.direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

pub fn generate_ray(camera: &Camera, x: f64, y: f64, width: f64, height: f64) -> Ray {
    let aspect = width / height;
    let fov_rad = camera.fov * std::f64::consts::PI / 180.0;
    let height_at_distance = 2.0 * (fov_rad / 2.0).tan();
    let width_at_distance = height_at_distance * aspect;

    let right = camera
        .direction
        .cross(&Vec3::new(0.0, 1.0, 0.0))
        .normalize();
    let up = right.cross(&camera.direction).normalize();

    let ndc_x = (x + 0.5) / width;
    let ndc_y = (y + 0.5) / height;

    let px = (ndc_x - 0.5) * width_at_distance;
    let py = (0.5 - ndc_y) * height_at_distance;

    let direction = camera.direction + right * px + up * py;

    Ray::new(camera.position, direction)
}

pub fn render(
    camera: &Camera,
    omni_lights: &[OmniLight],
    spheres: &[Sphere],
    planes: &[Plane],
    width: usize,
    height: usize,
) -> Vec<Vec<Vec3>> {
    let shared_camera = Arc::new(*camera);
    let shared_lights = Arc::new(lights.to_vec());
    let shared_spheres = Arc::new(spheres.to_vec());
    let shared_planes = Arc::new(planes.to_vec());

    let image = Arc::new(Mutex::new(vec![vec![Vec3::default(); width]; height]));

    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(height);
    let mut handles = Vec::new();

    let rows_per_thread = height.div_ceil(max_threads);
    for thread_id in 0..max_threads {
        let camera = Arc::clone(&shared_camera);
        let lights = Arc::clone(&shared_lights);
        let spheres = Arc::clone(&shared_spheres);
        let planes = Arc::clone(&shared_planes);
        let image = Arc::clone(&image);

        let start_row = thread_id * rows_per_thread;
        let end_row = ((thread_id + 1) * rows_per_thread).min(height);

        let handle = thread::spawn(move || {
            for y in start_row..end_row {
                let mut row = vec![Vec3::default(); width];
                for (x, pixel) in row.iter_mut().enumerate() {
                    let ray =
                        generate_ray(&camera, x as f64, y as f64, width as f64, height as f64);
                    *pixel = trace_ray(&ray, &lights, &spheres, &planes, 0);
                }
                let mut img = image.lock().unwrap();
                img[y] = row;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(image).unwrap().into_inner().unwrap()
}

pub fn write_ppm(filename: &str, image: &[Vec<Vec3>]) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let height = image.len();
    let width = if height > 0 { image[0].len() } else { 0 };

    let mut buffer = String::new();
    buffer.push_str("P3\n");
    buffer.push_str(&format!("{} {}\n", width, height));
    buffer.push_str("255\n");

    for row in image {
        for pixel in row {
            let r = (pixel.x * 255.0) as u8;
            let g = (pixel.y * 255.0) as u8;
            let b = (pixel.z * 255.0) as u8;
            buffer.push_str(&format!("{} {} {} ", r, g, b));
        }
        buffer.push('\n');
    }

    let mut file = File::create(filename)?;
    file.write_all(buffer.as_bytes())?;
    Ok(())
}
