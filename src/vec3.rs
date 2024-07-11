use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, RangeInclusive, Sub, SubAssign};

use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vec3 {
	e: [f64; 3]
}

pub type Point3 = Vec3;

const NEAR_ZERO: f64 = 1e-8;

impl Vec3 {
	pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
		Vec3 { e: [e0, e1, e2] }
	}

	pub fn new_diag(e: f64) -> Vec3 {
		Vec3 { e: [e, e, e] }
	}

	pub fn new_zero() -> Vec3 {
		Vec3::new_diag(0.0)
	}

	pub fn new_rand_range(range: RangeInclusive<f64>) -> Vec3 {
	    let mut rng = rand::thread_rng();
	    Vec3 {
	        e: [
	            rng.gen_range(range.clone()),
	            rng.gen_range(range.clone()),
	            rng.gen_range(range)
	        ]
	    }
	}
	
	pub fn new_rand_normalized() -> Vec3 {
		Vec3::new_rand_range(-1.0..=1.0).normalized()
	}
	
	pub fn new_rand_normalized_on_hemisphere(&self) -> Vec3 {
	    let rand_normalized = Vec3::new_rand_normalized();
		if rand_normalized.dot(self) > 0.0 {
			rand_normalized
		} else {
			rand_normalized * -1.0
		}
	}

	pub fn x(&self) -> f64 {
		self[0]
	}

	pub fn y(&self) -> f64 {
		self[1]
	}

	pub fn z(&self) -> f64 {
		self[2]
	}

	pub fn dot(&self, other: &Vec3) -> f64 {
		self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
	}
	
	pub fn length_squared(&self) -> f64 {
		self.dot(self)
	}

	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn cross(&self, other: &Vec3) -> Vec3 {
		Vec3 {
			e: [
				self[1] * other[2] - self[2] * other[1],
				self[2] * other[0] - self[0] * other[2],
				self[0] * other[1] - self[1] * other[0]
			]
		}
	}
	
	pub fn normalized(&self) -> Vec3 {
		*self / self.length()
	}

	pub fn is_near_zero(&self) -> bool {
		self[0].abs() < NEAR_ZERO && self[1].abs() < NEAR_ZERO && self[2].abs() < NEAR_ZERO
	}
}

impl Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, index: usize) -> &Self::Output {
		&self.e[index]
	}
}

impl IndexMut<usize> for Vec3 {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.e[index]
	}
}

impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, other: Self) -> Self::Output {
		Vec3 {
			e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
		}
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		*self = Vec3 {
			e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
		}
	}
}

impl Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Self) -> Self::Output {
		Vec3 {
			e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
		}
	}
}

impl SubAssign for Vec3 {
	fn sub_assign(&mut self, other: Vec3) {
		*self = Vec3 {
			e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
		};
	}
}

impl Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: f64) -> Self::Output {
		Vec3 {
			e: [self[0] * other, self[1] * other, self[2] * other]
		}
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Self::Output {
		Vec3 {
			e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]]
		}
	}
}

impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, other: f64) {
		*self = Vec3 {
			e: [self[0] * other, self[1] * other, self[2] * other]
		};
	}
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Self::Output {
		Vec3 {
			e: [self * other[0], self * other[1], self * other[2]]
		}
	}
}

impl Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, other: f64) -> Self::Output {
		Vec3 {
			e: [self[0] / other, self[1] / other, self[2] / other]
		}
	}
}

impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, other: f64) {
		*self = Vec3 {
			e: [self[0] / other, self[1] / other, self[2] / other]
		};
	}
}

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
	}
}