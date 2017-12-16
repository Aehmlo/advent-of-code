use Puzzle;

pub struct Solution { }

impl Solution {
	fn line_digits(row: &str) -> Vec<u32> {
		row.split("\t").map(|x| x.parse().unwrap() ).collect()
	}
	fn difference(row: &str) -> u32 {
		let digits = Solution::line_digits(row);
		digits.iter().max().unwrap() - digits.iter().min().unwrap()
	}
	fn quotient(row: &str) -> u32 {
		let digits = Solution::line_digits(row);
		let len = digits.len();
		for i in 0..len {
			let digit = digits[i];
			for j in (i + 1)..len {
				if digit % digits[j] == 0 {
					return digit / digits[j]
				} else if digits[j] % digit == 0 {
					return digits[j] / digit
				}
			}
		}
		0
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut differences = 0;
		let mut quotients = 0;
		for line in lines {
			differences = differences + Solution::difference(line);
			quotients = quotients + Solution::quotient(line);
		}
		vec!(differences, quotients)
	}
	fn index() -> i8 {
		2
	}
}