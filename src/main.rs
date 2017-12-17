use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

mod solutions;

macro_rules! do_the_solving_thing {
	($day:ident) => (
		solve_puzzle::<solutions::$day::Solution>();
	)
}

fn load_puzzle(index: i8) -> String {
	match File::open(format!("puzzles/{}.txt", index)) {
		Ok(mut f) => {
			let mut string = String::new();
			let _ = f.read_to_string(&mut string);
			string
		},
		Err(reason) => panic!("Failed to open the data for puzzle {}: {}", index, reason.description())
	}
}

fn main() {
	do_the_solving_thing!(one);
	do_the_solving_thing!(two);
	do_the_solving_thing!(three);
	do_the_solving_thing!(four);
	do_the_solving_thing!(five);
	do_the_solving_thing!(six);
}

trait Solvable {
	fn solution() -> Vec<u32>;
}

impl<T> Solvable for T where T: Puzzle {
	fn solution() -> Vec<u32> {
		let input = load_puzzle(T::index());
		T::solve(input.split("\n").collect())
	}
}

pub trait Puzzle {
	fn index() -> i8;
	fn solve(lines: Vec<&str>) -> Vec<u32>;
}

fn solve_puzzle<T : Puzzle>() {
	let solution = T::solution();
	println!("Puzzle {}:", T::index());
	for sol in solution {
		println!("{}", sol);
	}
}