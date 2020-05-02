use crate::kd_tree::Point;
use fs_extra::file::read_to_string;
use serde_json::from_str;
use std::collections::HashMap;
use std::env;

use crate::kd_tree::KdTree;

#[derive(Deserialize)]
struct Input {
	rpu_off: f32,
	rpu_on: f32,
	seq: String,
}

#[derive(Deserialize)]
struct FnRpu {
	name: String,
	rpu_on: f32,
	rpu_off: f32,
}

pub fn build_trees() {
	let dir = env::current_dir().unwrap();
	let rf_path = format!("{}/datasets/{}", dir.display(), "rf_rpus.json");
	let inputs_path = format!("{}/datasets/{}", dir.display(), "inputs.json");

	let f = read_to_string(rf_path).unwrap();
	let input_f = read_to_string(inputs_path).unwrap();
	let rpu_vec: Vec<FnRpu> = from_str(&f).unwrap();
	let input_map: HashMap<String, Input> = from_str(&input_f).unwrap();
	let mut points: Vec<Point> = rpu_vec
		.iter()
		.map(|item| Point {
			name: item.name.to_owned(),
			p: [item.rpu_off, item.rpu_on],
		})
		.collect();
	let input_points: Vec<Point> = input_map
		.iter()
		.map(|(key, value)| Point {
			name: key.to_owned(),
			p: [value.rpu_off, value.rpu_on],
		})
		.collect();

	points.extend(input_points);

	let mut kd = KdTree::new(2);
	kd.build(&mut points);
	kd.save("kd_tree");
}
