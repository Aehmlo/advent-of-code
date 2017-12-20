use Puzzle;

pub struct Solution { }

impl Solution { }

enum Move {
	Spin(usize),
	Exchange(usize, usize),
	Partner(char, char)
}

impl Move {

	fn parse(input: &str) -> Vec<Self> {

		use self::Move::*;

		let mut moves = Vec::new();

		for instr in input.trim().split(",") {
			if instr.starts_with("s") {
				//println!("{}", &instr[1..]);
				let n = instr[1..].parse().unwrap();
				moves.push(Spin(n));
			} else if instr.starts_with("x") {
				let f: Vec<usize> = instr[1..].split("/").map(|x| x.parse().unwrap()).collect();
				moves.push(Exchange(f[0], f[1]));
			} else if instr.starts_with("p") {
				let mut f = instr[1..].split("/").map(|s| s.chars().next().unwrap());
				moves.push(Partner(f.next().unwrap(), f.next().unwrap()));
			}
		}

		moves

	}

}

struct Entourage {
    members: Vec<char>
}

impl Entourage {

    fn new() -> Self {
        Self {
            members: "abcdefghijklmnop".chars().collect() // progs
        }
    }

	fn dance(&mut self, moves: Vec<Move>) -> (String, String) {

		use self::Move::*;

		let mut seen = vec![self.members.clone()];
		let mut iterations = 0;

		for _ in 0..1_000_000_000 {
			for mv in &moves {
				match *mv {
					Spin(n) => {
						for _ in 0..n {
							let c = self.members.pop().unwrap();
							self.members.insert(0, c);
						}
					},
					Exchange(a, b) => {
						self.members.swap(a, b);
					},
					Partner(a, b) => {
						let (a, b) = {
							let mut i = self.members.iter().enumerate().filter(|&(_, c)| *c == a || *c == b).map(|(i, _)| i);
							(i.next().unwrap(), i.next().unwrap())
						};
						self.members.swap(a, b);
					}
				}
			}
			if !seen.contains(&self.members) {
				seen.push(self.members.clone());
			} else {
				if let Some((i, _)) = seen.iter().enumerate().find(|&(_, p)| p == &self.members) {
					iterations = i;
				}
				break;
			}
		}

		let period = seen.len() - iterations;
		let remainder = 1_000_000_000 % period;
		(seen[1].iter().collect::<String>(), seen[iterations + remainder].iter().collect::<String>())
	}

}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let moves = Move::parse(lines[0]);
		let mut company = Entourage::new();
		let results = company.dance(moves);
		println!("{}", results.0);
		println!("{}", results.1);
		return vec!();
	}
	fn index() -> i8 {
		16
	}
}