use Puzzle;
use std::collections::HashMap;

type Value = bool;
type Delta = bool;
type Ruleset = (Value, Delta, State);

pub struct Solution {
	position: usize,
	values: Vec<Value>,
	rules: HashMap<State, (Ruleset, Ruleset)>
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum State {
	A, B, C, D, E, F
}

impl State {
	fn from(c: char) -> Self {
		match c {
			'a' => State::A,
			'b' => State::B,
			'c' => State::C,
			'd' => State::D,
			'e' => State::E,
			'f' => State::F,
			_ => unreachable!()
		}
	}
}

impl Solution {

	fn mv(&mut self, right: bool) {
		if right {
			self.position += 1;
			if self.position > self.values.len() - 1 {
				self.values.push(false);
			}
		} else {
			if self.position == 0 {
				self.values.insert(0, false);
			} else {
				self.position -= 1;
			}
		}
	}

	fn next_from(&mut self, state: State) -> State {
		let (ref a, ref b) = self.rules.clone()[&state];
		let (val, delta, s) = if self.values[self.position] { a.clone() } else { b.clone() };
		self.values[self.position] = val;
		self.mv(delta);
		s
	}

	fn run(&mut self, iterations: usize) -> usize {
		let mut state = State::A;
		for _ in 0..iterations {
			state = self.next_from(state);
		}
		self.checksum()
	}

	fn checksum(&self) -> usize {
		self.values.iter().map(|v| *v as usize).sum()
	}

	fn new(lines: Vec<&str>) -> Self {
		let mut rules = HashMap::new();
		let states = "abcdef".chars().enumerate();
		for (index, state) in states {
			let lower = 3 + 10 * index;
			let upper = lower + 9;
			let block = &lines[(lower + 1)..upper];
			let rulesets = Solution::rulesets(block);
			rules.insert(State::from(state), rulesets);
		}
		Self {
			position: 0,
			values: vec!(false),
			rules
		}
	}

	fn ruleset_from_block(block: &[&str]) -> (Value, Delta, State) {
		let lines = block;
		let (a, b, c) = (lines[0].split(" ").last().unwrap(), lines[1].split(" ").last().unwrap(), lines[2].split(" ").last().unwrap());
		let one = a == "1.";
		let two = b == "right.";
		let three = match c.trim() {
				"A." => State::A,
				"B." => State::B,
				"C." => State::C,
				"D." => State::D,
				"E." => State::E,
				"F." => State::F,
				_ => unreachable!()	
		};
		(one, two, three)
	}
	
	fn rulesets(block: &[&str]) -> (Ruleset, Ruleset) {
		let (two, one) = (&block[1..4], &block[5..8]);
		let (two, one) = (Solution::ruleset_from_block(two), Solution::ruleset_from_block(one));
		(one, two)
	}

}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let iterations = lines[1].split(" ").nth(5).unwrap().parse::<usize>().unwrap();
		let mut solution = Solution::new(lines);
		let sol = solution.run(iterations);
		return vec!(sol as u32);
	}
	fn index() -> i8 {
		25
	}
}