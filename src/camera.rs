use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::world::World;
use crate::{MAX_DEPTH, SAMPLES_PER_PIXEL, SAMPLE_OFFSET_RANGE, T_MAX, T_MIN};
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub struct Camera {
	position: Point3,
	top_left_pixel: Point3,
	pixel_delta_h: Vec3,
	pixel_delta_v: Vec3,
	image_width: u32,
	image_height: u32,
}

impl Camera {
	pub fn new(position: Point3, look_at: Point3, up: Point3, image_width: u32, image_height: u32, v_fov: f64) -> Camera {
		let focal_length = (look_at - position).length();

		let viewport_height = 2.0 * (v_fov / 2.0).to_radians().tan() * focal_length;
		let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

		let w = (position - look_at).normalized(); // view opposite direction
		let u = w.cross(&up).normalized(); // camera right
		let v = u.cross(&w); // camera up

		let viewport_h = u * viewport_width; // vector across viewport horizontal edge
		let viewport_v = -v * viewport_height; // vector down viewport vertical edge

		let pixel_delta_h = viewport_h / image_width as f64;
		let pixel_delta_v = viewport_v / image_height as f64;

		let viewport_top_left = position + (-w * focal_length) - (viewport_h / 2.0) - (viewport_v / 2.0);
		let top_left_pixel = viewport_top_left + (pixel_delta_h / 2.0) + (pixel_delta_v / 2.0);

		Camera {
			position,
			top_left_pixel,
			pixel_delta_h, 
			pixel_delta_v,
			image_width,
			image_height,
		}
	}

	fn get_ray_for_pixel_with_offset(&self, row: u32, v_offset: f64, column: u32, h_offset: f64) -> Ray {
		let u = column as f64 + h_offset;
		let v = row as f64 + v_offset;
		let pixel_center = self.top_left_pixel + (self.pixel_delta_h * u) + (self.pixel_delta_v * v);

		let ray_direction = pixel_center - self.position;

		Ray::new(self.position, ray_direction)
	}

	fn get_ray_for_pixel(&self, row: u32, column: u32) -> Ray {
		self.get_ray_for_pixel_with_offset(row, 0.0, column, 0.0)
	}

	pub fn render(&self, world: &World) -> Vec<Color> {
		(0..self.image_height).into_par_iter().flat_map(|j| {
			(0..self.image_width).map(|i| {
				let mut color_acc = Color::new_zero();
				// FIXME: avoid creating a thread_rng for each pixel
				let mut rng = rand::thread_rng();

				for _ in 0..SAMPLES_PER_PIXEL {
					let rnd_v_offset: f64 = rng.gen_range(SAMPLE_OFFSET_RANGE);
					let rnd_h_offset: f64 = rng.gen_range(SAMPLE_OFFSET_RANGE);

					let ray = self.get_ray_for_pixel_with_offset(j, rnd_v_offset, i, rnd_h_offset);
					color_acc += ray.cast(world, T_MIN, T_MAX, MAX_DEPTH);
				}

				color_acc / SAMPLES_PER_PIXEL as f64
			}).collect::<Vec<Color>>()
		}).collect()
	}
}