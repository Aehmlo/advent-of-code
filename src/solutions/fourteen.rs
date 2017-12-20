use Puzzle;
use solutions::ten::Rope;

pub struct Solution { }

impl Solution { }

#[allow(dead_code)]
struct MemoryGrid {
	size: usize,
	base_hash: String,
	squares: Vec<Vec<bool>>
}

fn clear(squares: &mut Vec<Vec<bool>>, x: usize, y: usize) {
	if squares.len() > y && squares[x].len() > x {
		if !squares[y][x] { return }
		squares[y][x] = false;
		if y + 1 < squares.len() {
			clear(squares, x, y + 1);
		}
		if y > 0 {
			clear(squares, x, y - 1);
		}
		if x + 1 < squares[y].len() {
			clear(squares, x + 1, y);
		}
		if x > 0 {
			clear(squares, x - 1, y);
		}
	}
}

impl MemoryGrid {
	fn new(size: usize, base_hash: &str) -> Self {
		let mut grid = Self { size, base_hash: base_hash.to_string(), squares: vec!(vec!()) };
		for row in 0..size {
			let mut rope = Rope::new(256);
			let mut ascii: Vec<usize> = format!("{}-{}", base_hash, row).into_bytes().iter().map(|x| *x as usize).collect();
			for item in vec!(17, 31, 73, 47, 23) {
				ascii.push(item);
			}
			for _ in 0..64 {
				let a = ascii.clone();
				rope.tie(a);
			}
			let hash = rope.dense_hash();
			let chars = hash.chars();
			let bytes: Vec<String> = chars.map(|byte| format!("{:04b}", byte.to_digit(16).unwrap())).collect();
			let bytes = bytes.join(""); // Shadowing ftw
			let mut line: Vec<bool> = bytes.split("").map(|s| s == "1").collect();
			assert_eq!(line[129], false);
			assert_eq!(line[0], false);
			line.remove(129);
			line.remove(0);
			grid.squares.push(line);
		}
		grid.squares.remove(0);
		grid
	}
	#[allow(dead_code)]
	fn print(&self) {
		let rows = &self.squares;
		for row in rows {
			for tile in row {
				if *tile {
					print!("#");
				} else {
					print!(".");
				}
			}
			print!("\n");
		}
	}
	fn used(&self) -> u32 {
		let used: Vec<bool> = self.squares.iter().flat_map(|v| v.iter().filter(|e| **e).map(|e| *e)).collect();
		used.len() as u32
	}
	fn regions(&self) -> u32 {
		let mut regions = 0;
		let mut grid = self.squares.clone();
		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				if grid[y][x] {
					regions += 1;
					clear(&mut grid, x, y);
				}
			}
		}
		regions
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let grid = MemoryGrid::new(128, lines[0]);
		return vec!(grid.used(), grid.regions());
	}
	fn index() -> i8 {
		14
	}
}