use Puzzle;
use std::collections::HashSet;

type Port = usize;
type Component = (Port, Port);
type Strength = usize;
type Length = usize;

pub struct Solution { }

impl Solution {
	fn extend(parts: Vec<Component>, used: &HashSet<usize>, last: Port, target_longest: bool) -> (Length, Strength) {
		if parts.len() == used.len() { return (0, 0); }
		let mut _used = used.clone();
		parts.clone().iter().enumerate().filter(|&(index, part)| (part.0 == last || part.1 == last) && !used.contains(&index)).map(|(index, part)| {
			_used.insert(index);
			let (length, strength) = Solution::extend(parts.clone(), &_used, part.0 + part.1 - last, target_longest);
			_used.remove(&index);
			(length + target_longest as usize, strength + part.0 + part.1)
		}).max().unwrap_or((0, 0))
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let parts = lines.iter().map(|line| line.split("/")).map(|mut split| (split.nth(0).unwrap().parse::<usize>().unwrap(), split.nth(0).unwrap().parse::<usize>().unwrap())).collect::<Vec<_>>();
		let (one, two) = (Solution::extend(parts.clone(), &HashSet::new(), 0, false).1,  Solution::extend(parts, &HashSet::new(), 0, true).1);
		return vec!(one as u32, two as u32);
	}
	fn index() -> i8 {
		24
	}
}