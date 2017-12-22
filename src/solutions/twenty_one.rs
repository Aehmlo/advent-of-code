use Puzzle;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::fmt;

pub struct Solution { }

impl Solution { }

#[derive(Clone, Hash, Eq, PartialEq)]
struct Image {
	data: Vec<Vec<bool>>
}

impl Image {

	fn new(size: usize) -> Self {
		Self {
			data: vec![vec![false; size]; size]
		}
	}

	fn size(&self) -> usize {
		self.data.len()
	}

	fn merge(&mut self, x: usize, y: usize, other: &Self) {
		for (x, a) in (x..(x + other.size())).enumerate() {
			for (y, b) in (y..(y + other.size())).enumerate() {
				let data = other.data[x][y];
				self.data[a][b] = data;
			}
		}
	}

	fn flip(&self) -> Self {
		let mut image = self.clone();
		for y in 0..image.size() {
			for (a, b) in (0..(image.size() / 2)).zip(((image.size() / 2)..image.size()).rev()) {
				let (z, w) = (image.data[a][y], image.data[b][y]);
				image.data[a][y] = w;
				image.data[b][y] = z;
			}
		}
		image
	}

	fn set_row(&mut self, y: usize, values: &[bool]) {
		let s = self.size();
		let mut data = &mut self.data;
		for i in 0..s {
			data[i][y] = values[i];
		}
	}

	fn rotate(&self) -> Self {

		let mut image = self.clone();

		for r in 0..(image.size() / 2) {
			let z = r + image.size() - r * 2;
			for (x, y) in (r..z).zip(r..z) {

				// Good luck parsing this

				let s = self.size();

				let v = self.data[x][r];
				image.data[s - r - 1][y] = v;

				let v = self.data[s - r - 1][y];
				image.data[s - x - 1][s - r - 1] = v;

				let v = self.data[s - x - 1][s - r - 1];
				image.data[r][s - y - 1] = v;

				let v = self.data[r][s - y - 1];
				image.data[x][r] = v;

			}
		}

		image

	}

	fn frame(&self, size: usize) -> MoveableFrame {
		MoveableFrame {
			image: self,
			size: size,
			x: 0,
			y: 0
		}
	}

	fn on_after(iterations: usize, patterns: &HashMap<Image, Image>) -> usize {

		let mut image = Image::new(3);
		image.set_row(0, &[false, true, false]);
		image.set_row(1, &[false, false, true]);
		image.set_row(2, &[true, true, true]);

		for _ in 0..iterations {
			if image.size() % 2 == 0 {

				let mut neue = Image::new((image.size() * 3) / 2);

				for(x, y, img) in image.frame(2) {
					let ref m = patterns[&img];
					neue.merge(x * m.size(), y * m.size(), m);
				}

				image = neue;

			} else if image.size() % 3 == 0 {

				let mut neue = Image::new((image.size() * 4) / 3);

				for (x, y, img) in image.frame(3) {
					let ref m = patterns[&img];
					neue.merge(x * m.size(), y * m.size(), m);
				}

				image = neue;

			} else { panic!(); }

		}

		let mut on = 0;
		for row in image.data {
			let o = row.iter().filter(|p| **p).count();
			on += o;
		}

		on

	}

}

impl fmt::Debug for Image {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    	for line in &self.data {
    		for p in line {
  				if *p {
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

impl FromStr for Image {

	type Err = String;

	fn from_str(s: &str) -> Result<Image, Self::Err> {
		let pixels: Vec<Vec<bool>> = s.trim().split("/").map(|x| x.chars().map(|c| c == '#').collect()).collect();
		let size = pixels[0].len();
		let mut image = Image::new(size);
		for (y, r) in pixels.iter().enumerate() {
			image.set_row(y, &r);
		}
		Ok(image)
	}

}

// Inspired heavily by https://github.com/udoprog/rust-advent-of-code-2017/blob/master/src/day21.rs
struct MoveableFrame<'a> { // A frame that "slides" across the image, exposing only one part of it at a time.
	image: &'a Image, // Lifetimes are annoying, but I'm learning how to make them work for me!
	size: usize,
	x: usize,
	y: usize
}

impl<'a> Iterator for MoveableFrame<'a> {

	type Item = (usize, usize, Image);

	fn next(&mut self) -> Option<Self::Item> {

		if self.x * self.size >= self.image.size() { // End of row
			self.x = 0;
			self.y += 1;
		}

		if self.y * self.size >= self.image.size() { // Out of pixels/data
			return None;
		}

		let (x, y) = (self.x * self.size, self.y * self.size);
		let mut subimage = Image::new(self.size);
		for(a, c) in (x..(x + self.size)).enumerate() {
			for (b, d) in (y..(y + self.size)).enumerate() {
				let v = self.image.data[c][d];
				subimage.data[a][b] = v;
			}
		}

		let (x, y) = (self.x, self.y);
		self.x += 1;

		Some((x, y, subimage))

	}

}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {

		let mut patterns = HashMap::new();
		for line in lines {

			let parts: Vec<Image> = line.split("=>").map(|x| x.parse().unwrap()).collect();
			let mut uniques = HashSet::new();

			let mut neue = parts[0].clone();
			for _ in 0..4 {
				uniques.insert(neue.clone());
				neue = neue.rotate();
			}
			uniques.insert(neue.clone());

			let mut neue = parts[0].flip();
			for _ in 0..4 {
				uniques.insert(neue.clone());
				neue = neue.rotate();
			}
			uniques.insert(neue.clone());

			for p in uniques {
				if patterns.insert(p, parts[1].clone()).is_some() {
					panic!("We've already seen this pattern.");
				}
			}
		}

		return vec!(Image::on_after(5, &patterns) as u32, Image::on_after(18, &patterns) as u32);
	}
	fn index() -> i8 {
		21
	}
}