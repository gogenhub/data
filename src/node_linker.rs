use crate::kd_tree::LeafNode;
use std::collections::{HashMap, HashSet};
use std::f32::MAX;

pub fn init(tree: &mut HashMap<String, LeafNode>) {
	let name = "root".to_owned();
	let root = LeafNode::new(name.to_owned(), 0.0, MAX);
	tree.insert(name, root);
}

fn set_contains(tree: &HashMap<String, LeafNode>, arr: &HashSet<String>, node: &LeafNode) -> bool {
	for name in arr {
		let n = tree.get(name).unwrap();
		if n.contains(node) {
			return true;
		}
	}

	false
}

fn direct_child(tree: &HashMap<String, LeafNode>, parent: &LeafNode, child: &LeafNode) -> bool {
	if !parent.contains(child) {
		return false;
	}
	for ch in &parent.children {
		let ch_node = tree.get(ch).unwrap();

		if child.contains(ch_node) {
			return true;
		}

		if ch_node.contains(child) {
			return false;
		}

		if !ch_node.is_sibling(child) {
			return false;
		}
	}

	true
}

fn get_parent(
	tree: &HashMap<String, LeafNode>,
	pr_name: Option<String>,
	node: &LeafNode,
) -> Result<String, String> {
	let pr_node = tree.get(&pr_name.to_owned().unwrap()).unwrap();
	if pr_node.contains(node) {
		return Ok(pr_name.unwrap());
	}
	let mut closest = None;
	for pr in &pr_node.parents {
		let new_pr_node = tree.get(pr).unwrap();
		let closest_node = tree.get(pr);
		if new_pr_node.contains(node) {
			closest = Some(pr.to_owned());
			break;
		}

		if closest.is_none() {
			closest = Some(pr.to_owned());
		} else if node.dist(new_pr_node) < node.dist(&closest_node.unwrap()) {
			closest = Some(pr.to_owned());
		}
	}

	return get_parent(tree, closest, node);
}

fn walk(
	tree: &HashMap<String, LeafNode>,
	pr: &str,
	node: &LeafNode,
	children: &mut HashSet<String>,
	parents: &mut HashSet<String>,
) {
	let pr_node = tree.get(pr).unwrap();
	for ch in &pr_node.children {
		let ch_node = tree.get(ch).unwrap();
		if ch_node.contains(node) {
			walk(tree, ch, node, children, parents);
		}
	}

	if direct_child(tree, pr_node, node) {
		parents.insert(pr.to_owned());
	}

	for ch in &pr_node.children {
		let ch_node = tree.get(ch).unwrap();
		if node.contains(ch_node) && !set_contains(tree, &children, ch_node) {
			children.insert(ch.to_owned());
		}
	}

	for ch in &pr_node.children {
		let ch_node = tree.get(ch).unwrap();
		if ch_node.is_sibling(node) {
			walk(tree, ch, node, children, parents);
		}
	}
}

pub fn link(tree: &mut HashMap<String, LeafNode>, node: &mut LeafNode) {
	let mut parents = HashSet::new();
	let mut children = HashSet::new();
	walk(tree, "root", &node, &mut children, &mut parents);

	for pr in &parents {
		let pr_node = tree.get_mut(pr).unwrap();
		pr_node.children.insert(node.name.to_owned());
		node.parents.insert(pr.to_owned());
	}

	for ch in &children {
		let ch_node = tree.get_mut(ch).unwrap();
		ch_node.parents.insert(node.name.to_owned());
		node.children.insert(ch.to_owned());
	}

	for ch in children {
		let ch_node = tree.get_mut(&ch).unwrap();
		ch_node
			.parents
			.retain(|ch_pr| !node.parents.contains(ch_pr));
	}

	for pr in parents {
		let pr_node = tree.get_mut(&pr).unwrap();
		pr_node
			.children
			.retain(|pr_ch| !node.children.contains(pr_ch));
	}
}

pub fn unlink(tree: &mut HashMap<String, LeafNode>, name: String) {
	let node = tree.get(&name).cloned().unwrap();

	for pr in &node.parents {
		let pr_node = tree.get_mut(pr).unwrap();
		pr_node.children.remove(&name);
	}

	for ch in &node.children {
		let ch_node = tree.get_mut(ch).unwrap();
		ch_node.parents.remove(&name);
	}

	let mut direct_pairs = Vec::new();
	for pr in &node.parents {
		let pr_node = tree.get(pr).unwrap();
		for ch in &node.children {
			let ch_node = tree.get(ch).unwrap();
			if !set_contains(tree, &pr_node.children, ch_node) {
				direct_pairs.push((pr, ch));
			}
		}
	}

	for (pr, ch) in &direct_pairs {
		let pr_node = tree.get_mut(pr.to_owned()).unwrap();
		pr_node.children.insert(ch.to_owned().to_owned());
	}

	for (pr, ch) in direct_pairs {
		let ch_node = tree.get_mut(ch).unwrap();
		ch_node.parents.insert(pr.to_owned());
	}
}
