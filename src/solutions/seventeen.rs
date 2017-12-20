use Puzzle;

pub struct Solution { }

impl Solution { }

struct SpinlockBuffer {
	buffer: Vec<usize>,
	position: usize,
	step: usize
}

impl SpinlockBuffer {
	fn new(step: usize) -> Self {
		Self {
			buffer: vec!(0),
			position: 0,
			step
		}
	}
	fn insert(&mut self, n: usize) -> usize {
		let buffer = &mut self.buffer;
		let position = (self.position + self.step) % buffer.len() + 1;
		buffer.insert(position, n);
		self.position = position;
		let i = (position + 1) % buffer.len();
		buffer[i]
	}
	#[allow(dead_code)]
	fn print(&self) {
		let len = self.buffer.len() - 1;
		for i in 0..len - 1 {
			print!("{} ", self.buffer[i]);
		}
		println!("{}", self.buffer[len]);
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let step = lines[0].parse().unwrap();
		let mut buffer = SpinlockBuffer::new(step);
		for item in 1..2017 {
			buffer.insert(item);
		}
		let next = buffer.insert(2017);
		// I haven't come up with a better way to do this quickly that's true to the object-oriented approach, so we're back to functional programming. Again.
		let (_, after_zero) = (1..50_000_001).fold((0, 0), |a, b| {
			let next = (a.0 + step) % b;
			(next + 1, if next == 0 { b } else { a.1 })
		});
		return vec!(next as u32, after_zero as u32);
	}
	fn index() -> i8 {
		17
	}
}