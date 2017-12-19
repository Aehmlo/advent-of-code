use Puzzle;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Solution { }

impl Solution { }

struct Trip {
	targets: HashMap<u32, Vec<u32>>,
	visited: HashSet<u32>
}

impl Trip {

	fn add_link(&mut self, s: &str) {
		let parts: Vec<&str> = s.split(" <-> ").collect();
		let targets = parts[1].split(", ").map(|x| x.parse().unwrap()).collect();
		self.targets.insert(parts[0].parse().unwrap(), targets);
	}

	fn visit(&mut self, destinations: Vec<u32>) {
		let targets = self.targets.clone();
		for destination in destinations {
			if !self.visited.contains(&destination) {
				let d = targets[&destination].clone();
				self.visited.insert(destination);
				self.visit(d);
			}
		}
	}

	fn get_group(&mut self, start_node: u32) {
		let targets = self.targets.clone();
		let d = targets[&start_node].clone();
		self.visit(d);
	}

	fn new() -> Self {
		Self {
			targets: HashMap::new(),
			visited: HashSet::new()
		}
	}

}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut trip = Trip::new();
		for line in lines {
			trip.add_link(line);
		}
		trip.get_group(0);
		let mut groups = 1;
		let targets = trip.targets.clone();
		for (target, _) in targets {
			if !trip.visited.contains(&target) {
				groups += 1;
				trip.get_group(target);
			}
		}
		return vec!(trip.visited.len() as u32, groups);
	}
	fn index() -> i8 {
		12
	}
}