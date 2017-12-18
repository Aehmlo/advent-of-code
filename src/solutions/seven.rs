use Puzzle;
use std::collections::HashMap;

pub struct Solution { }

impl Solution { }

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut children = HashMap::new();
		let mut all_nodes = HashMap::new();
		for line in lines {
			let split: Vec<&str> = line.split("-> ").collect();
			if split.len() > 1 {
				let list = split[1];
				let nodes: Vec<&str> = list.split(", ").collect();
				for child in nodes {
					children.insert(child, true);
					all_nodes.insert(child, true);
				}
			}
			let new_split: Vec<&str> = split[0].split(" (").collect();
			all_nodes.insert(new_split[0], true);
		}
		let nodes: Vec<&&str> = all_nodes.keys().collect();
		let non_root: Vec<&&str> = children.keys().collect();
		let mut root: Option<&str> = None;
		for node in nodes {
			if non_root.iter().find(|n| **n == node) == None {
				root = Some(node);
				break;
			}
		}
		println!("{}", root.unwrap());
		vec!()
	}
	fn index() -> i8 {
		7
	}
}