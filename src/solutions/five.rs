use Puzzle;

pub struct Solution { }

impl Solution {
	fn doIt(mut jumps: Vec<i32>, decrement: bool) -> u32 {
		let mut i: i32 = 0;
		let mut iterations = 0;
		let lastIndex: i32 = jumps.len() as i32;
		while i < lastIndex {
			let offset = jumps[i as usize];
			if decrement && offset > 2 { jumps[i as usize] -= 1; }
			else { jumps[i as usize] += 1; }
			i += offset;
			iterations += 1;
		}
		iterations
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut jumps: Vec<i32> = lines.iter().map(|s| s.parse().unwrap()).collect();
		vec!(Solution::doIt(jumps.to_vec(), false), Solution::doIt(jumps.to_vec(), true))
	}
	fn index() -> i8 {
		5
	}
}