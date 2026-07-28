use std::fs::File;
use std::io::Write;
use std::ops::RangeInclusive;
use std::time::Instant;

use camera::Camera;
use color::Color;
use vec3::Point3;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let world = world::gen_world();

	let cam_position = Point3::new(13.0, 2.0, -3.0);
	let cam_look_at = Point3::new(0.0, 0.0, 0.0);
	let cam_up = Point3::new(0.0, 1.0, 0.0);
	let camera = Camera::new(cam_position, cam_look_at, cam_up, IMAGE_WIDTH, IMAGE_HEIGHT, VERTICAL_FOV);

	println!("Starting render...");
	let render_start = Instant::now();

	let pixel_data: Vec<Color> = camera.render(&world);

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

	let mut file = File::create("image.ppm")?;
	file.write_all(file_data.as_bytes())?;

	println!("Done!");
	Ok(())
}
