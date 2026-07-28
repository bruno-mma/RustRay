use crate::color::Color;
use crate::hit_record::{HitRecord, Hittable};
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use rand::Rng;

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

pub fn gen_world_big() -> World {
	let mut world = World::new();

	let ground_material = Lambertian::new(Color::new(0.6, 0.6, 0.6));
	world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

	let mat_1 = Dielectric::new(1.5);
	world.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat_1)));

	let mat_2 = Lambertian::new(Color::new(0.2, 0.2, 0.8));
	world.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat_2)));

	let mat_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
	world.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat_3)));

	let mut rng =  rand::thread_rng();

	for a in -11..=11 {
		for b in -11..=11 {
			if  a < 5 && a > -5 && b < 2 && b > -2 {
				continue;
			}

			let center = Point3::new(
				a as f64 + 0.9 * rng.gen_range(0.0..=1.0),
				0.2,
				b as f64 + 0.9 * rng.gen_range(0.0..=1.0)
			);

			let mat_float = rng.gen_range(0.0..=1.0);
			match mat_float {
				x if x < 0.8 => {
					let albedo = Color::new_rand_range(0.0..=1.0);
					let mat = Lambertian::new(albedo);
					world.push(Box::new(Sphere::new(center, 0.2, mat)));
				},
				x if x < 0.95 => {
					let albedo = Color::new_rand_range(0.5..=1.0);
					let fuzz = rng.gen_range(0.0..=0.1);
					let mat = Metal::new(albedo, fuzz);
					world.push(Box::new(Sphere::new(center, 0.2, mat)));
				},
				_ => {
					let mat = Dielectric::new(1.5);
					world.push(Box::new(Sphere::new(center, 0.2, mat)));
				},
			};
		}
	}

	world
}

pub fn gen_world() -> World {
	let mut world = World::new();

	let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
	let material_right = Lambertian::new(Color::new(0.1, 0.2, 0.5));
	let material_left = Lambertian::new(Color::new(0.9, 0.1, 0.1));
	let material_center = Metal::new(Color::new(0.8, 0.8, 0.8), 0.0);

	world.push(Box::new(Sphere::new(
		Point3::new(0.0, -100.5, 1.0),
		100.0,
		material_ground,
	)));
	world.push(Box::new(Sphere::new(
		Point3::new(0.0, 0.0, 1.2),
		0.5,
		material_center,
	)));
	world.push(Box::new(Sphere::new(
		Point3::new(-1.0, 0.0, 1.0),
		0.5,
		material_left,
	)));
	world.push(Box::new(Sphere::new(
		Point3::new(1.0, 0.0, 1.0),
		0.5,
		material_right,
	)));

	world
}