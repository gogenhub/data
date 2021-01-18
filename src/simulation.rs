use std::collections::HashMap;

use crate::plotter::plot;

use argmin::prelude::*;
use argmin::solver::brent::Brent;

#[derive(Deserialize, Serialize, Debug)]
struct Gate {
	name: String,
	parts: Vec<String>,
	promoter: String,
	params: HashMap<String, f64>,
}

#[derive(Clone, Copy)]
pub struct Gene {
	pub ymax: f64,
	pub k: f64,
	pub n: f64,
	pub decay: f64,
	pub ymin: f64,
}

impl ArgminOp for Gene {
	type Param = f64;
	type Output = f64;
	type Hessian = ();
	type Jacobian = ();
	type Float = f64;

	fn apply(&self, p: &Self::Param) -> Result<Self::Output, Error> {
		Ok(self.ymax - p * (1. + p.powf(self.n)))
	}
}

impl Gene {
	pub fn find_fixed_point(&self) -> f64 {
		let init_param = 0.1;
		let solver = Brent::new(0., self.ymax, 1e-18);
		let res = Executor::new(self.clone(), solver, init_param)
			.max_iters(100)
			.run()
			.unwrap();

		res.state.get_best_param()
	}

	pub fn will_oscilate(&self) -> bool {
		(self.ymin + self.strength()) > self.min_b()
	}

	pub fn a(&self) -> f64 {
		let x = self.find_fixed_point();
		self.n * x.powf(self.n) / (1. + x.powf(self.n))
	}

	pub fn model(&self, x: f64, y: f64) -> f64 {
		self.transfer(x) - self.decay * y
	}

	pub fn transfer(&self, x: f64) -> f64 {
		self.strength() / (1.0 + (x / self.k).powf(self.n))
	}

	pub fn steady_state(&self, x: f64) -> f64 {
		self.transfer(x) / self.decay
	}

	pub fn strength(&self) -> f64 {
		self.ymax
	}

	pub fn min_b(&self) -> f64 {
		let coef = self.n / 2.;
		let br = self.n / 2. - 1.;
		let pow = (self.n + 1.) / self.n;
		coef * br.powf(-pow)
	}
}

pub struct Repressilator {
	genes: [Gene; 3],
}

impl Repressilator {
	pub fn new(genes: [Gene; 3]) -> Self {
		Self { genes }
	}

	pub fn simulate(&self, x: (f64, f64, f64)) {
		let mut x = x.clone();
		let mut data1 = Vec::new();
		let mut data2 = Vec::new();
		let mut data3 = Vec::new();
		for i in 0..1000 {
			let x1 = self.genes[0].model(x.2, x.0);
			let x2 = self.genes[1].model(x.0, x.1);
			let x3 = self.genes[2].model(x.1, x.2);

			x = (x1 + x.0, x2 + x.1, x3 + x.2);
			data1.push((i as f64, x.0));
			data2.push((i as f64, x.1));
			data3.push((i as f64, x.2));
		}

		plot(&[data1, data2, data3], 4);
	}

	pub fn analyze(&self) {
		let x = (0.1, 0.1, 0.2);

		let A = &mut [
			-self.genes[0].decay,
			0.,
			-self.genes[2].a(),
			-self.genes[0].a(),
			-self.genes[1].decay,
			0.,
			0.,
			-self.genes[1].a(),
			-self.genes[2].decay,
		];

		self.simulate(x);
	}
}

pub fn start() {
	let param1 = Gene {
		ymin: 0.02188976,
		k: 0.1,
		ymax: 6.,
		n: 2.8,
		decay: 0.2,
	};

	let param2 = Gene {
		ymin: 0.02188976,
		k: 0.5,
		ymax: 5.,
		n: 2.9,
		decay: 0.1,
	};

	let param3 = Gene {
		ymin: 0.02188976,
		k: 0.9,
		ymax: 12.,
		n: 2.4,
		decay: 0.1,
	};
	println!(
		"{} {} {}",
		param1.will_oscilate(),
		param2.will_oscilate(),
		param3.will_oscilate()
	);

	let rep = Repressilator::new([param1, param2, param3]);

	rep.analyze();
}
