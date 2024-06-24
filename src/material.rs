use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Scater {
	fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}