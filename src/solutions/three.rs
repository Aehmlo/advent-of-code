use Puzzle;
use std::collections::HashMap;

enum Direction {
	Up,
	Down,
	Left,
	Right
}

impl Direction {

	fn next(&self) -> Self {
		match *self {
			Direction::Right => Direction::Up,
			Direction::Up => Direction::Left,
			Direction::Left => Direction::Down,
			_ => Direction::Right
		}
	}

	fn delta(&self) -> (i32, i32) {
		match *self {
			Direction::Right => (1, 0),
			Direction::Up => (0, 1),
			Direction::Left => (-1, 0),
			_ => (0, -1)
		}
	}

}

pub struct Solution { }

impl Solution {
	fn find_center(position: u32) -> u32 {
		let mut relevant_factor = (position as f64).sqrt().ceil() as u32;
		if relevant_factor % 2 == 0 { relevant_factor += 1; }
		let mut closest_corner = 0;
		let mut corner_distance = relevant_factor.pow(2);
		for i in 0..3 {
			let corner = relevant_factor.pow(2) - i * (relevant_factor - 1);
			let distance = corner - position;
			if distance < corner_distance {
				corner_distance = distance;
				closest_corner = corner;
			}
		}
		relevant_factor - (closest_corner - position) - 1
	}
	fn find_greater(input: u32) -> u32 {
		let mut grid = HashMap::new();
		grid.insert((0, 0), 1);
		let mut position = (0, 0);
		let mut semiaxis = 1;
		let mut double = false;
		let mut direction = Direction::Right;
		let mut directional_iterations = 0;
		loop {
			let mut delta = direction.delta();
			position.0 += delta.0;
			position.1 += delta.1;
			let mut sum = 0;
			for i in 0..8 {
				delta = match i {
					0 => (1, 0),
					1 => (1, 1),
					2 => (0, 1),
					3 => (-1, 1),
					4 => (-1, 0),
					5 => (-1, -1),
					6 => (0, -1),
					_ => (1, -1)
				};
				if grid.contains_key(&(position.0 + delta.0, position.1 + delta.1)) {
					sum += grid[&(position.0 + delta.0, position.1 + delta.1)];
				}
			}
			if sum > input {
				return sum;
			}
			grid.insert(position, sum);
			directional_iterations += 1;
			if directional_iterations == semiaxis {
				directional_iterations = 0;
				direction = direction.next();
				if double {
					double = false;
					semiaxis += 1;
				} else {
					double = true;
				}
			}
		}
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let input: Vec<u32> = lines.iter().map(|x| x.parse().unwrap()).collect();
		vec!(Self::find_center(input[0]), Self::find_greater(input[0]))
	}
	fn index() -> i8 {
		3
	}
}