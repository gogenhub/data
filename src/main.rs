extern crate fs_extra;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod clean_up;
mod kd_tree;
mod node_linker;
mod tree_builder;

fn main() {
	tree_builder::build_trees();
}
