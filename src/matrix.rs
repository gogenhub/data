use ndarray::*;
use ndarray_linalg::*;
use num_complex::Complex;

type MatrixComplex = Vec<Vec<Complex<f64>>>;
type Matrix = Vec<Vec<f64>>;
type VectorComplex = Vec<Complex<f64>>;
type Vector = Vec<f64>;

const width: usize = 3;

pub fn matrix_to_vec(matrix: Matrix) -> Vec<f64> {
	let mut mx = Vec::new();
	for i in 0..width {
		for j in 0..width {
			mx.push(matrix[i][j]);
		}
	}
	mx
}

pub fn product_vec(c: f64, vector: &Vector) -> Vector {
	let mut new_vec = Vec::new();
	for r in vector {
		new_vec.push(c * r);
	}
	new_vec
}

pub fn add_vec(lv: &Vector, rv: &Vector) -> Vector {
	let mut new_vec = Vec::new();
	for i in 0..rv.len() {
		new_vec.push(lv[i] + rv[i]);
	}
	new_vec
}

pub fn sub_vec(lv: &Vector, rv: &Vector) -> Vector {
	let mut new_vec = Vec::new();
	for i in 0..rv.len() {
		new_vec.push(lv[i] - rv[i]);
	}
	new_vec
}

pub fn get_bs(vec: &VectorComplex) -> (Vector, Vector) {
	let mut re = Vec::new();
	let mut im = Vec::new();
	for c in vec {
		re.push(c.re);
		im.push(c.im);
	}
	(re, im)
}

pub fn eigen(matrix: Matrix) -> (VectorComplex, MatrixComplex) {
	let mx = matrix_to_vec(matrix);
	let a = Array::from_shape_vec((width, width), mx).unwrap();
	let (evals, evecs) = a.eig().unwrap();

	let mut new_evals = vec![Complex::new(0.0, 0.0); width];
	for i in 0..width {
		new_evals[i].re = evals[i].re;
		new_evals[i].im = evals[i].im;
	}

	let raw_vec = evecs.into_raw_vec();
	let cz = Complex::<f64>::from(0.0);
	let mut new_evecs = vec![vec![cz; width]; width];
	for i in 0..width {
		for j in 0..width {
			let index = i * width + j;
			new_evecs[i][j].re = raw_vec[index].re;
			new_evecs[i][j].im = raw_vec[index].im;
		}
	}
	(new_evals, new_evecs)
}

pub fn general_solution(m: Matrix, t: f64) -> Matrix {
	let (eigenvalues, eigenvectors) = eigen(m);
	for ev in eigenvalues.clone() {
		println!("{}", ev);
	}
	for evec in eigenvectors.clone() {
		println!("{:?}", evec);
	}

	let e1 = (eigenvalues[2].re * t).exp();
	let (bc, _) = get_bs(&eigenvectors[2]);

	let e2 = (eigenvalues[1] * t).exp();
	let (a, b) = (e2.re, e2.im);
	let (b1, b2) = get_bs(&eigenvectors[1]);

	let cos1 = product_vec(a, &b1);
	let sin1 = product_vec(b, &b2);
	let cos2 = product_vec(a, &b2);
	let sin2 = product_vec(b, &b1);

	let x0 = product_vec(e1, &bc);
	let x1 = sub_vec(&cos1, &sin1);
	let x2 = add_vec(&cos2, &sin2);

	vec![x0, x1, x2]
}

pub fn find_constants(m: Matrix, res: Vec<f64>) -> Vec<f64> {
	let t = 0.0;
	let sol = general_solution(m, t);
	let x0 = sol[0].clone();
	let x1 = sol[1].clone();
	let x2 = sol[2].clone();
	let entries = vec![
		(0, 0, x0[0]),
		(0, 1, x1[0]),
		(0, 2, x2[0]),
		(1, 0, x0[1]),
		(1, 1, x1[1]),
		(1, 2, x2[1]),
		(2, 0, x0[2]),
		(2, 1, x1[2]),
		(2, 2, x2[2]),
	];
	let mut m = sparse21::Matrix::from_entries(entries);

	let soln = m.solve(res).unwrap();
	soln
}

pub fn run(m: Matrix, consts: &Vec<f64>, t: f64) -> Vector {
	let sol = general_solution(m, t);
	let mut res = vec![0.0; width];
	for i in 0..width {
		let pr = product_vec(consts[i], &sol[i].clone());
		res = add_vec(&res, &pr);
	}
	res
}
