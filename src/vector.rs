use argmin::prelude::Zero;
use num_complex::Complex;
use std::convert::From;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};
use std::slice::{Iter, IterMut};

#[derive(Clone)]
pub struct Vector(pub Vec<Complex<f64>>);

impl Vector {
	pub fn iter(&self) -> Iter<'_, Complex<f64>> {
		self.0.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, Complex<f64>> {
		self.0.iter_mut()
	}

	pub fn zeros(size: usize) -> Self {
		Self(vec![Complex::<f64>::zero(); size])
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}
}

impl From<Vec<Complex<f64>>> for Vector {
	fn from(value: Vec<Complex<f64>>) -> Self {
		Self(value.to_vec())
	}
}

impl From<Vec<f64>> for Vector {
	fn from(value: Vec<f64>) -> Self {
		Self(value.iter().map(|x| Complex::from(x)).collect())
	}
}

impl Mul<Complex<f64>> for Vector {
	type Output = Self;

	fn mul(self, rhs: Complex<f64>) -> Self::Output {
		Self(self.iter().map(|c| c * rhs).collect())
	}
}

impl Mul<Vector> for Complex<f64> {
	type Output = Vector;

	fn mul(self, rhs: Vector) -> Self::Output {
		rhs * self
	}
}

impl Mul<f64> for Vector {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		Self(self.iter().map(|c| c * rhs).collect())
	}
}

impl Mul<Vector> for f64 {
	type Output = Vector;

	fn mul(self, rhs: Vector) -> Self::Output {
		rhs * self
	}
}

impl Mul<Vector> for Vector {
	type Output = Vector;

	fn mul(self, rhs: Vector) -> Self::Output {
		Self(self.iter().zip(rhs.iter()).map(|(l, r)| l * r).collect())
	}
}

impl Add<Complex<f64>> for Vector {
	type Output = Self;

	fn add(self, rhs: Complex<f64>) -> Self::Output {
		Self(self.iter().map(|c| c + rhs).collect())
	}
}

impl Add<Vector> for Complex<f64> {
	type Output = Vector;

	fn add(self, rhs: Vector) -> Self::Output {
		rhs + self
	}
}

impl Add<f64> for Vector {
	type Output = Self;

	fn add(self, rhs: f64) -> Self::Output {
		Self(self.iter().map(|c| c + rhs).collect())
	}
}

impl Add<Vector> for f64 {
	type Output = Vector;

	fn add(self, rhs: Vector) -> Self::Output {
		rhs + self
	}
}

impl AddAssign<Vector> for Vector {
	fn add_assign(&mut self, rhs: Vector) {
		for (l, r) in self.iter_mut().zip(rhs.iter()) {
			*l += r
		}
	}
}

impl Sub<Complex<f64>> for Vector {
	type Output = Self;

	fn sub(self, rhs: Complex<f64>) -> Self::Output {
		Self(self.0.iter().map(|c| c - rhs).collect())
	}
}

impl Sub<Vector> for Complex<f64> {
	type Output = Vector;

	fn sub(self, rhs: Vector) -> Self::Output {
		Vector(rhs.0.iter().map(|c| self - c).collect())
	}
}

impl Sub<f64> for Vector {
	type Output = Self;

	fn sub(self, rhs: f64) -> Self::Output {
		Self(self.0.iter().map(|c| rhs - c).collect())
	}
}

impl Sub<Vector> for f64 {
	type Output = Vector;

	fn sub(self, rhs: Vector) -> Self::Output {
		Vector(rhs.0.iter().map(|c| self - c).collect())
	}
}

impl Index<usize> for Vector {
	type Output = Complex<f64>;

	fn index(&self, idx: usize) -> &Complex<f64> {
		&self.0[idx]
	}
}

impl IndexMut<usize> for Vector {
	fn index_mut(&mut self, idx: usize) -> &mut Complex<f64> {
		&mut self.0[idx]
	}
}
