use crate::hit_record::{HitRecord, Hittable};
use crate::ray::Ray;

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
	fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut tmp_rec = None;
		let mut closest_so_far = t_max;

		for object in self {
			if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
				if rec.t < closest_so_far {
					closest_so_far = rec.t;
					tmp_rec = Some(rec);
				}
			}
		}

		tmp_rec
	}
}