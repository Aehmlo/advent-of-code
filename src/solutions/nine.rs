use Puzzle;

#[derive(Clone)]
enum Entity {
	Group(isize),
	Garbage(isize)
}

#[derive(Clone)]
struct Stream {
	components: Vec<Entity>
}

impl Stream {
	fn new() -> Stream {
		Stream { components: vec!() }
	}
	fn push(&mut self, item: Entity) {
		self.components.push(item);
	}
	fn parse(input: &str) -> Stream {
		let mut stream = Stream::new();
		let mut in_garbage = false;
		let mut garbage_score: isize = 0;
		let mut ignore = false;
		let mut score: isize = 1;
		for character in input.chars() {
			match character {
				'{' if !in_garbage => { stream.push(Entity::Group(score)); score += 1; },
				'}' if !in_garbage => { score -= 1; },
				'!' if in_garbage && !ignore => { ignore = true; },
				'<' if !in_garbage => { in_garbage = true; garbage_score = 0; },
				'>' if in_garbage && !ignore => { stream.push(Entity::Garbage(garbage_score)); in_garbage = false; },
				_ if ignore => { ignore = false; },
				_ if in_garbage && !ignore => { garbage_score += 1; },
				_ => {}
			};
		}
		stream
	}
}

impl Iterator for Stream {

	type Item = Entity;

	fn next(&mut self) -> Option<Entity> {
		self.components.pop()
	}

}

pub struct Solution { }

impl Solution { }

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let stream = Stream::parse(lines[0]);
		let mut score: isize = 0;
		for packet in stream.clone() {
			score += match packet {
				Entity::Group(s) => s,
				_ => 0
			};
		}
		let mut garbage_score: isize = 0;
		for packet in stream {
			garbage_score += match packet {
				Entity::Garbage(s) => s,
				_ => 0
			};
		}
		return vec!(score as u32, garbage_score as u32);
	}
	fn index() -> i8 {
		9
	}
}