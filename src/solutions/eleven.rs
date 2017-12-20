use Puzzle;
use std::cmp::max;

pub struct Solution { }

impl Solution {}

enum Direction {
	Northwest,
	Southwest,
	South,
	Southeast,
	Northeast,
	North
}

struct Translation {
	x: i32,
	y: i32,
	z: i32
}

impl Direction {
	fn transformation(self) -> Translation {
		let dir = match self {
			Direction::Northwest => (-1, 1, 0),
			Direction::Southwest => (-1, 0, 1),
			Direction::South => (0, -1, 1),
			Direction::Southeast => (1, -1, 0),
			Direction::Northeast => (1, 0, -1),
			_ => (0, 1, -1)
		};
		Translation {
			x: dir.0,
			y: dir.1,
			z: dir.2
		}
	}
	fn from(s: &str) -> Direction {
		match s {
			"nw" => Direction::Northwest,
			"ne" => Direction::Northeast,
			"sw" => Direction::Southwest,
			"se" => Direction::Southeast,
			"s" => Direction::South,
			_ => Direction::North
		}
	}
}

struct Position { // There are three "axes" we can move on, but they aren't all orthogonal.
	x: i32,
	y: i32,
	z: i32
}

impl Position {
	fn moved(&self, direction: Direction) -> Position {
		let translation = direction.transformation();
		Position {
			x: self.x + translation.x,
			y: self.y + translation.y,
			z: self.z + translation.z
		}
	}
	fn follow(path: Vec<Direction>) -> Vec<Position> {
		let mut followed = vec!(Position { x: 0, y: 0, z: 0});
		for direction in path {
			let len = followed.len();
			let pos = followed[len - 1].moved(direction);
			followed.push(pos);
		}
		followed
	}
	fn distance_from_origin(&self) -> u32 {
		max(max(self.x.abs(), self.y.abs()), self.z.abs()) as u32
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let directions = lines[0].split(",").map(|s| Direction::from(s)).collect();
		let path = Position::follow(directions);
		let mut distances: Vec<u32> = path.iter().map(|n| n.distance_from_origin()).collect();
		let mut max = 0;
		let last: u32 = distances.pop().unwrap();
		for distance in distances {
			if distance > max {
				max = distance;
			}
		}
		return vec!(last, max);
	}
	fn index() -> i8 {
		11
	}
}