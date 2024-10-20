use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;


pub struct Dielectric {
	// Refractive index in vacuum or air, or the ratio of the material's refractive index over
	// the refractive index of the enclosing media
	refraction_index: f64
}

impl Dielectric {
	pub fn new(refraction_index: f64) -> Dielectric {
		Dielectric {
			refraction_index
		}
	}
}

impl Material for Dielectric {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
		let ray_dir = ray.direction().normalized();
		let n = hit_record.normal;
		let r = if hit_record.front_face {
			1.0 / self.refraction_index
		} else {
			self.refraction_index
		};

		let cos = ray_dir.dot(&-n);
		let sin = (1.0 - cos * cos).sqrt();

		let cannot_refract = r * sin > 1.0;
		let ray_out_dir=
		if cannot_refract || reflectance(cos, r) > rand::random::<f64>() {
			// reflect
			ray_dir.reflect(&n)
		} else {
			// refract
			let r_out_perp = r * (ray_dir + (cos * n));
			let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n;

			r_out_perp + r_out_parallel
		};

		let ray_out = Ray::new(hit_record.hit_point, ray_out_dir);
		Some((Color::new_diag(1.0), ray_out))
	}
}

fn reflectance(cos: f64, refraction_index: f64) -> f64 {
	// Schlick's approximation
	let temp = (1.0 - refraction_index) / (1.0 + refraction_index);
	let r0 = temp * temp;
	r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}