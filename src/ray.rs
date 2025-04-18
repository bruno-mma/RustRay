use crate::color::Color;
use crate::hit_record::Hittable;
use crate::vec3::{Point3, Vec3};
use crate::world::World;

#[derive(Clone, Copy)]
pub struct Ray {
	origin: Point3,
	direction: Vec3
}

impl Ray {
	pub fn new(origin: Vec3, direction: Vec3) -> Ray {
		Ray {
			origin,
			direction
		}
	}

	pub fn at(self, t: f64) -> Point3 {
		self.origin + (self.direction * t)
	}

	pub fn origin(self) -> Point3 {
		self.origin
	}

	pub fn direction(self) -> Vec3 {
		self.direction
	}

	pub fn cast(self, world: &World, t_min: f64, t_max: f64, depth: u8) -> Color {
		if depth == 0 {
			return Color::new_zero();
		}

		if let Some(hit_record) = world.hit(self, t_min, t_max) {
			if let Some((color, reflection_ray)) = hit_record.material.scatter(&self, &hit_record) {
				return color * reflection_ray.cast(world, t_min, t_max, depth - 1);
			}
			unreachable!("Hit scatter did not produce a reflection ray")
		}

		// did not hit anything, use background color

		// blended Value = (1 − a) * startValue + a * endValue, 0.0 <= a <= 1.0
		let dir_normalized = self.direction().normalized();
		let a = 0.5 * (dir_normalized.y() + 1.0);
		(1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
	}
}