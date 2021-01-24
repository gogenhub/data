#[macro_use]
extern crate serde_derive;

mod clean_up;
mod matrix;
mod plotter;
mod simulation;

use matrix::{find_constants, general_solution, run};
use plotter::plot;

fn main() {
	let v1 = vec![-0.2, 0.0, -2.0059707967319333];
	let v2 = vec![-2.1061890623329234, -0.1, 0.0];
	let v3 = vec![0.0, -2.093908522219756, -0.1];

	let m = vec![v1, v2, v3];
	let sol = general_solution(m.clone(), 0.0);
	let consts = find_constants(sol, vec![0.1, 0.1, 0.2]);
	let res = run(m.clone(), &consts, 1000.0);
	println!("res: {:?}", res);
	// let mut pts1 = Vec::new();
	// let mut pts2 = Vec::new();
	// let mut pts3 = Vec::new();
	// for i in 0..600 {
	// 	let t = i as f64 / 10.0;
	// 	let res = run(m.clone(), &consts, t);
	// 	pts1.push((t, res[0]));
	// 	pts2.push((t, res[1]));
	// 	pts3.push((t, res[2]));
	// }
	// plot(&[pts1, pts2, pts3], 3);
}
