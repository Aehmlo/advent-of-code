use Puzzle;
use std::fmt;

pub struct Solution { }

impl Solution {
}

#[derive(Clone, Debug)]
enum Direction {
	Up, Right, Down, Left
}

impl Direction {
	fn delta(self) -> (isize, isize) {
		match self {
			Direction::Up => (0, 1),
			Direction::Down => (0, -1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0)
		}
	}

	fn turn_left(self) -> Self {
		match self {
			Direction::Up => Direction::Left,
			Direction::Left => Direction::Down,
			Direction::Down => Direction::Right,
			Direction::Right => Direction::Up
		}
	}

	fn turn_right(self) -> Self {
		match self {
			Direction::Up => Direction::Right,
			Direction::Left => Direction::Up,
			Direction::Down => Direction::Left,
			Direction::Right => Direction::Down
		}
	}

	fn turn_around(self) -> Self {
		match self {
			Direction::Up => Direction::Down,
			Direction::Left => Direction::Right,
			Direction::Down => Direction::Up,
			Direction::Right => Direction::Left
		}
	}

}

struct Sporifica {
	position: (isize, isize),
	direction: Direction,
	infections_caused: usize,
	uninfections_caused: usize,
	evolved: bool
}

impl Sporifica {
	fn new(evolved: bool) -> Self {
		Self {
			position: (0, 0),
			direction: Direction::Up,
			infections_caused: 0,
			uninfections_caused: 0,
			evolved
		}
	}

	fn tick(&mut self, grid: &mut Grid) {
		let direction = self.direction.clone();
		if grid.is_infected(self.position) {
			self.direction = direction.turn_right();
		} else {
			if self.evolved {
				self.direction = match grid.state_of(self.position) {
					State::Flagged => direction.turn_around(),
					State::Clean => direction.turn_left(),
					_ => direction
				};
			} else {
				self.direction = direction.turn_left();
			}
		}
		grid.cycle(self.position, self);
		let (dx, dy) = self.direction.clone().delta();
		self.position.0 += dx;
		self.position.1 += dy;
		if !self.is_on_grid(grid) {
			grid.add_layer();
		}
	}

	fn is_on_grid(&self, grid: &Grid) -> bool {
		let layers = grid.layers as isize;
		let (min, max) = (-layers, layers);
		let (x, y) = self.position;
		x >= min && x <= max && y >= min && y <= max
	}

}

#[derive(Clone, Eq, PartialEq, Hash)]
enum State {
	Clean, Weakened, Infected, Flagged
}

#[derive(Clone)]
struct Node {
	state: State
}

impl Node {
	fn new() -> Self {
		Self {
			state: State::Clean
		}
	}
}

struct Grid {
	nodes: Vec<Vec<Node>>,
	layers: usize
}

impl Grid {

	fn new(lines: Vec<&str>) -> Self {
		Self {
			nodes: lines.iter().map(|s| s.chars().map(|s| { let mut node = Node::new(); node.state = if s == '#' { State::Infected } else { State::Clean }; node }).collect()).collect(),
			layers: lines.len() / 2
		}
	}

	fn absolute_position(&self, position: (isize, isize)) -> (usize, usize) {
		let x = if position.0 <= 0 { self.layers - position.0.abs() as usize } else { self.layers + position.0.abs() as usize };
		let y = if position.1 <= 0 { self.layers + position.1.abs() as usize } else { self.layers - position.1.abs() as usize };
		(x, y)
	}

	fn state_of(&self, position: (isize, isize)) -> State {
		let (x, y) = self.absolute_position(position);
		let node = &self.nodes[y][x];
		node.state.clone()
	}

	fn is_infected(&self, position: (isize, isize)) -> bool {
		self.state_of(position) == State::Infected
	}

	fn infect(&mut self, position: (isize, isize), carrier: &mut Sporifica) {
		let (x, y) = self.absolute_position(position);
		self.nodes[y][x].state = State::Infected;
		carrier.infections_caused += 1;
	}

	fn uninfect(&mut self, position: (isize, isize), carrier: &mut Sporifica) {
		let (x, y) = self.absolute_position(position);
		self.nodes[y][x].state = State::Clean;
		carrier.uninfections_caused += 1;
	}

	fn cycle(&mut self, position: (isize, isize), carrier: &mut Sporifica) {
		let (x, y) = self.absolute_position(position);
		let state = self.state_of(position);
		if carrier.evolved {
			if state == State::Weakened { carrier.infections_caused += 1; }
			self.nodes[y][x].state = match state {
				State::Infected => State::Flagged,
				State::Flagged => State::Clean,
				State::Clean => State::Weakened,
				State::Weakened => State::Infected
			};
		} else {
			if self.is_infected(position) {
				self.uninfect(position, carrier);
			} else {
				self.infect(position, carrier);
			}
		}
	}

	fn add_layer(&mut self) {
		self.layers += 1;
		let layers = self.layers;
		let first = vec![Node::new(); 2 * layers - 1];
		let last = first.clone();
		let mut nodes = self.nodes.clone();
		nodes.insert(0, first);
		nodes.push(last);
		self.nodes = nodes.iter().map(|line| {
			let mut l = line.clone();
			l.insert(0, Node::new());
			l.push(Node::new());
			l
		}).collect();
	}

}

impl fmt::Debug for Grid {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		for line in &self.nodes {
			for p in line {
					if p.state == State::Infected {
						write!(fmt, "#")?;
					} else {
						write!(fmt, ".")?;
					}
			}
			write!(fmt, "\n")?;
		}
		Ok(())
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut grid = Grid::new(lines.clone());
		let mut carrier = Sporifica::new(false);
		for _ in 0..10_000 {
			carrier.tick(&mut grid);
		}
		let mut moar = Grid::new(lines);
		let mut better = Sporifica::new(true);
		for _ in 0..10000000 {
			better.tick(&mut moar);
		}
		return vec!(carrier.infections_caused as u32, better.infections_caused as u32);
	}
	fn index() -> i8 {
		22
	}
}