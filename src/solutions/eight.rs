use Puzzle;
use std::collections::HashMap;

pub struct Solution { }

impl Solution { }

struct CPU {
	registers: HashMap<String, i32>
}

fn get(map: &HashMap<String, i32>, name: &str) -> i32 {
	if map.contains_key(name) {
		return map[name];
	}
	0
}

impl CPU {
	fn new() -> Self {
		CPU {
			registers: HashMap::new()
		}
	}
	fn change(&mut self, name: String, amount: i32) -> i32 {
		let mut registers = &mut self.registers;
		let value = get(registers, &name);
		registers.insert(name, value + amount);
		value + amount
	}
	fn inc(&mut self, name: String, amount: i32) -> i32 {
		self.change(name, amount)
	}
	fn dec(&mut self, name: String, amount: i32) -> i32 {
		self.change(name, -amount)
	}
	fn run(&mut self, instruction: String) -> i32 {
		let parts: Vec<&str> = instruction.split(" ").collect();
		let (register, direction, amount, other, comparator, other_amount) = (parts[0], parts[1], parts[2], parts[4], parts[5], parts[6]);
		let amount: i32 = amount.parse().unwrap();
		let other_amount: i32 = other_amount.parse().unwrap();
		let other_current_value = get(&self.registers, other);
		let conditional =
			comparator == "==" && other_current_value == other_amount ||
			comparator == "<=" && other_current_value <= other_amount ||
			comparator == ">=" && other_current_value >= other_amount ||
			comparator == ">" && other_current_value > other_amount ||
			comparator == "<" && other_current_value < other_amount ||
			comparator == "!=" && other_current_value != other_amount;
		if conditional {
			if direction == "inc" {
				return self.inc(register.to_string(), amount);
			} else {
				return self.dec(register.to_string(), amount);
			}
		}
		get(&self.registers, register)
	}
	fn max(&self) -> i32 {
		let registers = &self.registers;
		let mut max = 0;
		for (_, value) in registers {
			if *value > max {
				max = *value;
			}
		}
		max
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut processor = CPU::new();
		let mut max = 0;
		for line in lines {
			let val = processor.run(line.to_string());
			if val > max { max = val; }
		}
		return vec!(processor.max() as u32, max as u32);
	}
	fn index() -> i8 {
		8
	}
}