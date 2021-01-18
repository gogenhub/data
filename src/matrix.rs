use crate::vector::Vector;
use ndarray::*;
use ndarray_linalg::*;
use num_complex::Complex;
use std::convert::From;
use std::ops::Index;

pub struct Matrix {
	value: Vec<Vector>,
	rows: usize,
	cols: usize,
}

impl From<Vec<Vector>> for Matrix {
	fn from(a: Vec<Vector>) -> Self {
		Self {
			value: a.to_vec(),
			rows: a.len(),
			cols: a.len(),
		}
	}
}

impl Matrix {
	pub fn to_vec(&self) -> Vec<f64> {
		let mut mx = Vec::new();
		for i in 0..self.rows {
			for j in 0..self.cols {
				mx.push(self.value[i][j].re);
			}
		}
		mx
	}

	pub fn eigen(&self) -> (Vector, Vec<Vector>) {
		let mx = self.to_vec();
		let a = Array::from_shape_vec((self.rows, self.cols), mx).unwrap();
		let (evals, evecs) = a.eig().unwrap();

		let mut new_evals = vec![Complex::new(0.0, 0.0); self.rows];
		for i in 0..self.rows {
			new_evals[i].re = evals[i].re;
			new_evals[i].im = evals[i].im;
		}

		let raw_vec = evecs.into_raw_vec();
		let mut new_evecs = vec![Vector::zeros(self.rows); self.cols];
		for i in 0..self.rows {
			for j in 0..self.cols {
				let index = i * self.rows + j;
				new_evecs[i][j].re = raw_vec[index].re;
				new_evecs[i][j].im = raw_vec[index].im;
			}
		}
		(Vector::from(new_evals), new_evecs)
	}

	pub fn general_solution(&self, t: f64) -> Matrix {
		let (eigenvalues, eigenvectors) = self.eigen();

		let mut res = Vec::new();
		for i in 0..self.rows {
			res.push((eigenvalues[i] * t).exp() * eigenvectors[i].clone());
		}

		Matrix::from(res)
	}

	pub fn amplitude(&self, consts: Vec<f64>) -> Vec<f64> {
		let t = 1.;
		let (eigenvalues, eigenvectors) = self.eigen();

		let mut res = Vector::zeros(self.rows);
		for i in 0..eigenvalues.len() {
			if eigenvalues[i].im != 0.0 {
				res += consts[i] * (eigenvalues[i] * t).exp() * eigenvectors[i].clone();
			}
		}
		let mut m = Vec::new();
		for c in res.iter() {
			m.push((c.re.powf(2.0) + c.im.powf(2.0)).sqrt());
		}
		m
	}

	pub fn find_constants(&self, res: Vec<f64>) -> Vec<f64> {
		let t = 0.0;
		let sol = self.general_solution(t);
		let mut entries = Vec::new();
		for i in 0..self.rows {
			for j in 0..self.cols {
				let c = sol[j][i];
				entries.push((i, j, c.re + c.im));
			}
		}
		let mut m = sparse21::Matrix::from_entries(entries);

		let soln = m.solve(res).unwrap();
		soln
	}

	pub fn run(&self, consts: &Vec<f64>, t: f64) -> Vector {
		let sol = self.general_solution(t);
		let mut res = Vector::zeros(self.rows);
		for i in 0..consts.len() {
			res += consts[i] * sol[i].clone();
		}
		res
	}
}

impl Index<usize> for Matrix {
	type Output = Vector;

	fn index(&self, idx: usize) -> &Vector {
		&self.value[idx]
	}
}
