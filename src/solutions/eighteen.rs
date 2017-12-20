use Puzzle;
use std::collections::VecDeque; // For some reason, just using Vec isn't sufficient and results in an infinite loop. TODO: Understand why.
use std::collections::HashMap;
use std::str::FromStr;

pub struct Solution { }

impl Solution { }

type RegisterIndex = char;

#[derive(Clone)]
enum Input {
	Register(RegisterIndex),
	Constant(isize)
}

impl FromStr for Input {

	type Err = String;

	fn from_str(s: &str) -> Result<Input, String> {
		if s.len() == 1 && s.as_bytes()[0] >= 65 { // 65 = 'a'; it's a real character that refers to a register
			Ok(Input::Register(s.chars().nth(0).unwrap()))
		} else {
			s.parse().map(Input::Constant).map_err(|err| format!("Failed to parse value {}: {}", s, err))
		}
	}

}

impl Input {

	fn value_on(&self, cpu: &CPU) -> isize {
		match *self {
			Input::Constant(val) => val,
			Input::Register(name) => cpu.registers[&name]
		}
	}

}

#[derive(Clone)]
enum Operation {
	Send(Input),
	Set(RegisterIndex, Input),
	Add(RegisterIndex, Input),
	Multiply(RegisterIndex, Input),
	Modulo(RegisterIndex, Input),
	Receive(RegisterIndex),
	MaybeJump(Input, Input)
}

impl FromStr for Operation {

	type Err = String;

	fn from_str(s: &str) -> Result<Operation, Self::Err> {
		let parts: Vec<&str> = s.split_whitespace().collect();
		Ok(match parts[0] {
			"snd" => Operation::Send(parts[1].parse().unwrap()),
			"set" => Operation::Set(parts[1].chars().nth(0).unwrap(), parts[2].parse().unwrap()),
			"add" => Operation::Add(parts[1].chars().nth(0).unwrap(), parts[2].parse().unwrap()),
			"mul" => Operation::Multiply(parts[1].chars().nth(0).unwrap(), parts[2].parse().unwrap()),
			"mod" => Operation::Modulo(parts[1].chars().nth(0).unwrap(), parts[2].parse().unwrap()),
			"rcv" => Operation::Receive(parts[1].chars().nth(0).unwrap()),
			"jgz" => Operation::MaybeJump(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
			_ => {
				return Err(format!("Unknown instruction {} (must be one of snd, set, add, mul, mod, rcv, jgz)", parts[0]));
			}
		})
	}

}

struct CPU {
	registers: HashMap<char, isize>,
	memory: Vec<Operation>,
	position: usize,
	queue: VecDeque<isize>,
	sent: usize,
	last: isize,
	first_nonzero_recv: isize
}

impl CPU {

	fn new(memory: Vec<Operation>, id: isize) -> Self {
		let mut it = Self {
			registers: HashMap::new(),
			memory,
			position: 0,
			queue: VecDeque::new(),
			sent: 0,
			last: 0,
			first_nonzero_recv: 0
		};
		for key in "abcdefghijklmnopqrstuvwxyz".chars() {
			it.registers.insert(key, if key == 'p' { id } else { 0 });
		}
		it
	}

	fn run(&mut self, other: &mut Self, primary: bool) {
		while self.position < self.memory.len() {
			let go = self.memory[self.position].clone().execute_on(self, other);
			if !go {
				if !primary { break; }
				other.run(self, false);
				if !self.memory[self.position].clone().execute_on(self, other) {
					break; // We let the other one run and this still isn't working; we've reached an impasse.
				}
			}
		}
	}

	fn parallelize(one: &mut Self, two: &mut Self) {
		one.run(two, true);
	}

}

impl Operation {

	fn execute_on(&self, primary: &mut CPU, secondary: &mut CPU) -> bool { // Inspired heavily by https://gist.github.com/samueltardieu/9f4fee9b4bf99c0987941fd8f300d974
		use solutions::eighteen::Operation::*;
		primary.position += 1;
		let registers = primary.registers.clone();
		match *self {
			Send(ref v) => {
				let v = v.value_on(primary);
				primary.last = v;
				secondary.queue.push_back(v);
				primary.sent += 1;
			}
			Set(ref r, ref v) => {
				let value = v.value_on(primary);
				primary.registers.insert(*r, value);
			}
			Add(ref r, ref v) => {
				let current = registers[r];
				let value = v.value_on(primary);
				primary.registers.insert(*r, current + value);
			}
			Multiply(ref r, ref v) => {
				let current = registers[r];
				let value = v.value_on(primary);
				primary.registers.insert(*r, current * value);
			}
			Modulo(ref r, ref v) => {
				let current = registers[r];
				let value = v.value_on(primary);
				primary.registers.insert(*r, current % value);
				assert!(primary.registers[r] >= 0);
			}
			Receive(ref r) => {
				if primary.first_nonzero_recv == 0 && primary.registers[r] != 0 {
					primary.first_nonzero_recv = primary.last;
				}
				if let Some(v) = primary.queue.pop_front() {
					primary.registers.insert(*r, v);
				} else {
					primary.position -= 1;
					return false;
				}
			}
			MaybeJump(ref t, ref o) => {
				if t.value_on(primary) > 0 {
					let delta = o.value_on(primary) - 1;
					let subtract = delta < 0;
					if subtract {
						primary.position -= delta.abs() as usize;
					} else {
						primary.position += delta.abs() as usize;
					}
				}
			}
		}
		true
	}

}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let memory = lines.iter().map(|o| o.parse().unwrap()).collect::<Vec<Operation>>();
		let mut one = CPU::new(memory.clone(), 0);
		let mut two = CPU::new(memory, 1);
		CPU::parallelize(&mut one, &mut two);
		return vec!(one.first_nonzero_recv as u32, two.sent as u32);
	}
	fn index() -> i8 {
		18
	}
}