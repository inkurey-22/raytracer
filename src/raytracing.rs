use std::f64;
use std::sync::{Arc, Mutex};
use std::thread;

use color::Color;
use vec3::Vec3;

use camera::Camera;
use ray::{EPSILON, Ray};

const MAX_RECURSION: i32 = 22;

#[derive(Debug, Clone)]
pub struct HitInfo {
    pub _depth: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub object: object_interface::IObject,
}

pub fn find_closest_hit(ray: &Ray, objects: &[object_interface::IObject]) -> Option<HitInfo> {
    let mut closest_t = f64::INFINITY;
    let mut hit_info: Option<HitInfo> = None;

    for object in objects {
        if let Some(hit) = object.intersect(ray, EPSILON)
            && hit.t < closest_t
        {
            closest_t = hit.t;
            hit_info = Some(HitInfo {
                _depth: hit.t,
                point: hit.point,
                normal: hit.normal,
                object: object.clone(),
            });
        }
    }

    hit_info
}

pub fn compute_lighting(
    hit_point: Vec3,
    normal: Vec3,
    hit_object: object_interface::IObject,
    lights: &[light_interface::ILight],
    objects: &[object_interface::IObject],
) -> Color {
    let mut lighting = Color::new(0.0, 0.0, 0.0);

    for light in lights {
        lighting += light.compute_contribution(hit_point, normal, objects);
    }

    (lighting * hit_object.get_color()).normalize_max()
}

pub fn trace_ray(
    ray: &Ray,
    lights: &[light_interface::ILight],
    objects: &[object_interface::IObject],
    depth: i32,
) -> Color {
    if depth > MAX_RECURSION {
        return Color::new(0.0, 0.0, 0.0);
    }

    match find_closest_hit(ray, objects) {
        Some(hit) => compute_lighting(hit.point, hit.normal, hit.object, lights, objects).normalize_max(),
        None => {
            let t = 0.5 * (ray.direction.x + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
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
    width: usize,
    height: usize,
    lights: &[light_interface::ILight],
    objects: &[object_interface::IObject],
) -> Vec<Vec<Color>> {
    let shared_camera = Arc::new(*camera);
    let shared_lights = Arc::new(lights.to_vec());
    let shared_objects = Arc::new(objects.to_vec());

    let image = Arc::new(Mutex::new(vec![vec![Color::default(); width]; height]));

    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(height);
    let mut handles = Vec::new();

    let rows_per_thread = height.div_ceil(max_threads);
    for thread_id in 0..max_threads {
        let camera = Arc::clone(&shared_camera);
        let lights = Arc::clone(&shared_lights);
        let objects = Arc::clone(&shared_objects);
        let image = Arc::clone(&image);

        let start_row = thread_id * rows_per_thread;
        let end_row = ((thread_id + 1) * rows_per_thread).min(height);

        let handle = thread::spawn(move || {
            for y in start_row..end_row {
                let mut row = vec![Color::default(); width];
                for (x, pixel) in row.iter_mut().enumerate() {
                    let ray = generate_ray(&camera, x as f64, y as f64, width as f64, height as f64);
                    *pixel = trace_ray(&ray, &lights, &objects, 0);
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

pub fn write_ppm(filename: &str, image: &[Vec<Color>]) -> std::io::Result<()> {
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
            let clamped = pixel.saturate();
            let r = (clamped.r * 255.0) as u8;
            let g = (clamped.g * 255.0) as u8;
            let b = (clamped.b * 255.0) as u8;
            buffer.push_str(&format!("{} {} {} ", r, g, b));
        }
        buffer.push('\n');
    }

    let mut file = File::create(filename)?;
    file.write_all(buffer.as_bytes())?;
    Ok(())
}
