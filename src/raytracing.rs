use std::f64;
use std::sync::{Arc, Mutex};
use std::thread;

use color::Color;
use vec3::Vec3;

use camera::Camera;
use ray::{EPSILON, Ray};

const MAX_RECURSION: i32 = 22;
const DEFAULT_SAMPLES_PER_PIXEL: usize = 16;
const DEFAULT_VARIANCE_THRESHOLD: f64 = 0.01;

pub struct SamplingConfig {
    pub samples_per_pixel: usize,
    pub variance_threshold: f64,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        SamplingConfig {
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            variance_threshold: DEFAULT_VARIANCE_THRESHOLD,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HitInfo {
    pub _depth: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub object: object_interface::IObject,
}

struct AdaptiveSamplingContext<'a> {
    camera: &'a Camera,
    width: f64,
    height: f64,
    lights: &'a [light_interface::ILight],
    objects: &'a [object_interface::IObject],
    max_depth: usize,
    variance_threshold: f64,
}

#[derive(Clone)]
struct RenderResources {
    camera: Arc<Camera>,
    lights: Arc<Vec<light_interface::ILight>>,
    objects: Arc<Vec<object_interface::IObject>>,
    image: Arc<Mutex<Vec<Vec<Color>>>>,
    width: usize,
    height: usize,
    max_depth: usize,
    variance_threshold: f64,
}

impl RenderResources {
    fn new(
        camera: &Camera,
        width: usize,
        height: usize,
        lights: &[light_interface::ILight],
        objects: &[object_interface::IObject],
        sampling_config: SamplingConfig,
    ) -> Self {
        Self {
            camera: Arc::new(*camera),
            lights: Arc::new(lights.to_vec()),
            objects: Arc::new(objects.to_vec()),
            image: Arc::new(Mutex::new(vec![vec![Color::default(); width]; height])),
            width,
            height,
            max_depth: (sampling_config.samples_per_pixel as f64).log2().ceil() as usize,
            variance_threshold: sampling_config.variance_threshold,
        }
    }

    fn sampling_context(&self) -> AdaptiveSamplingContext<'_> {
        AdaptiveSamplingContext {
            camera: self.camera.as_ref(),
            width: self.width as f64,
            height: self.height as f64,
            lights: self.lights.as_slice(),
            objects: self.objects.as_slice(),
            max_depth: self.max_depth,
            variance_threshold: self.variance_threshold,
        }
    }
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
        Some(hit) => {
            compute_lighting(hit.point, hit.normal, hit.object, lights, objects).normalize_max()
        }
        None => {
            let t = 0.5 * (ray.direction.x + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn color_variance(colors: &[Color]) -> f64 {
    if colors.len() < 2 {
        return 0.0;
    }

    let mean_r = colors.iter().map(|c| c.r).sum::<f64>() / colors.len() as f64;
    let mean_g = colors.iter().map(|c| c.g).sum::<f64>() / colors.len() as f64;
    let mean_b = colors.iter().map(|c| c.b).sum::<f64>() / colors.len() as f64;

    let var_r = colors.iter().map(|c| (c.r - mean_r).powi(2)).sum::<f64>() / colors.len() as f64;
    let var_g = colors.iter().map(|c| (c.g - mean_g).powi(2)).sum::<f64>() / colors.len() as f64;
    let var_b = colors.iter().map(|c| (c.b - mean_b).powi(2)).sum::<f64>() / colors.len() as f64;

    var_r + var_g + var_b
}

fn sample_pixel_corners(context: &AdaptiveSamplingContext, x: f64, y: f64) -> Vec<Color> {
    const SAMPLE_OFFSETS: [(f64, f64); 4] =
        [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)];
    let mut samples = Vec::with_capacity(SAMPLE_OFFSETS.len());

    for (dx, dy) in SAMPLE_OFFSETS {
        let ray = generate_ray(
            context.camera,
            x + dx,
            y + dy,
            context.width,
            context.height,
        );
        samples.push(trace_ray(&ray, context.lights, context.objects, 0));
    }

    samples
}

fn refine_samples(context: &AdaptiveSamplingContext, x: f64, y: f64, depth: usize) -> Vec<Color> {
    const QUADRANTS: [(f64, f64); 4] = [(0.0, 0.0), (0.5, 0.0), (0.0, 0.5), (0.5, 0.5)];
    let mut refined_samples = Vec::new();

    for (qx, qy) in QUADRANTS {
        refined_samples.extend(adaptive_sample(context, x + qx, y + qy, depth + 1));
    }

    refined_samples
}

fn adaptive_sample(context: &AdaptiveSamplingContext, x: f64, y: f64, depth: usize) -> Vec<Color> {
    let samples = sample_pixel_corners(context, x, y);

    if depth < context.max_depth && color_variance(&samples) > context.variance_threshold {
        refine_samples(context, x, y, depth)
    } else {
        samples
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

fn average_color(samples: &[Color]) -> Color {
    if samples.is_empty() {
        return Color::default();
    }

    let sum = samples.iter().fold(Color::new(0.0, 0.0, 0.0), |acc, c| {
        Color::new(acc.r + c.r, acc.g + c.g, acc.b + c.b)
    });
    Color::new(
        sum.r / samples.len() as f64,
        sum.g / samples.len() as f64,
        sum.b / samples.len() as f64,
    )
}

fn render_row(width: usize, y: usize, sampling_context: &AdaptiveSamplingContext) -> Vec<Color> {
    let mut row = vec![Color::default(); width];
    for (x, pixel) in row.iter_mut().enumerate() {
        let samples = adaptive_sample(sampling_context, x as f64, y as f64, 0);
        *pixel = average_color(&samples);
    }
    row
}

fn spawn_render_thread(
    start_row: usize,
    end_row: usize,
    resources: RenderResources,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let sampling_context = resources.sampling_context();

        for y in start_row..end_row {
            let row = render_row(resources.width, y, &sampling_context);
            let mut img = resources.image.lock().unwrap();
            img[y] = row;
        }
    })
}

pub fn render(
    camera: &Camera,
    width: usize,
    height: usize,
    lights: &[light_interface::ILight],
    objects: &[object_interface::IObject],
    sampling_config: SamplingConfig,
) -> Vec<Vec<Color>> {
    let resources = RenderResources::new(camera, width, height, lights, objects, sampling_config);

    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(height);
    let mut handles = Vec::new();

    let rows_per_thread = height.div_ceil(max_threads);
    for thread_id in 0..max_threads {
        let start_row = thread_id * rows_per_thread;
        let end_row = ((thread_id + 1) * rows_per_thread).min(height);
<<<<<<< feat/cone

        let handle = thread::spawn(move || {
            for y in start_row..end_row {
                let mut row = vec![Color::default(); width];
                for (x, pixel) in row.iter_mut().enumerate() {
                    let ray =
                        generate_ray(&camera, x as f64, y as f64, width as f64, height as f64);
                    *pixel = trace_ray(&ray, &lights, &objects, 0);
                }
                let mut img = image.lock().unwrap();
                img[y] = row;
            }
        });
        handles.push(handle);
=======
        handles.push(spawn_render_thread(start_row, end_row, resources.clone()));
>>>>>>> main
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(resources.image)
        .unwrap()
        .into_inner()
        .unwrap()
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
