use Puzzle;
use std::collections::HashMap;

pub struct Solution { }

impl Solution { }

struct Firewall {
	scanners: HashMap<usize, Scanner>
}

impl Firewall {
	fn new() -> Self {
		Self {
			scanners: HashMap::new()
		}
	}
	fn add(&mut self, scanner: Scanner) {
		self.scanners.insert(scanner.depth, scanner);
	}
	fn navigate(&self, offset: usize) -> usize {
		let mut severity = 0;
		for s in 0..91 {
			if self.scanners.contains_key(&s) {
				let scanner = &self.scanners[&s];
				if scanner.will_collide(s + offset) {
					severity += scanner.severity();
				}
			}
		}
		severity
	}
}

struct Scanner {
	depth: usize,
	range: usize
}

impl Scanner {
	fn will_collide(&self, time: usize) -> bool {
		time % (2 * (self.range - 1)) == 0
	}
	fn severity(&self) -> usize {
		self.depth * self.range
	}
	fn from(s: &str) -> Self {
		let parts: Vec<usize> = s.split(": ").map(|x| x.parse().unwrap()).collect();
		let depth = parts[0];
		let range = parts[1];
		Self { depth, range }
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut firewall = Firewall::new();
		for line in lines.clone() {
			firewall.add(Scanner::from(line));
		}
		// After all this object-oriented stuff, let's fall back to functional programming. It's powerful.
		let mut solution = 0;
		while firewall.scanners.iter().any(|(layer, scanner)| (layer + solution) % ((scanner.range - 1) * 2) == 0) {
			solution += 1;
		}
		return vec!(firewall.navigate(0) as u32, solution as u32)
	}
	fn index() -> i8 {
		13
	}
}