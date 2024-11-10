use std::fs::File;
use std::io::Write;
use std::ops::RangeInclusive;
use std::time::Instant;

use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use camera::Camera;
use color::Color;
use rand::Rng;
use rayon::prelude::*;
use sphere::Sphere;
use vec3::Point3;
use world::World;

mod color;
mod vec3;
mod ray;
mod hit_record;
mod sphere;
mod world;
mod camera;
mod material;

const IMAGE_WIDTH: u32 = 1600;
const IMAGE_HEIGHT: u32 = 900;
const VERTICAL_FOV: f64 = 60.0;

const SAMPLES_PER_PIXEL: u32 = 1024;
const MAX_DEPTH: u8 = 16;

const SAMPLE_OFFSET: f64 = 0.5;
const SAMPLE_OFFSET_RANGE: RangeInclusive<f64> = -SAMPLE_OFFSET..=SAMPLE_OFFSET;

const T_MIN: f64 = 0.001;
const T_MAX: f64 = f64::INFINITY;

#[allow(clippy::vec_init_then_push)]
fn main() {
	let material_blue = Lambertian::new(Color::new(0.1, 0.2, 0.5));
	let material_green = Lambertian::new(Color::new(0.8, 0.8, 0.0));
	let material_gray_metal = Metal::new(Color::new(0.8, 0.8, 0.8), 0.0);
	// let material_red_metal = Metal::new(Color::new(0.8, 0.6, 0.2), 0.3);
	let material_glass = Dielectric::new(1.5);

	let mut world = World::new();
	world.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, 1.0), 0.5, material_glass)));
	world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, 1.2), 0.5, material_blue)));
	world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, 1.0), 0.5, material_gray_metal)));
	world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, 1.0), 100.0, material_green)));
	
	let camera_position = Point3::new(0.0, 0.0, 0.0);
	let camera = Camera::new(camera_position, IMAGE_WIDTH, IMAGE_HEIGHT, VERTICAL_FOV);


	println!("Starting render...");
	let render_start = Instant::now();

	let pixel_data: Vec<Color> = (0..IMAGE_HEIGHT).into_par_iter().flat_map(|j| {
		(0..IMAGE_WIDTH).map(|i| {
			let ray = camera.get_ray_for_pixel(j, i);
			let mut color_acc = ray.cast(&world, T_MIN, T_MAX, MAX_DEPTH);
			let mut rng = rand::thread_rng();

			for _ in 1..SAMPLES_PER_PIXEL {
				let rnd_v_offset: f64 = rng.gen_range(SAMPLE_OFFSET_RANGE);
				let rnd_h_offset: f64 = rng.gen_range(SAMPLE_OFFSET_RANGE);

				let ray = camera.get_ray_for_pixel_with_offset(j, rnd_v_offset, i, rnd_h_offset);
				color_acc += ray.cast(&world, T_MIN, T_MAX, MAX_DEPTH);
			}
			
			color_acc / SAMPLES_PER_PIXEL as f64
		}).collect::<Vec<Color>>()
	}).collect();

	let render_duration = render_start.elapsed();
	println!("Render complete (render time={:?}), writing to file...", render_duration);

	let file_data = format!(
		"P3\n{} {}\n255\n{}\n",
		IMAGE_WIDTH,
		IMAGE_HEIGHT,
		pixel_data.iter()
			.map(|pixel_color| pixel_color.ppm_format_ln())
			.collect::<String>()
	);

	let mut file = File::create("image.ppm").unwrap();
	file.write_all(file_data.as_bytes()).unwrap();

	println!("Done!");
}
