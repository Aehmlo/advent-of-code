use Puzzle;

pub struct Solution { }

impl Solution {
	fn validate(line: &str, flag: bool) -> bool {
		let mut words = line.split(" ").collect::<Vec<&str>>();
		if flag {
			let len = words.len();
			for i in 0..len {
				for j in (i + 1)..len {
					let mut new: Vec<&str> = words[j].split("").collect();
					new.sort();
					let mut curr: Vec<&str> = words[i].split("").collect();
					curr.sort();
					if new == curr {
						return false;
					}
				}
			}
			return true;
		} else {
			let word_count = words.len();
			words.sort();
			words.dedup();
			let count = words.len();
			return count == word_count;
		}
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut counter1 = 0;
		let mut counter2 = 0;
		for line in lines {
			if Solution::validate(line, false) {
				counter1 = counter1 + 1;
			}
			if Solution::validate(line, true) {
				counter2 = counter2 + 1;
			}
		}
		vec!(counter1, counter2)
	}
	fn index() -> i8 {
		4
	}
}