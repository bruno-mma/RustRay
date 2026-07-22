use crate::hit_record::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::mem::swap;

#[derive(Clone, Copy)]
pub struct AABB {
	min: Point3,
	max: Point3,
}

impl AABB {
	pub fn new(a: Point3, b: Point3) -> AABB {
		AABB {
			min: Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z())),
			max: Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z())),
		}
	}
}

impl Hittable for AABB {
	fn hit(&'_ self, ray: Ray, mut t_min: f64, mut t_max: f64) -> Option<HitRecord<'_>> {
		for axis_i in 0..=2 {
			let axis_min = self.min[axis_i];
			let axis_max = self.max[axis_i];
			let ray_axis_dir_inv = 1.0 / ray.direction()[axis_i];

			let mut t0 = (axis_min - ray.origin()[axis_i]) * ray_axis_dir_inv;
			let mut t1 = (axis_max - ray.origin()[axis_i]) * ray_axis_dir_inv;
			if t0 > t1 {
				swap(&mut t0, &mut t1);
			}

			t_min = t_min.max(t0);
			t_max = t_max.min(t1);

			if t_max <= t_min {
				return None;
			}
		}

		Some(HitRecord::new(
			ray.at(t_min),
			Vec3::new_zero(),
			t_min,
			false,
			None,
		))
	}
}
