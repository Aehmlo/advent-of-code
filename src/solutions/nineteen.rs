use Puzzle;

pub struct Solution { }

impl Solution { }

#[derive(Clone)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}

impl Direction {
	fn is_vertical(&self) -> bool {
		self == &Direction::Up || self == &Direction::Down
	}
}

impl PartialEq for Direction {
	fn eq(&self, rhs: &Direction) -> bool {
		match *self {
			Direction::Up => match *rhs {
				Direction::Up => true, _ => false
			},
			Direction::Down => match *rhs {
				Direction::Down => true, _ => false
			},
			Direction::Left => match *rhs {
				Direction::Left => true, _ => false
			},
			Direction::Right => match *rhs {
				Direction::Right => true, _ => false
			}
		}
	}
}

struct Track {
    grid: Vec<Vec<char>>
}

impl Track {
    fn new(lines: Vec<&str>) -> Self {
        Self {
            grid: lines.iter().map(|line| line.chars().collect()).collect()
        }
    }
}

type Position = (usize, usize);

struct Compass {
    position: Position,
    direction: Direction,
	distance_covered: usize
}

impl Compass {
	fn from(position: Position) -> Self {
		Self {
			position,
			direction: Direction::Down,
			distance_covered: 0
		}
	}
	fn mv(&mut self, direction: Direction) {
		self.distance_covered += 1;
		self.position = match direction {
			Direction::Up => (self.position.0, self.position.1 - 1),
			Direction::Down => (self.position.0, self.position.1 + 1),
			Direction::Left => (self.position.0 - 1, self.position.1),
			Direction::Right => (self.position.0 + 1, self.position.1)
		};
		self.direction = direction;
	}
}

struct Adventure {
	encountered: Vec<char>
}

impl Adventure {
	fn navigate(track: &Track, compass: &mut Compass) -> Self {
		let mut a = Self { encountered: vec!() };
		loop {
			let mut direction = compass.direction.clone();
			let current = track.grid[compass.position.1][compass.position.0];
			if current >= 'A' && current <= 'Z' { // Letter
				a.encountered.push(current);
			} else if current == ' ' { // Empty
				break;
			} else if current == '+' { // Intersection
				let position = compass.position;
				let d = compass.direction.clone();
				let _direction = match d.is_vertical() {
					true => {
						if position.0 > 0 && track.grid[position.1][position.0 - 1] == '-' { Direction::Left }
						else if position.0 < track.grid[position.1].len() - 1 && track.grid[position.1][position.0 + 1] == '-' { Direction::Right }
						else { panic!("Invalid direction at position {:?}", position); }
					}, false => {
						if position.1 > 0 && track.grid[position.1 - 1][position.0] == '|' { Direction::Up }
						else if position.1 < track.grid.len() - 1 && track.grid[position.1 + 1][position.0] == '|' { Direction::Down }
						else { panic!("Invalid direction at position {:?}", position); }
					}
				};
				direction = _direction;
			}
			compass.mv(direction);
		}
		a
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let top = lines[0].clone();
		let track = Track::new(lines);
		let x = top.chars().position(|s| s == '|').unwrap();
		let mut compass = Compass::from((x, 0));
		let adventure = Adventure::navigate(&track, &mut compass);
		let encountered = adventure.encountered.clone().iter().map(|c| *c).collect::<String>();
		println!("{}", encountered);
		return vec!(compass.distance_covered as u32);
	}
	fn index() -> i8 {
		19
	}
}