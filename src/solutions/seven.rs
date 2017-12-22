use Puzzle;
use std::collections::{HashSet, HashMap};

pub struct Solution { }

impl Solution { }

#[derive(Clone)]
struct Node {
	name: String,
	weight: usize,
	children: Vec<Node>,
	calculated_weight: usize
}

type NodeInfo = (String, usize, Vec<String>);

impl Node {

	fn new(head: String, nodes: &HashMap<String, NodeInfo>) -> Self {
		let tuple = nodes.get(&head).unwrap();
		let children: Vec<Node> = tuple.2.iter().map(|c| Self::new(c.clone(), nodes)).collect();
		let calculated_weight: usize = children.iter().fold(tuple.1, |sum, c| sum + c.calculated_weight);
		Self {
			name: tuple.0.clone(),
			weight: tuple.1,
			children,
			calculated_weight
		}
	}

	fn parse(line: String) -> NodeInfo {
		let mut iter = line.split_whitespace();
		let name = iter.next().unwrap();
		let size = iter.next().map(|s| usize::from_str_radix(s.trim_matches(|c: char| !c.is_numeric()), 10).unwrap()).unwrap();
		let _ = iter.next();
		let children = iter.map(|x| x.trim_matches(',').into()).collect();
		(name.into(), size, children)
	}

}

struct Tree { }

impl Tree {

	fn new(items: Vec<String>) -> Node {
		let nodes = items.clone().iter().map(|line| {
			let item = Node::parse(line.clone());
			(item.0.clone(),item)
		}).collect::<HashMap<_, _>>();
		let head = Tree::find_root(items.clone());
		Node::new(head, &nodes)
	}

	fn find_root(items: Vec<String>) -> String {

		let mut children = HashSet::new();
		let mut nodes = HashSet::new();

		for item in items {
			let (name, _, cs) = Node::parse(item);
			nodes.insert(name);
			for c in cs {
				children.insert(c.clone());
			}
		}

		nodes.difference(&children).next().unwrap().clone()

	}

	fn corrected_weight(node: Node) -> Option<usize> {

		let mut weights = HashMap::new();

		for child in node.children.clone() {
			if let Some(previous) = weights.insert(child.calculated_weight, 1) {
				weights.insert(child.calculated_weight, previous + 1);
			}
		}

		if weights.len() > 1 { // There are multiple weights at this level, so the error is here.
			let bad_child = node.children.iter().find(|c| weights.get(&c.calculated_weight) == Some(&1)).map(|c| c.clone()).unwrap();
			let bad_weight = bad_child.calculated_weight;
			for child in bad_child.children {
				if let Some(result) = Self::corrected_weight(child.clone()) {
					return Some(result);
				}
			}
			// This node is the problem.
			let sibling_weight = *weights.keys().find(|w| **w != bad_weight).unwrap();
			if bad_weight > sibling_weight { // Avoid underflow
				Some(bad_child.weight - (bad_weight - sibling_weight))
			} else {
				Some(bad_child.weight + (sibling_weight - bad_weight))
			}
		} else {
			None
		}

	}

}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		println!("{}", Tree::find_root(lines.iter().map(|s| s.to_string()).collect()));
		let tree = Tree::new(lines.iter().map(|s| s.to_string()).collect());
		vec!(Tree::corrected_weight(tree).unwrap() as u32)
	}
	fn index() -> i8 {
		7
	}
}