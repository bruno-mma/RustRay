use std::fs::File;
use std::io::Write;

use rand::Rng;

use camera::Camera;
use color::Color;
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

const IMAGE_WIDTH: u32 = 1000;
const IMAGE_HEIGHT: u32 = 800;

const SAMPLES_PER_PIXEL: u8 = 32;
const MAX_DEPTH: u8 = 32;

const T_MIN: f64 = 0.001;
const T_MAX: f64 = f64::INFINITY;

fn main() {
	let mut world = World::new();
	world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5)));
	world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, 1.0), 100.0)));

	let mut pixel_data: Vec<Color> = Vec::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT) as usize);
	
	let camera_position = Point3::new_zero();
	let camera = Camera::new(camera_position, IMAGE_WIDTH, IMAGE_HEIGHT);

	let mut rng = rand::thread_rng();

	println!("Starting render...");
	for j in 0..IMAGE_HEIGHT {
		println!("Image lines remaining: {:3}", IMAGE_HEIGHT - j);
		for i in 0..IMAGE_WIDTH {
			let ray = camera.get_ray_for_pixel(j, i);
			let mut color_acc = ray.cast(&world, T_MIN, T_MAX, MAX_DEPTH);

			for _ in 1..SAMPLES_PER_PIXEL {
				let rnd_v_offset: f64 = rng.gen_range(-0.5..=0.5);
				let rnd_h_offset: f64 = rng.gen_range(-0.5..=0.5);

				let ray = camera.get_ray_for_pixel_with_offset(j, rnd_v_offset, i, rnd_h_offset);
				color_acc += ray.cast(&world, T_MIN, T_MAX, MAX_DEPTH);
			}

			pixel_data.push(color_acc / SAMPLES_PER_PIXEL as f64);
		}
	}

	let file_data = String::from(
		format!(
			"P3\n{} {}\n255\n{}\n",
			IMAGE_WIDTH,
			IMAGE_HEIGHT,
			pixel_data.iter()
				.map(|pixel_color| pixel_color.ppm_format_ln())
				.collect::<String>()
		)
	);

	println!("Rendering complete, writing to file...");
	let mut file = File::create("image.ppm").unwrap();
	file.write_all(file_data.as_bytes()).unwrap();
	println!("Done");
}
