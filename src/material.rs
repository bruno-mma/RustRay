use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

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
		let mut reflection_dir = hit_record.normal + Vec3::new_rand_normalized();
		if reflection_dir.is_near_zero() {
			reflection_dir = hit_record.normal;
		}
		let reflection_ray = Ray::new(hit_record.hit_point, reflection_dir);
		
		Some((self.albedo, reflection_ray))
	}
}

pub struct Metal {
	albedo: Color,
	fuzz: f64,
}

impl Metal {
	pub fn new(albedo: Color, fuzz: f64) -> Metal {
		Metal {
			albedo,
			fuzz: fuzz.clamp(0.0, 1.0),
		}
	}
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
		let reflection_dir = ray.direction().reflect(&hit_record.normal);
		let reflection_dir_fuzzed = reflection_dir.normalized() + (self.fuzz * Vec3::new_rand_normalized());
		let reflection_ray = Ray::new(hit_record.hit_point, reflection_dir_fuzzed);
		
		Some((self.albedo, reflection_ray))
	}
}