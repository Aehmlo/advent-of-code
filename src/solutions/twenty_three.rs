use Puzzle;
use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;
extern crate primal;

type RegisterIndex = char;

enum Value {
	Constant(isize),
	Register(RegisterIndex)
}

impl Value {
	fn eval(&self, processor: &Coprocessor) -> isize {
		match *self {
			Value::Constant(val) => val,
			Value::Register(register) => {
				processor.registers[&register]
			}
		}
	}
}

impl fmt::Debug for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Value::Constant(val) => write!(f, "{}", val),
			Value::Register(register) => write!(f, "{}", register)
		}
	}
}

impl FromStr for Value {

	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Value, Self::Err> {
		s.parse::<isize>().and_then(|value| {
			Ok(Value::Constant(value))
		}).or_else(|_| {
			let register = s.chars().nth(0).unwrap();
			Ok(Value::Register(register))
		})
	}

}

enum Instruction {
	Set(RegisterIndex, Value),
	Subtract(RegisterIndex, Value),
	Multiply(RegisterIndex, Value),
	MaybeJump(Value, Value)
}

impl fmt::Debug for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (name, a, b) = match *self {
			Instruction::Set(idx, ref val) => ("set", format!("{}", idx), val),
			Instruction::Subtract(idx, ref val) => ("sub", format!("{}", idx), val),
			Instruction::Multiply(idx, ref val) => ("mul", format!("{}", idx), val),
			Instruction::MaybeJump(ref v, ref val) => ("jnz", format!("{:?}", v), val)
		};
		write!(f, "{} {} {:?}", name, a, b)
	}
}

impl FromStr for Instruction {

	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Instruction, Self::Err> {
		let parts = s.split(" ").collect::<Vec<_>>();
		match parts[0] {
			"set" => {
				let register = parts[1].chars().nth(0).unwrap();
				let value = parts[2].parse::<Value>().unwrap();
				Ok(Instruction::Set(register, value))
			}, "sub" => {
				let register = parts[1].chars().nth(0).unwrap();
				let value = parts[2].parse::<Value>().unwrap();
				Ok(Instruction::Subtract(register, value))
			}, "mul" => {
				let register = parts[1].chars().nth(0).unwrap();
				let value = parts[2].parse::<Value>().unwrap();
				Ok(Instruction::Multiply(register, value))
			}, "jnz" => {
				Ok(Instruction::MaybeJump(parts[1].parse::<Value>().unwrap(), parts[2].parse::<Value>().unwrap()))
			}, i => panic!("Unrecognized instruction {}", i)
		}
	}

}

struct Coprocessor {
	registers: HashMap<RegisterIndex, isize>,
	position: usize,
	instructions: Vec<Instruction>,
	multiplications: usize
}

impl Coprocessor {

	fn new(instructions: Vec<Instruction>) -> Self {
		let mut registers = HashMap::new();
		for register in "abcdefgh".chars() {
			registers.insert(register, 0);
		}
		Self { registers, position: 0, instructions, multiplications: 0 }
	}

	fn execute_next(&mut self) {
		let instruction = self.instructions.get(self.position).expect("Ran out of instructions.");
		match *instruction {
			Instruction::Set(register, ref value) => { 
				let value = value.eval(&self);
				self.registers.insert(register, value);
				self.position += 1;
			}, Instruction::Subtract(register, ref value) => {
				let current = self.registers[&register];
				let value = current - value.eval(&self);
				self.registers.insert(register, value);	
				self.position += 1;
			}, Instruction::Multiply(register, ref value) => {
				let current = self.registers[&register];
				let value = current * value.eval(&self);
				self.registers.insert(register, value);
				self.multiplications += 1;
				self.position += 1;
			}, Instruction::MaybeJump(ref v, ref value) => {
				let v = v.eval(&self);
				if v != 0 {
					let value = value.eval(&self);
					if value > 0 {
						self.position += value as usize;
					} else {
						self.position -= value.abs() as usize;
					}
				} else {
					self.position += 1;
				}
			}
		};
	}

	fn run(&mut self) {
		while self.position < self.instructions.len() {
			self.execute_next();
		}
	}

}

impl fmt::Debug for Coprocessor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (registers, position) = (&self.registers, self.position);
		write!(f, "{:?} (p{})", registers, position)
	}
}

pub struct Solution { }

impl Solution {
}

fn part2(b: u64) -> u32 {
	let is_composite = |x| !primal::is_prime(x);
	(0..1001).filter(|x: &u64| is_composite(17 * x + 100_000 + 100 * b)).count() as u32
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let instructions = lines.iter().map(|l| l.parse::<Instruction>().unwrap()).collect();
		let mut processor = Coprocessor::new(instructions);
		processor.run();
		let multiplications = processor.multiplications as u32;
		let value = part2(lines[0].split(" ").collect::<Vec<_>>()[2].parse().unwrap());
		return vec!(multiplications, value);
	}
	fn index() -> i8 {
		23
	}
}