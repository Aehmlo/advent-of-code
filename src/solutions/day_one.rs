use Puzzle;

pub struct Solution { }

impl Solution {
	fn score(input: &str, l: bool) -> u32 {
		let mut score = 0;
		let characters: Vec<char> = input.chars().collect();
		let offset = if l { characters.len() / 2 } else { 1 };
		for i in 0..characters.len() {
			let character = characters[i];
			if characters[(i + offset) % characters.len()] == character {
				score = score + character.to_digit(10).unwrap();
			}
		}
		return score;
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		return vec!(
			Solution::score(lines[0], true),
			Solution::score(lines[0], false)
		)
	}
	fn index() -> i8 {
		1
	}
}