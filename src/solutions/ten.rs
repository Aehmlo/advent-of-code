use Puzzle;

pub struct Solution { }

impl Solution {
}

struct Rope {
	nodes: Vec<usize>,
	skip: usize,
	position: usize
}

impl Rope {
	fn new(size: usize) -> Self {
		Rope {
			nodes: (0..size).collect(),
			skip: 0,
			position: 0
		}
	}
	fn inc_position(&mut self, delta: usize) -> usize {
		let mut position = self.position;
		position += delta + self.skip;
		self.skip += 1;
		position %= self.nodes.len();
		self.position = position;
		position
	}
	fn knot(&mut self, length: usize) {
		let mut n = &mut self.nodes;
		let len = n.len();
		let pos = self.position;
		let nodes: Vec<usize> = (0..length).map(|i| n[(pos + i) % len]).rev().collect();
		for i in 0..length {
			n[(pos + i) % len] = nodes[i];
		}
	}
	fn check(&self) -> usize {
		self.nodes[0] * self.nodes[1]
	}
	fn tie(&mut self, offsets: Vec<usize>) {
		for offset in offsets {
			self.knot(offset);
			self.inc_position(offset);
		}
	}
	fn dense_hash(&self) -> String {
		let len = (self.nodes.len() as f32).sqrt() as usize;
		let mut coll: Vec<u32> = vec!();
		for i in 0..len {
			let mut hash = self.nodes[i * len];
			for j in (i*len + 1)..((i+1)*len) {
				hash ^= self.nodes[j];
			}
			coll.push(hash as u32);
		}
		let them: Vec<String> = coll.iter().map(|x| format!("{:x}", x)).collect();
		return them.join("");
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let mut rope = Rope::new(256);
		let offsets = lines[0].split(",").map(|x| x.parse().unwrap()).collect();
		rope.tie(offsets);
		let mut ascii = lines[0].to_string().into_bytes();
		let others: Vec<u8> = lines[1].split(",").map(|x| x.parse().unwrap()).collect();
		for additional in others {
			ascii.push(additional);
		}
		let mut garland = Rope::new(256);
		for _ in 0..64 {
			garland.tie(ascii.iter().map(|x| *x as usize).collect());
		}
		println!("{}", garland.dense_hash());
		return vec!(rope.check() as u32);
	}
	fn index() -> i8 {
		10
	}
}