use std::f64;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use color::Color;
use vec3::Vec3;

use camera::Camera;
use ray::{EPSILON, Ray};

const MAX_RECURSION: i32 = 22;
const DEFAULT_SAMPLES_PER_PIXEL: usize = 16;
const DEFAULT_VARIANCE_THRESHOLD: f64 = 0.01;
const TILE_SIZE: usize = 8;

#[derive(Clone)]
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
    tiles: Arc<Vec<Mutex<Option<TileBuffer>>>>,
    width: usize,
    height: usize,
    max_depth: usize,
    variance_threshold: f64,
}

struct TileBuffer {
    start_row: usize,
    start_col: usize,
    data: Vec<Vec<Color>>,
}

impl RenderResources {
    fn new(
        camera: &Camera,
        width: usize,
        height: usize,
        lights: &[light_interface::ILight],
        objects: &[object_interface::IObject],
        sampling_config: SamplingConfig,
        total_tiles: usize,
    ) -> Self {
        Self {
            camera: Arc::new(*camera),
            lights: Arc::new(lights.to_vec()),
            objects: Arc::new(objects.to_vec()),
            tiles: {
                let mut tiles: Vec<Mutex<Option<TileBuffer>>> = Vec::with_capacity(total_tiles);
                for _ in 0..total_tiles {
                    tiles.push(Mutex::new(None));
                }
                Arc::new(tiles)
            },
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
    view_dir: Vec3,
    hit_object: &object_interface::IObject,
    lights: &[light_interface::ILight],
    objects: &[object_interface::IObject],
) -> Color {
    let mut lighting = Color::new(0.0, 0.0, 0.0);

    for light in lights {
        lighting += light.compute_contribution(
            hit_point,
            normal,
            view_dir,
            hit_object.get_color(),
            hit_object.get_reflectiveness(),
            objects,
        );
    }

    lighting.normalize_max()
}

fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
    direction - normal * (2.0 * direction.dot(&normal))
}

fn refract(direction: Vec3, normal: Vec3, eta_i: f64, eta_t: f64) -> Option<Vec3> {
    let dir = direction.normalize();
    let n = normal.normalize();
    let eta = eta_i / eta_t;
    let cos_i = (-dir).dot(&n).clamp(-1.0, 1.0);
    let sin_t2 = eta * eta * (1.0 - cos_i * cos_i);

    if sin_t2 > 1.0 {
        return None;
    }

    let cos_t = (1.0 - sin_t2).sqrt();
    Some((dir * eta + n * (eta * cos_i - cos_t)).normalize())
}

fn schlick(cosine: f64, eta_i: f64, eta_t: f64) -> f64 {
    let mut r0 = (eta_i - eta_t) / (eta_i + eta_t);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine.clamp(0.0, 1.0)).powi(5)
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
            let view_dir = -ray.direction;
            let local_color = compute_lighting(
                hit.point,
                hit.normal,
                view_dir,
                &hit.object,
                lights,
                objects,
            );
            let reflectiveness = hit.object.get_reflectiveness().clamp(0.0, 1.0);
            let transparency = hit.object.get_transparency().clamp(0.0, 1.0);

            if reflectiveness <= 0.0 && transparency <= 0.0 {
                return local_color.normalize_max();
            }

            let mut shading_normal = hit.normal.normalize();
            let surface_normal = shading_normal;
            let mut eta_i = 1.0;
            let mut eta_t = hit.object.get_refractive_index().max(EPSILON);
            let entering = ray.direction.dot(&shading_normal) < 0.0;

            if !entering {
                shading_normal = -shading_normal;
                eta_i = hit.object.get_refractive_index().max(EPSILON);
                eta_t = 1.0;
            }

            let reflected_direction = reflect(ray.direction, shading_normal).normalize();
            let reflected_ray = Ray::new(hit.point + shading_normal * EPSILON, reflected_direction);
            let reflected_color = trace_ray(&reflected_ray, lights, objects, depth + 1);

            let fresnel = if transparency > 0.0 {
                schlick((-ray.direction).dot(&shading_normal).abs(), eta_i, eta_t)
            } else {
                0.0
            };

            let reflection_weight = (reflectiveness + fresnel * transparency).max(0.0);
            let transmission_weight = ((1.0 - fresnel) * transparency).max(0.0);
            let local_weight = (1.0 - reflectiveness - transparency).max(0.0);

            let refracted_color = if transmission_weight > 0.0 {
                refract(ray.direction, shading_normal, eta_i, eta_t)
                    .map(|direction| {
                        let bias = if entering {
                            -surface_normal * EPSILON
                        } else {
                            surface_normal * EPSILON
                        };
                        let refracted_ray = Ray::new(hit.point + bias, direction);
                        trace_ray(&refracted_ray, lights, objects, depth + 1)
                    })
                    .unwrap_or_else(|| reflected_color)
            } else {
                Color::new(0.0, 0.0, 0.0)
            };

            let total_weight = local_weight + reflection_weight + transmission_weight;
            let scale = if total_weight > 1.0 {
                1.0 / total_weight
            } else {
                1.0
            };

            (local_color * (local_weight * scale)
                + reflected_color * (reflection_weight * scale)
                + refracted_color * (transmission_weight * scale))
                .normalize_max()
        }
        None => {
            let t = 0.5 * (ray.direction.z + 1.0);
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

    let forward = camera.direction.normalize();
    let world_up = Vec3::new(0.0, 0.0, 1.0);
    let world_right = Vec3::new(0.0, 1.0, 0.0);

    let right = if world_up.cross(&forward).length() > EPSILON {
        world_up.cross(&forward).normalize()
    } else {
        world_right.cross(&forward).normalize()
    };
    let up = forward.cross(&right).normalize();

    let ndc_x = (x + 0.5) / width;
    let ndc_y = (y + 0.5) / height;

    let px = (ndc_x - 0.5) * width_at_distance;
    let py = (0.5 - ndc_y) * height_at_distance;

    let direction = forward + right * px + up * py;

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

fn spawn_render_thread(
    next_tile: Arc<AtomicUsize>,
    tiles_x: usize,
    total_tiles: usize,
    start_row: usize,
    end_row: usize,
    start_col: usize,
    end_col: usize,
    resources: RenderResources,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let sampling_context = resources.sampling_context();

        loop {
            let tile_index = next_tile.fetch_add(1, Ordering::Relaxed);
            if tile_index >= total_tiles {
                break;
            }
            let tile_x = tile_index % tiles_x;
            let tile_y = tile_index / tiles_x;

            let tile_row_start = start_row + tile_y * TILE_SIZE;
            let tile_col_start = start_col + tile_x * TILE_SIZE;
            let tile_row_end = (tile_row_start + TILE_SIZE).min(end_row);
            let tile_col_end = (tile_col_start + TILE_SIZE).min(end_col);
            let tile_height = tile_row_end - tile_row_start;
            let tile_width = tile_col_end - tile_col_start;
            let mut tile = vec![vec![Color::default(); tile_width]; tile_height];

            for (dy, y) in (tile_row_start..tile_row_end).enumerate() {
                for (dx, x) in (tile_col_start..tile_col_end).enumerate() {
                    let samples = adaptive_sample(&sampling_context, x as f64, y as f64, 0);
                    tile[dy][dx] = average_color(&samples);
                }
            }

            let mut slot = resources.tiles[tile_index].lock().unwrap();
            *slot = Some(TileBuffer {
                start_row: tile_row_start,
                start_col: tile_col_start,
                data: tile,
            });
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn render_rows_multithreaded(
    camera: &Camera,
    width: usize,
    height: usize,
    lights: &[light_interface::ILight],
    objects: &[object_interface::IObject],
    sampling_config: SamplingConfig,
    start_row: usize,
    end_row: usize,
) -> Vec<Vec<Color>> {
    let total_rows = end_row
        .saturating_sub(start_row)
        .min(height.saturating_sub(start_row));
    if total_rows == 0 {
        return Vec::new();
    }

    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(total_rows)
        .max(1);

    let total_cols = width;
    let tile_rows = total_rows.div_ceil(TILE_SIZE);
    let tile_cols = total_cols.div_ceil(TILE_SIZE);
    let total_tiles = tile_rows.saturating_mul(tile_cols).max(1);
    let max_threads = max_threads.min(total_tiles).max(1);
    let next_tile = Arc::new(AtomicUsize::new(0));
    let resources = RenderResources::new(camera, width, height, lights, objects, sampling_config, total_tiles);
    eprintln!(
        "Rendering rows {}..{} using {} worker threads (tile={}x{}, tiles={})",
        start_row, end_row, max_threads, TILE_SIZE, TILE_SIZE, total_tiles
    );
    let mut handles = Vec::new();

    for _ in 0..max_threads {
        handles.push(spawn_render_thread(
            next_tile.clone(),
            tile_cols,
            total_tiles,
            start_row,
            end_row,
            0,
            total_cols,
            resources.clone(),
        ));
    }

    for h in handles {
        h.join().unwrap();
    }

    let mut result: Vec<Vec<Color>> = Vec::with_capacity(total_rows);
    result.resize_with(total_rows, || vec![Color::default(); width]);

    for tile_slot in resources.tiles.iter() {
        let mut slot = tile_slot.lock().unwrap();
        let tile = slot.take().expect("render tile missing");
        for (dy, row) in tile.data.into_iter().enumerate() {
            let target_y = tile.start_row - start_row + dy;
            let target_row = &mut result[target_y];
            for (dx, pixel) in row.into_iter().enumerate() {
                target_row[tile.start_col + dx] = pixel;
            }
        }
    }

    result
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

#[cfg(test)]
mod tests {
    use super::*;

    fn z_up_camera() -> Camera {
        Camera {
            fov: 60.0,
            position: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    #[test]
    fn center_ray_matches_depth_axis() {
        let camera = z_up_camera();
        let width = 200.0;
        let height = 100.0;
        let center = generate_ray(
            &camera,
            width / 2.0 - 0.5,
            height / 2.0 - 0.5,
            width,
            height,
        );

        assert!((center.direction.x - 1.0).abs() < 1e-12);
        assert!(center.direction.y.abs() < 1e-12);
        assert!(center.direction.z.abs() < 1e-12);
    }

    #[test]
    fn right_pixels_map_to_y_axis() {
        let camera = z_up_camera();
        let width = 100.0;
        let height = 100.0;
        let mid_y = height / 2.0 - 0.5;

        let left_ray = generate_ray(&camera, 0.0, mid_y, width, height);
        let right_ray = generate_ray(&camera, width - 1.0, mid_y, width, height);

        assert!(left_ray.direction.y < 0.0);
        assert!(right_ray.direction.y > 0.0);
    }

    #[test]
    fn vertical_pixels_map_to_z_axis() {
        let camera = z_up_camera();
        let width = 100.0;
        let height = 100.0;
        let mid_x = width / 2.0 - 0.5;

        let top_ray = generate_ray(&camera, mid_x, 0.0, width, height);
        let bottom_ray = generate_ray(&camera, mid_x, height - 1.0, width, height);

        assert!(top_ray.direction.z > 0.0);
        assert!(bottom_ray.direction.z < 0.0);
    }

    #[test]
    fn reflect_flips_toward_surface_normal() {
        let direction = Vec3::new(1.0, -1.0, 0.0).normalize();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let reflected = reflect(direction, normal);

        assert!((reflected.x - direction.x).abs() < 1e-12);
        assert!((reflected.y + direction.y).abs() < 1e-12);
        assert!(reflected.z.abs() < 1e-12);
    }

    #[test]
    fn refract_bends_toward_the_normal_when_entering() {
        let direction = Vec3::new(1.0, -1.0, 0.0).normalize();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let refracted = refract(direction, normal, 1.0, 1.5).unwrap();

        assert!(refracted.x.abs() < direction.x.abs());
        assert!(refracted.y < 0.0);
        assert!(refracted.z.abs() < 1e-12);
    }

    #[test]
    fn refract_returns_none_for_total_internal_reflection() {
        let direction = Vec3::new(0.9, -0.1, 0.0).normalize();
        let normal = Vec3::new(0.0, 1.0, 0.0);

        assert!(refract(direction, normal, 1.5, 1.0).is_none());
    }
}
