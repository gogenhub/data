use fs_extra::file::read_to_string;
use regex::Regex;
use serde_json::{from_str, from_value, to_string, Value};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{read_dir, write};

#[derive(Deserialize, Serialize)]
pub struct Input {
	rpu_on: f32,
	rpu_off: f32,
	seq: String,
	index: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum GateKind {
	OR,
	NOT,
	NOR,
	AND,
	NAND,
	Unknown,
}

#[derive(Deserialize, Serialize)]
enum PartKind {
	#[serde(alias = "ribozyme")]
	Ribozyme,
	#[serde(alias = "rbs")]
	Rbs,
	#[serde(alias = "cds")]
	Cds,
	#[serde(alias = "promoter")]
	Promoter,
	#[serde(alias = "terminator")]
	Terminator,
	#[serde(alias = "scar")]
	Scar,
	#[serde(alias = "sgRNA")]
	SgRNA,
}

#[derive(Deserialize, Serialize)]
pub struct Part {
	#[serde(alias = "type")]
	kind: PartKind,
	name: String,
	#[serde(alias = "dnasequence")]
	seq: String,
}

#[derive(Deserialize, Serialize)]
pub struct BGate {
	#[serde(alias = "group_name")]
	group: String,
	#[serde(alias = "gate_name")]
	name: String,
	#[serde(alias = "gate_type")]
	kind: GateKind,
}

#[derive(Deserialize, Serialize)]
struct Cassette {
	#[serde(alias = "maps_to_variable")]
	maps_to: String,
	#[serde(alias = "cassette_parts")]
	parts: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct OldGateParts {
	#[serde(alias = "gate_name")]
	name: String,
	#[serde(alias = "expression_cassettes")]
	cassettes: Vec<Cassette>,
	promoter: String,
}

#[derive(Deserialize, Serialize)]
struct Var {
	name: String,
	off_threshold: Option<f64>,
	on_threshold: Option<f64>,
}

#[derive(Deserialize, Serialize, Clone)]
struct Param {
	name: String,
	value: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResponseFunction {
	#[serde(alias = "gate_name")]
	name: String,
	equation: String,
	#[serde(alias = "parameters")]
	params: Vec<Param>,
}

#[derive(Deserialize, Serialize)]
pub struct GateParts {
	name: String,
	parts: Vec<String>,
	promoter: String,
}

#[derive(Deserialize, Serialize)]
struct Point {
	x: f32,
	y: f32,
}

#[derive(Deserialize, Serialize)]
struct Gate {
	name: String,
	parts: Vec<String>,
	promoter: String,
	equation: String,
	params: HashMap<String, f32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NewResponseFunction {
	pub name: String,
	pub equation: String,
	pub params: HashMap<String, f32>,
}

fn extract_parts() {
	let dir = env::current_dir().unwrap();

	let mut parts: HashMap<String, Part> = HashMap::new();
	for entry in read_dir(format!("{}/raw/", dir.display())).unwrap() {
		let f = read_to_string(entry.unwrap().path()).unwrap();
		let json: Vec<Value> = from_str(&f).unwrap();
		for item in json {
			if item["collection"] == "parts" && item["kind"] != "SgRNA" {
				let part: Part = from_value(item.clone()).unwrap();
				parts.insert(part.name.to_owned(), part);
			}
		}
	}

	write(
		format!("{}/datasets/{}", dir.display(), "parts.json"),
		to_string(&parts).unwrap(),
	)
	.unwrap();
}

fn export_gate_types() {
	let dir = env::current_dir().unwrap();

	let mut types: HashMap<String, HashSet<String>> = HashMap::new();
	for entry in read_dir(format!("{}/raw/", dir.display())).unwrap() {
		let f = read_to_string(entry.unwrap().path()).unwrap();
		let json: Vec<Value> = from_str(&f).unwrap();
		for item in json {
			if item["collection"] == "gates"
				&& (item["system"] == "TetR" || item["system"] == "sensor")
			{
				let gate_type = item["gate_type"].as_str().unwrap();
				if !types.contains_key(gate_type) {
					types.insert(gate_type.to_owned(), HashSet::new());
				}

				let gt = types.get_mut(gate_type).unwrap();
				gt.insert(item["gate_name"].as_str().unwrap().to_owned());
			}
		}
	}

	let not = types.get("NOT").unwrap();
	println!("not: {}", not.len());
	let nor = types.get("NOR").unwrap();
	println!("nor: {}", nor.len());

	write(
		format!("{}/datasets/{}", dir.display(), "gate_types.json"),
		to_string(&types).unwrap(),
	)
	.unwrap();
}

fn extract_gate_parts() {
	let dir = env::current_dir().unwrap();

	let mut gate_parts: HashMap<String, GateParts> = HashMap::new();
	for entry in read_dir(format!("{}/raw/", dir.display())).unwrap() {
		let f = read_to_string(entry.unwrap().path()).unwrap();
		let json: Vec<Value> = from_str(&f).unwrap();
		for item in json {
			if item["collection"] == "gate_parts"
				&& item["gate_name"] != "SicA_InvF"
				&& item["gate_name"] != "ExsC_ExsDA"
			{
				let gp: OldGateParts = from_value(item).unwrap();
				let c: Vec<&str> = gp.name.split('_').collect();
				if c[0] != "sgRNA" {
					let new_gate_parts = GateParts {
						name: gp.name.clone(),
						parts: gp.cassettes[0].parts.clone(),
						promoter: gp.promoter,
					};
					gate_parts.insert(gp.name, new_gate_parts);
				}
			}
		}
	}

	println!("gate parts: {}", gate_parts.len());

	write(
		format!("{}/datasets/{}", dir.display(), "gate_parts.json"),
		to_string(&gate_parts).unwrap(),
	)
	.unwrap();
}

fn extract_response_functions() {
	let dir = env::current_dir().unwrap();

	let mut response_functions: HashMap<String, NewResponseFunction> = HashMap::new();
	for entry in read_dir(format!("{}/raw/", dir.display())).unwrap() {
		let f = read_to_string(entry.unwrap().path()).unwrap();
		let json: Vec<Value> = from_str(&f).unwrap();
		for item in json {
			if item["collection"] == "response_functions"
				&& item["gate_name"] != "SicA_InvF"
				&& item["gate_name"] != "ExsC_ExsDA"
			{
				let response_function: ResponseFunction = from_value(item).unwrap();
				let c: Vec<&str> = response_function.name.split('_').collect();
				if c[0] != "sgRNA" {
					let new_params: HashMap<String, f32> = response_function
						.params
						.iter()
						.map(|item| (item.name.to_owned(), item.value))
						.collect();
					let new_rf = NewResponseFunction {
						equation: response_function.equation,
						name: response_function.name.to_owned(),
						params: new_params,
					};
					response_functions.insert(response_function.name.to_owned(), new_rf);
				}
			}
		}
	}

	println!("rp: {}", response_functions.len());

	write(
		format!("{}/datasets/{}", dir.display(), "response_functions.json"),
		to_string(&response_functions).unwrap(),
	)
	.unwrap();
}

fn merge_gate_parts_and_rf() {
	let dir = env::current_dir().unwrap();

	let gp_path = format!("{}/datasets/{}", dir.display(), "gate_parts.json");
	let rf_path = format!("{}/datasets/{}", dir.display(), "response_functions.json");

	let gp_f = read_to_string(gp_path).unwrap();
	let gp: HashMap<String, GateParts> = from_str(&gp_f).unwrap();

	let rf_f = read_to_string(rf_path).unwrap();
	let rf: HashMap<String, NewResponseFunction> = from_str(&rf_f).unwrap();

	let mut gates: HashMap<String, Gate> = HashMap::new();
	for (key, p) in gp {
		let res_f = rf.get(&key).cloned().unwrap();

		let gate = Gate {
			name: p.name,
			parts: p.parts,
			promoter: p.promoter,
			equation: res_f.equation.to_owned(),
			params: res_f.params,
		};

		gates.insert(key, gate);
	}

	write(
		format!("{}/datasets/{}", dir.display(), "gates.json"),
		to_string(&gates).unwrap(),
	)
	.unwrap();
}

pub fn extract() {
	extract_parts();
	export_gate_types();
	extract_gate_parts();
	extract_response_functions();
	merge_gate_parts_and_rf();
}
