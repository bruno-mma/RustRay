use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
	position: Point3,
	top_left_pixel: Point3,
	pixel_delta_h: Vec3,
	pixel_delta_v: Vec3,
}

impl Camera {
	pub fn new(position: Point3, image_width: u32, image_height: u32) -> Camera {
		let viewport_height = 2.0;
		let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
		
		let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
		let vertical = Vec3::new(0.0, viewport_height, 0.0);

		let pixel_delta_h = horizontal / image_width as f64;
		let pixel_delta_v = vertical / image_height as f64;

		let viewport_top_left = (position + Point3::new(0.0, 0.0, FOCAL_LENGTH)) - (horizontal / 2.0) + (vertical / 2.0);
		let top_left_pixel = (viewport_top_left - position) - (pixel_delta_h / 2.0) + (pixel_delta_v / 2.0);

		Camera {
			position,
			top_left_pixel,
			pixel_delta_h,
			pixel_delta_v
		}
	}
	
	pub fn get_ray_for_pixel_with_offset(&self, row: u32, v_offset: f64, column: u32, h_offset: f64) -> Ray {
		let u = column as f64 + h_offset;
		let v = row as f64 + v_offset;
		let pixel_center = self.top_left_pixel + (self.pixel_delta_h * u) - (self.pixel_delta_v * v);
	
		let ray_direction = pixel_center - self.position;
	
		Ray::new(self.position, ray_direction)
	}
	
	pub fn get_ray_for_pixel(&self, row: u32, column: u32) -> Ray {
		self.get_ray_for_pixel_with_offset(row, 0.0, column, 0.0)
	}
}