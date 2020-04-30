use fs_extra::file::read_to_string;
use serde_json::from_str;
use std::collections::HashMap;
use std::env;

use crate::kd_tree::KdTree;

#[derive(Deserialize, Serialize, Clone)]
struct Point {
	name: String,
	x: f32,
	y: f32,
}

#[derive(Deserialize, Clone)]
pub struct Var {
	pub name: String,
	pub off_threshold: f32,
	pub on_threshold: f32,
}

#[derive(Deserialize, Clone)]
pub struct Param {
	pub name: String,
	pub value: f32,
}

#[derive(Deserialize, Clone)]
pub struct ResponseFunction {
	pub name: String,
	pub x: f32,
	pub y: f32,
	pub equation: String,
	pub params: Vec<Param>,
}

pub fn build_trees() {
	let dir = env::current_dir().unwrap();
	let rf_path = format!("{}/datasets/{}", dir.display(), "response_functions.json");

	let f = read_to_string(rf_path).unwrap();
	let response_functions: HashMap<String, ResponseFunction> = from_str(&f).unwrap();
	let mut rf_vec: Vec<ResponseFunction> = response_functions
		.iter()
		.map(|(_, func)| func.clone())
		.collect();

	let mut kd = KdTree::new(2);

	kd.build(&mut rf_vec);
	// kd.remove("P3_PhlF");
	kd.save("kd_tree");
}
