use fs_extra::file::read_to_string;
use regex::Regex;
use serde_json::{from_str, from_value, to_string, Value};
use std::collections::HashMap;
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

#[derive(Deserialize, Serialize)]
struct Param {
	name: String,
	value: f32,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseFunction {
	#[serde(alias = "gate_name")]
	name: String,
	equation: String,
	#[serde(alias = "parameters")]
	params: Vec<Param>,
}

#[derive(Clone)]
pub struct FunctionProperties {
	pub y: f32,
	pub x: f32,
	pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct GateParts {
	name: String,
	parts: Vec<String>,
	promoter: String,
}

fn extract_parts() {
	let dir = env::current_dir().unwrap();

	let mut parts: HashMap<String, Part> = HashMap::new();
	for entry in read_dir(format!("{}/datasets/raw/", dir.display())).unwrap() {
		let f = read_to_string(entry.unwrap().path()).unwrap();
		let json: Vec<Value> = from_str(&f).unwrap();
		for item in json {
			if item["collection"] == "parts" && item["kind"] != "SgRNA" {
				let part: Part = from_value(item).unwrap();
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

fn extract_gates() {
	let dir = env::current_dir().unwrap();

	let mut gates_map: HashMap<String, Vec<BGate>> = HashMap::new();
	for entry in read_dir(format!("{}/datasets/raw/", dir.display())).unwrap() {
		let f = read_to_string(entry.unwrap().path()).unwrap();
		let json: Vec<Value> = from_str(&f).unwrap();
		for item in json {
			if item["collection"] == "gates"
				&& (item["system"] == "TetR" || item["system"] == "sensor")
			{
				let gate: BGate = from_value(item).unwrap();
				let key = format!("{:?}", gate.kind);
				if !gates_map.contains_key(&key) {
					gates_map.insert(key.to_owned(), Vec::new());
				}
				let gs = gates_map.get_mut(&key).unwrap();
				gs.push(gate);
			}
		}
	}

	write(
		format!("{}/datasets/{}", dir.display(), "gates.json"),
		to_string(&gates_map).unwrap(),
	)
	.unwrap();
}

fn extract_gate_parts() {
	let dir = env::current_dir().unwrap();

	let mut gate_parts_arr: HashMap<String, GateParts> = HashMap::new();
	for entry in read_dir(format!("{}/datasets/raw/", dir.display())).unwrap() {
		let f = read_to_string(entry.unwrap().path()).unwrap();
		let json: Vec<Value> = from_str(&f).unwrap();
		for item in json {
			if item["collection"] == "gate_parts"
				&& item["gate_name"] != "SicA_InvF"
				&& item["gate_name"] != "ExsC_ExsDA"
			{
				let gate_parts: OldGateParts = from_value(item).unwrap();
				let new_gate_parts = GateParts {
					name: gate_parts.name.clone(),
					parts: gate_parts.cassettes[0].parts.clone(),
					promoter: gate_parts.promoter,
				};
				gate_parts_arr.insert(gate_parts.name, new_gate_parts);
			}
		}
	}

	write(
		format!("{}/datasets/{}", dir.display(), "gate_parts.json"),
		to_string(&gate_parts_arr).unwrap(),
	)
	.unwrap();
}

fn extract_response_functions() {
	let dir = env::current_dir().unwrap();

	let mut response_functions: HashMap<String, ResponseFunction> = HashMap::new();
	for entry in read_dir(format!("{}/datasets/raw/", dir.display())).unwrap() {
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
					response_functions.insert(response_function.name.to_owned(), response_function);
				}
			}
		}
	}

	write(
		format!("{}/datasets/{}", dir.display(), "response_functions.json"),
		to_string(&response_functions).unwrap(),
	)
	.unwrap();
}

fn make_new_gates() {
	let dir = env::current_dir().unwrap();
	let gates_path = format!("{}/datasets/{}", dir.display(), "gates.json");
	let gate_parts_path = format!("{}/datasets/{}", dir.display(), "gate_parts.json");

	let gates_f = read_to_string(gates_path).unwrap();
	let gates_parts_f = read_to_string(gate_parts_path).unwrap();

	let gates: HashMap<String, Vec<BGate>> = serde_json::from_str(&gates_f).unwrap();
	let gates_parts: HashMap<String, GateParts> = serde_json::from_str(&gates_parts_f).unwrap();

	let mut new_gate_parts = HashMap::new();
	for (name, gs) in gates {
		let mut vc = Vec::new();
		for g in gs {
			vc.push(gates_parts.get(&g.name).unwrap());
		}
		new_gate_parts.insert(name, vc);
	}

	write(
		format!("{}/datasets/{}", dir.display(), "gate_parts.json"),
		to_string(&new_gate_parts).unwrap(),
	)
	.unwrap();
}

pub fn extract() {
	extract_parts();
	extract_gates();
	extract_gate_parts();
	extract_response_functions();
	make_new_gates();
}
