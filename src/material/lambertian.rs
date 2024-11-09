use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
	albedo: Color,
}

impl Lambertian {
	pub fn new(albedo: Color) -> Lambertian {
		Lambertian {
			albedo,
		}
	}
}

impl Material for Lambertian {
	fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
		// Could we use a normal distribution to generate the reflection dir for the same results?
		let mut reflection_dir = hit_record.normal + Vec3::new_rand_unit();
		if reflection_dir.is_near_zero() {
			reflection_dir = hit_record.normal;
		}
		let reflection_ray = Ray::new(hit_record.hit_point, reflection_dir);

		Some((self.albedo, reflection_ray))
	}
}