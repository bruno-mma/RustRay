pub(crate) mod dielectric;
pub(crate) mod metal;
pub(crate) mod lambertian;

use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Material {
	fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}
