use Puzzle;

pub struct Solution { }

impl Solution { }

struct Generator {
	multiplier: usize,
	divisor: usize,
	seed: usize,
	previous: Option<usize>
}

impl Generator {
	fn new(seed: usize, multiplier: usize) -> Self {
		Self {
			seed,
			multiplier,
			divisor: 2147483647,
			previous: None
		}
	}
}

impl Iterator for Generator {

	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(old) = self.previous {
			let value = (old * self.multiplier) % self.divisor;
			self.previous = Some(value);
			return Some(value);
		} else {
			let value = (self.seed * self.multiplier) % self.divisor;
			self.previous = Some(value);
			return Some(value);
		}
	}

}

trait Judgeable {
	fn judge(self) -> bool;
}

impl Judgeable for (usize, usize) {
	fn judge(self) -> bool {
		let mask = 0b00000000000000001111111111111111;
		(self.0 & mask) == (self.1 & mask)
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let seeds = lines.iter().map(|line| line.split(" ").last().unwrap());
		let seeds: Vec<usize> = seeds.map(|s| s.parse().unwrap()).collect();
		let mut one = Generator::new(seeds[0], 16807);
		let mut two = Generator::new(seeds[1], 48271);
		let mut sum = 0;
		for _ in 0..40_000_000 {
			let pair = (one.next().unwrap(), two.next().unwrap());
			if pair.judge() {
				sum += 1;
			}
		}
		one = Generator::new(seeds[0], 16807);
		two = Generator::new(seeds[1], 48271);
		let mut s = 0;
		for _ in 0..5_000_000 {
			let mut o = one.next().unwrap();
			while o % 4 != 0 {
				o = one.next().unwrap();
			}
			let mut t = two.next().unwrap();
			while t % 8 != 0 {
				t = two.next().unwrap();
			}
			let pair = (o, t);
			if pair.judge() {
				s += 1;
			}
		}
		return vec!(sum, s);
	}
	fn index() -> i8 {
		15
	}
}