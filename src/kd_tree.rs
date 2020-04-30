use crate::node_linker;
use crate::tree_builder::ResponseFunction;
use serde_json::to_string;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::write;
use std::hash::Hasher;

#[derive(Deserialize, Serialize, Clone)]
pub struct LeafNode {
	pub name: String,
	pub parent: Option<String>,
	pub x: f32,
	pub y: f32,
	pub children: HashSet<String>,
	pub parents: HashSet<String>,
}

impl LeafNode {
	pub fn new(name: String, x: f32, y: f32) -> Self {
		Self {
			name: name,
			parent: None,
			x: x,
			y: y,
			children: HashSet::new(),
			parents: HashSet::new(),
		}
	}

	pub fn contains(&self, n: &LeafNode) -> bool {
		self.x < n.x && self.y > n.y
	}

	pub fn is_sibling(&self, n: &LeafNode) -> bool {
		(self.x < n.x && self.y < n.y) || (self.x > n.x && self.y > n.y)
	}

	pub fn dist(&self, n: &LeafNode) -> f32 {
		((n.x - self.x).powi(2) + (n.y - self.y).powi(2)).sqrt()
	}
}

#[derive(Deserialize, Serialize, Clone)]
struct InternalNode {
	parent: Option<String>,
	div: f32,
	less: String,
	more: String,
}

#[derive(Deserialize, Serialize)]
pub struct KdTree {
	k: u8,
	tree: HashMap<String, InternalNode>,
	leaf_nodes: HashMap<String, LeafNode>,
	root: Option<String>,
}

fn hash_keys(first: String, second: String) -> String {
	let mut hasher = DefaultHasher::new();
	hasher.write(first.as_bytes());
	hasher.write(second.as_bytes());

	format!("{:x}", hasher.finish())
}

impl KdTree {
	pub fn new(k: u8) -> Self {
		Self {
			k: k,
			tree: HashMap::new(),
			leaf_nodes: HashMap::new(),
			root: None,
		}
	}

	fn insert(&mut self, arr: &mut Vec<ResponseFunction>, depth: u8) -> String {
		let n = arr.len();

		let axis = depth % self.k;

		if n == 1 {
			let name = arr[0].name.to_owned();
			let mut node = LeafNode::new(name.to_owned(), arr[0].x, arr[0].y);
			node_linker::link(&mut self.leaf_nodes, &mut node);
			self.leaf_nodes.insert(name.to_owned(), node);
			return name;
		}

		match axis == 0 {
			true => arr.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap()),
			_ => arr.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap()),
		};

		let div = match axis == 0 {
			true => (arr[(n / 2) - 1].x + arr[(n / 2)].x) / 2.0,
			_ => (arr[(n / 2) - 1].y + arr[(n / 2)].y) / 2.0,
		};
		let left_key = self.insert(&mut arr[0..(n / 2)].to_vec(), depth + 1);
		let right_key = self.insert(&mut arr[(n / 2)..n].to_vec(), depth + 1);
		let hash_key = hash_keys(left_key.to_owned(), right_key.to_owned());

		let left_node = self.tree.get_mut(&left_key).unwrap();
		left_node.parent = Some(hash_key.to_owned());

		let right_node = self.tree.get_mut(&right_key).unwrap();
		right_node.parent = Some(hash_key.to_owned());

		self.tree.insert(
			hash_key.to_owned(),
			InternalNode {
				div: div,
				less: left_key,
				more: right_key,
				parent: None,
			},
		);
		return hash_key;
	}

	pub fn build(&mut self, arr: &mut Vec<ResponseFunction>) {
		let root_key = self.insert(arr, 0);
		self.root = Some(root_key);
	}

	fn walk(&self, key: &str, sx: f32, sy: f32, depth: u8) -> String {
		if !self.tree.contains_key(key) {
			return key.to_owned();
		}
		let axis = depth % self.k;
		let node = self.tree.get(key).unwrap();
		match axis {
			0 => match sx < node.div {
				true => self.walk(&node.less, sx, sy, depth + 1),
				_ => self.walk(&node.more, sx, sy, depth + 1),
			},
			_ => match sy < node.div {
				true => self.walk(&node.less, sx, sy, depth + 1),
				_ => self.walk(&node.more, sx, sy, depth + 1),
			},
		}
	}

	pub fn remove(&mut self, name: &str) {
		let node = self.leaf_nodes.get(name).cloned().unwrap();
		let node_parent_name = node.parent.as_ref().unwrap();
		let parent = self.tree.get(node_parent_name).cloned().unwrap();

		let replacement = if name == parent.less {
			parent.more.to_owned()
		} else {
			parent.less.to_owned()
		};

		let parent_parent_name = parent.parent.as_ref().unwrap();
		let parent_parent = self.tree.get_mut(parent_parent_name).unwrap();

		if node_parent_name == &parent_parent.less {
			parent_parent.less = replacement;
		} else {
			parent_parent.more = replacement;
		}

		node_linker::unlink(&mut self.leaf_nodes, name.to_owned());
		self.leaf_nodes.remove(name);
		self.tree.remove(node_parent_name);
	}

	fn search(&self, x: f32, y: f32) -> String {
		self.walk(self.root.as_ref().unwrap(), x, y, 0)
	}

	pub fn save(&self, to: &str) {
		let dir = env::current_dir().unwrap();
		let s = to_string(self).unwrap();
		write(format!("{}/datasets/{}.json", dir.display(), to), s).unwrap();
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn should_insert_item_into_tree() {
		// let kt = KdTree::new(2);

		// kt.insert(arr: &mut Vec<ResponseFunction>, depth: u8)
	}
}
