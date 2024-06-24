use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
	pub fn ppm_format(self) -> String {
		let r = self.x();
		let g = self.y();
		let b = self.z();

		// Apply a linear to gamma transform for gamma 2
		let r_corrected = linear_to_gamma(r);
		let g_corrected = linear_to_gamma(g);
		let b_corrected = linear_to_gamma(b);
		
		format!("{} {} {}", 
		        (255.999999 * r_corrected) as u8,
		        (255.999999 * g_corrected) as u8,
		        (255.999999 * b_corrected) as u8
		)
	}
	
	pub fn ppm_format_ln(self) -> String {
		format!("{}\n", self.ppm_format())
	}
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
	// if linear_component > 0.0 {
	// 	return linear_component.sqrt();
	// }
	// 
	// return 0.0

	linear_component.sqrt()
} 