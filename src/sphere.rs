use crate::hit_record::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
	center: Point3,
	radius: f64,
}

impl Sphere {
	pub fn new(center: Point3, radius: f64) -> Sphere {
		Sphere {
			center,
			radius,
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc = self.center - ray.origin();
		let a = ray.direction().length_squared();
		let half_b = oc.dot(&ray.direction());
		let c = oc.length_squared() - self.radius * self.radius;
		
		let discriminant = half_b * half_b - a * c;
		if discriminant < 0.0 {
			return None;
		}

		// Find the nearest root that lies in the acceptable range.
		let sqrt_discriminant = discriminant.sqrt();
		let mut root = (half_b - sqrt_discriminant) / a;
		if root <= t_min || root >= t_max {
			root = (half_b + sqrt_discriminant) / a;
			if root <= t_min || root >= t_max {
				return None;
			}
		}

		let mut hit = HitRecord::new(ray.at(root), Vec3::new_zero(), root, false);
		let outward_normal = (hit.hit_point - self.center) / self.radius;
		hit.set_face_normal(&ray, outward_normal);

		Some(hit)
	}
}