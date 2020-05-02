use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::to_string;
use std::collections::HashMap;
use std::env;
use std::fs::write;

#[derive(Deserialize, Clone)]
pub struct Point {
	pub name: String,
	pub p: [f32; 2],
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LeafNode {
	pub name: String,
	pub parent: Option<String>,
	pub point: [f32; 2],
}

impl LeafNode {
	pub fn new(name: String, x: f32, y: f32) -> Self {
		Self {
			name: name,
			parent: None,
			point: [x, y],
		}
	}

	pub fn dist(&self, n: &LeafNode) -> f32 {
		((n.point[0] - self.point[0]).powi(2) + (n.point[1] - self.point[1]).powi(2)).sqrt()
	}

	pub fn contains(&self, n: &LeafNode) -> bool {
		n.point[0] > self.point[0] && n.point[1] < self.point[1]
	}
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Node {
	Leaf(LeafNode),
	Internal(InternalNode),
}

impl Node {
	fn is_leaf(&self) -> bool {
		matches!(*self, Node::Leaf(_))
	}

	fn is_internal(&self) -> bool {
		matches!(*self, Node::Internal(_))
	}

	fn leaf(&self) -> &LeafNode {
		match self {
			Node::Leaf(n) => n,
			_ => panic!("Node is not a leaf!"),
		}
	}

	fn internal(&self) -> &InternalNode {
		match self {
			Node::Internal(n) => n,
			_ => panic!("Node is not a internal!"),
		}
	}

	fn internal_mut(&mut self) -> &mut InternalNode {
		match self {
			Node::Internal(n) => n,
			_ => panic!("Node is not a internal!"),
		}
	}

	fn set_parent(&mut self, parent: Option<String>) {
		match self {
			Node::Leaf(n) => n.parent = parent,
			Node::Internal(n) => n.parent = parent,
		}
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
	tree: HashMap<String, Node>,
	root: Option<String>,
}

fn gen_key() -> String {
	rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(10)
		.collect::<String>()
}

fn get_closer<'a>(
	first: Option<&'a LeafNode>,
	second: Option<&'a LeafNode>,
	point: &LeafNode,
) -> Option<&'a LeafNode> {
	if first.is_none() {
		return second;
	}

	if second.is_none() {
		return first;
	}

	if first.unwrap().dist(point) < second.unwrap().dist(point) {
		return first;
	} else {
		return second;
	}
}

fn should_check_bad_side(
	closest: Option<&LeafNode>,
	node: &LeafNode,
	div: f32,
	axis: usize,
) -> bool {
	if closest.is_none() {
		return true;
	}
	(node.point[axis] - div).abs() < node.dist(closest.unwrap())
}

fn is_inside(point: &LeafNode, div: f32, axis: usize) -> bool {
	match axis {
		0 => point.point[axis] >= div,
		_ => point.point[axis] <= div,
	}
}

fn get_sides(in_node: &InternalNode, axis: usize) -> (String, String) {
	match axis {
		0 => (in_node.less.to_owned(), in_node.more.to_owned()),
		_ => (in_node.more.to_owned(), in_node.less.to_owned()),
	}
}

impl KdTree {
	pub fn new(k: u8) -> Self {
		Self {
			k: k,
			tree: HashMap::new(),
			root: None,
		}
	}

	fn insert(&mut self, arr: &mut Vec<Point>, parent: Option<String>, depth: u8) -> String {
		let n = arr.len();

		let axis = (depth % self.k) as usize;

		if n == 1 {
			let name = arr[0].name.to_owned();
			let mut node = LeafNode::new(name.to_owned(), arr[0].p[0], arr[0].p[1]);
			node.parent = parent;
			self.tree.insert(name.to_owned(), Node::Leaf(node));
			return name;
		}

		arr.sort_by(|a, b| a.p[axis].partial_cmp(&b.p[axis]).unwrap());

		let rand_key = gen_key();
		let div = (arr[(n / 2) - 1].p[axis] + arr[(n / 2)].p[axis]) / 2.0;
		let left_key = self.insert(
			&mut arr[0..(n / 2)].to_vec(),
			Some(rand_key.to_owned()),
			depth + 1,
		);
		let right_key = self.insert(
			&mut arr[(n / 2)..n].to_vec(),
			Some(rand_key.to_owned()),
			depth + 1,
		);

		self.tree.insert(
			rand_key.to_owned(),
			Node::Internal(InternalNode {
				div: div,
				less: left_key,
				more: right_key,
				parent: parent,
			}),
		);
		return rand_key;
	}

	pub fn build(&mut self, arr: &mut Vec<Point>) {
		let root_key = self.insert(arr, None, 0);
		self.root = Some(root_key);
	}

	fn walk(&self, point: &LeafNode, curr: String, depth: u8) -> Option<&LeafNode> {
		let node = self.tree.get(&curr).unwrap();

		if node.is_leaf() {
			if point.contains(node.leaf()) {
				return Some(node.leaf());
			} else {
				return None;
			}
		}

		let axis = (depth % self.k) as usize;

		let in_node = node.internal();
		let (good_side, bad_side) = get_sides(in_node, axis);
		if is_inside(point, node.internal().div, axis) {
			return self.walk(point, bad_side, depth + 1);
		}

		let mut closest = self.walk(point, good_side, depth + 1);
		if should_check_bad_side(closest, point, in_node.div, axis) {
			let closest_bad_side = self.walk(point, bad_side, depth + 1);
			closest = get_closer(closest, closest_bad_side, point);
		}
		return closest;
	}

	fn search(&self, x: f32, y: f32) -> Option<&LeafNode> {
		let closest = self.walk(
			&LeafNode::new("new".to_owned(), x, y),
			self.root.as_ref().unwrap().to_owned(),
			0,
		);
		closest
	}

	pub fn remove(&mut self, name: &str) {
		let node = self.tree.get(name).cloned().unwrap();
		let node = node.leaf();
		let node_parent_name = node.parent.as_ref();
		if node_parent_name.is_none() {
			self.root = None;
			self.tree.remove(name);
			return;
		}
		let node_parent_name = node_parent_name.unwrap();
		let parent = self.tree.get(node_parent_name).cloned().unwrap();
		let parent = parent.internal();
		let replacement = if name == parent.less {
			parent.more.to_owned()
		} else {
			parent.less.to_owned()
		};

		let replacement_node = self.tree.get_mut(&replacement).unwrap();
		let parent_parent_name = parent.parent.as_ref();
		if parent_parent_name.is_none() {
			self.root = Some(replacement);
			replacement_node.set_parent(None);
			self.tree.remove(name);
			self.tree.remove(node_parent_name);
			return;
		}

		let parent_parent_name = parent_parent_name.unwrap();
		replacement_node.set_parent(Some(parent_parent_name.to_owned()));
		let parent_parent = self.tree.get_mut(parent_parent_name).unwrap();
		let parent_parent = parent_parent.internal_mut();

		if node_parent_name == &parent_parent.less {
			parent_parent.less = replacement;
		} else {
			parent_parent.more = replacement;
		}

		self.tree.remove(name);
		self.tree.remove(node_parent_name);
	}

	pub fn save(&self, to: &str) {
		let dir = env::current_dir().unwrap();
		let s = to_string(self).unwrap();
		write(format!("{}/datasets/{}.json", dir.display(), to), s).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use crate::kd_tree::{KdTree, Point};
	#[test]
	fn should_create_kd_tree() {
		let mut kd = KdTree::new(2);

		let mut arr = vec![
			Point {
				name: "1".to_owned(),
				p: [4.0, 6.0],
			},
			Point {
				name: "2".to_owned(),
				p: [8.0, 4.0],
			},
			Point {
				name: "3".to_owned(),
				p: [10.0, 5.0],
			},
			Point {
				name: "4".to_owned(),
				p: [12.0, 8.0],
			},
		];

		kd.build(&mut arr);

		let root = kd.tree.get(&kd.root.unwrap()).unwrap().internal();
		let root_less = kd.tree.get(&root.less).unwrap().internal();
		let root_more = kd.tree.get(&root.more).unwrap().internal();

		let two = kd.tree.get(&root_less.less).unwrap().leaf();
		let one = kd.tree.get(&root_less.more).unwrap().leaf();

		let three = kd.tree.get(&root_more.less).unwrap().leaf();
		let four = kd.tree.get(&root_more.more).unwrap().leaf();

		assert_eq!(one.name, "1");
		assert_eq!(two.name, "2");
		assert_eq!(three.name, "3");
		assert_eq!(four.name, "4");
	}

	#[test]
	fn should_find_nearest() {
		let mut kd = KdTree::new(2);

		let mut arr = vec![
			Point {
				name: "1".to_owned(),
				p: [4.0, 6.0],
			},
			Point {
				name: "2".to_owned(),
				p: [8.0, 4.0],
			},
			Point {
				name: "3".to_owned(),
				p: [10.0, 5.0],
			},
			Point {
				name: "4".to_owned(),
				p: [12.0, 8.0],
			},
		];

		kd.build(&mut arr);

		let res = kd.search(2.0, 8.0);
		assert_eq!(res.unwrap().name, "1");
		let res = kd.search(7.0, 4.2);
		assert_eq!(res.unwrap().name, "2");
		let res = kd.search(9.0, 6.0);
		assert_eq!(res.unwrap().name, "3");
		let res = kd.search(11.0, 9.0);
		assert_eq!(res.unwrap().name, "4");
		let res = kd.search(7.0, 5.5);
		assert_eq!(res.unwrap().name, "2");
		let res = kd.search(7.9, 7.9);
		assert_eq!(res.unwrap().name, "3");
		let res = kd.search(9.8, 8.1);
		assert_eq!(res.unwrap().name, "4");
		let res = kd.search(10.0, 2.0);
		assert_eq!(res.is_none(), true);
	}

	#[test]
	fn should_remove_leaf_node() {
		let mut kd = KdTree::new(2);

		let mut arr = vec![
			Point {
				name: "1".to_owned(),
				p: [4.0, 6.0],
			},
			Point {
				name: "2".to_owned(),
				p: [8.0, 4.0],
			},
			Point {
				name: "3".to_owned(),
				p: [10.0, 5.0],
			},
			Point {
				name: "4".to_owned(),
				p: [12.0, 8.0],
			},
		];

		kd.build(&mut arr);

		kd.remove("2");

		let root = kd.tree.get(kd.root.as_ref().unwrap()).unwrap().internal();
		let root_more = kd.tree.get(&root.more).unwrap().internal();

		let one = kd.tree.get(&root.less).unwrap().leaf();
		let three = kd.tree.get(&root_more.less).unwrap().leaf();
		let four = kd.tree.get(&root_more.more).unwrap().leaf();

		assert_eq!(one.name, "1");
		assert_eq!(three.name, "3");
		assert_eq!(four.name, "4");

		kd.remove("3");

		let root = kd.tree.get(kd.root.as_ref().unwrap()).unwrap().internal();

		let one = kd.tree.get(&root.less).unwrap().leaf();
		let four = kd.tree.get(&root.more).unwrap().leaf();

		assert_eq!(one.name, "1");
		assert_eq!(four.name, "4");

		kd.remove("1");

		let root = kd.tree.get(kd.root.as_ref().unwrap()).unwrap().leaf();
		assert_eq!(root.name, "4");

		kd.remove("4");

		assert_eq!(kd.root.as_ref().is_none(), true);
		assert_eq!(kd.tree.len(), 0);
	}

	#[test]
	fn should_find_nearest_after_remove() {
		let mut kd = KdTree::new(2);

		let mut arr = vec![
			Point {
				name: "1".to_owned(),
				p: [4.0, 6.0],
			},
			Point {
				name: "2".to_owned(),
				p: [8.0, 4.0],
			},
			Point {
				name: "3".to_owned(),
				p: [10.0, 5.0],
			},
			Point {
				name: "4".to_owned(),
				p: [12.0, 8.0],
			},
		];

		kd.build(&mut arr);

		kd.remove("2");

		let res = kd.search(2.0, 8.0);
		assert_eq!(res.unwrap().name, "1");
		let res = kd.search(7.0, 4.2);
		assert_eq!(res.is_none(), true);
		let res = kd.search(9.0, 6.0);
		assert_eq!(res.unwrap().name, "3");
		let res = kd.search(11.0, 9.0);
		assert_eq!(res.unwrap().name, "4");
	}
}
