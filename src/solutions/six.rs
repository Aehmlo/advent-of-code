use Puzzle;
use std::collections::HashMap;

pub struct Solution { }

const SIZE: usize = 16;

#[derive(Copy)]
#[derive(Clone)]
struct Bank {
	blocks: u32
}

struct Debugger {
	banks: [Bank; SIZE]
}

impl Debugger {

	fn from_string(string: &String) -> Self {
		let mut db = Self {
			banks: [Bank { blocks: 0 }; SIZE]
		};
		let blocks: Vec<u32> = string.split("\t").map(|s| s.parse().unwrap()).collect();
		for i in 0..SIZE {
			db.banks[i].blocks = blocks[i];
		}
		db
	}

	fn to_string(&self) -> String {
		self.banks.iter().map(|b| b.blocks.to_string()).collect::<Vec<String>>().join("\t")
	}

	fn redistribute(&mut self) -> String {
		let mut max_blocks = 0;
		let mut bank = 0;
		for i in 0..SIZE {
			if self.banks[i as usize].blocks > max_blocks {
				max_blocks = self.banks[i as usize].blocks;
				bank = i;
			}
		}
		self.banks[bank].blocks = 0;
		for i in 0..max_blocks {
			self.banks[(1 + bank + i as usize) % SIZE].blocks += 1;
		}
		self.to_string()
	}

}

impl Solution { }

impl Puzzle for Solution {

	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut store = HashMap::new();
		let mut state = String::from(lines[0]);
		let mut db = Debugger::from_string(&state);
		let mut count = 0;
		while !store.contains_key(&state) {
			store.insert(state, count);
			state = db.redistribute();
			count += 1;
		}
		return vec!(count, count - store[&state])
	}

	fn index() -> i8 {
		6
	}
}