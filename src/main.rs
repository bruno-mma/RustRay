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

const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = 800;
const VERTICAL_FOV: f64 = 20.0;

const SAMPLES_PER_PIXEL: u32 = 512;
const MAX_DEPTH: u8 = 32;

const SAMPLE_OFFSET: f64 = 0.5;
const SAMPLE_OFFSET_RANGE: RangeInclusive<f64> = -SAMPLE_OFFSET..=SAMPLE_OFFSET;

const T_MIN: f64 = 0.001;
const T_MAX: f64 = f64::INFINITY;

#[allow(clippy::vec_init_then_push)]
fn gen_world() -> World {
	let mut world = World::new();

	let ground_material = Lambertian::new(Color::new(0.6, 0.6, 0.6));
	world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

	let mat_1 = Dielectric::new(1.5);
	world.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat_1)));

	let mat_2 = Lambertian::new(Color::new(0.2, 0.2, 0.8));
	world.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat_2)));

	let mat_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
	world.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat_3)));

	let mut rng =  rand::thread_rng();

	for a in -11..=11 {
		for b in -11..=11 {
			if  a < 5 && a > -5 && b < 2 && b > -2 {
				continue;
			}

			let center = Point3::new(
				a as f64 + 0.9 * rng.gen_range(0.0..=1.0),
				0.2,
				b as f64 + 0.9 * rng.gen_range(0.0..=1.0)
			);

			let mat_float = rng.gen_range(0.0..=1.0);
			match mat_float {
				x if x < 0.8 => {
					let albedo = Color::new_rand_range(0.0..=1.0);
					let mat = Lambertian::new(albedo);
					world.push(Box::new(Sphere::new(center, 0.2, mat)));
				},
				x if x < 0.95 => {
					let albedo = Color::new_rand_range(0.5..=1.0);
					let fuzz = rng.gen_range(0.0..=0.1);
					let mat = Metal::new(albedo, fuzz);
					world.push(Box::new(Sphere::new(center, 0.2, mat)));
				},
				_ => {
					let mat = Dielectric::new(1.5);
					world.push(Box::new(Sphere::new(center, 0.2, mat)));
				},
			};
		}
	}

	world
}

fn main() {
	let world = gen_world();

	let cam_position = Point3::new(13.0, 2.0, -3.0);
	let cam_look_at = Point3::new(0.0, 0.0, 0.0);
	let cam_up = Point3::new(0.0, 1.0, 0.0);
	let camera = Camera::new(cam_position, cam_look_at, cam_up, IMAGE_WIDTH, IMAGE_HEIGHT, VERTICAL_FOV);


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
