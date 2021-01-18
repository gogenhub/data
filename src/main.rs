#[macro_use]
extern crate serde_derive;

mod clean_up;
mod matrix;
mod plotter;
mod simulation;
mod vector;

use matrix::Matrix;
use plotter::plot;
use vector::Vector;

fn main() {
	simulation::start();
	let v1 = Vector::from(vec![-0.2, 0.0, -2.0059707967319333]);
	let v2 = Vector::from(vec![-2.1061890623329234, -0.1, 0.0]);
	let v3 = Vector::from(vec![0.0, -2.093908522219756, -0.1]);

	let m = Matrix::from(vec![v1, v2, v3]);

	let consts = m.find_constants(vec![0.1, 0.1, 0.2]);
	println!("consts: {:?}", consts);
	let ampl = m.amplitude(consts.clone());
	println!("amplitudes: {:?}", ampl);
	let mut pts1 = Vec::new();
	let mut pts2 = Vec::new();
	let mut pts3 = Vec::new();
	for i in 0..200 {
		let t = i as f64;
		let res = m.run(&consts, t);
		pts1.push((t, res[0].re + res[0].im));
		pts2.push((t, res[1].re + res[1].im));
		pts3.push((t, res[2].re + res[2].im));
	}
	plot(&[pts1, pts2, pts3], 3);

	// let mut xs = Vec::new();
	// let mut ys = Vec::new();
	// let mut x = 0.1;
	// let mut y = 0.1;
	// for j in 0..200 {
	// 	let t = j as f64;
	// 	x += y;
	// 	y += -0.1 * x;
	// 	xs.push((t, x));
	// 	ys.push((t, y));
	// }
	// plot(&[xs, ys], 4);
}
