use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord<'a> {
	pub hit_point: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
	pub material: &'a dyn Material,
}

impl HitRecord<'_> {
	pub fn new(hit_point: Point3, normal: Vec3, t: f64, front_face: bool, material: &dyn Material) -> HitRecord {
		HitRecord {
			hit_point,
			normal,
			t,
			front_face,
			material,
		}
	}

	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
		self.front_face = r.direction().dot(&outward_normal) < 0.0;
		self.normal = if self.front_face {
			outward_normal
		} else {
			(-1.0) * outward_normal
		};
	}
}

pub trait Hittable : Sync {
	fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

