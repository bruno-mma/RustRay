use crate::vec3::Vec3;
use std::io::{self, Write};

pub type Color = Vec3;

impl Color {
	pub fn write_ppm<W: Write>(&self, writer: &mut W) -> io::Result<()> {
		let r = linear_to_gamma(self[0]) * 256.0;
		let g = linear_to_gamma(self[1]) * 256.0;
		let b = linear_to_gamma(self[2]) * 256.0;
		writeln!(
			writer,
			"{} {} {}",
			r as u8,
			g as u8,
			b as u8
		)
	}
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
	linear_component.clamp(0.0, 0.999999).sqrt()
}