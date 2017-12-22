use Puzzle;
use std::ops::Add;
use std::ops::Sub;
use std::ops::BitXor;
use std::ops::Mul;
use std::ops::Div;
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::PartialEq;
use std::cmp::Eq;

pub struct Solution { }

impl Solution { }

#[derive(Clone, Debug)]
struct Vector {
	x: isize,
	y: isize,
	z: isize
}

impl Vector {
	fn manhattan_distance(&self) -> isize {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
	fn key(&self) -> String {
		format!("{},{},{}", self.x, self.y, self.z)
	}
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}
impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl BitXor<usize> for Vector { // Element-wise exponentiation
	type Output = Vector;
	fn bitxor(self, rhs: usize) -> Vector {
		Vector { x: self.x.pow(rhs as u32), y: self.y.pow(rhs as u32), z: self.z.pow(rhs as u32) }
	}
}

impl Mul<isize> for Vector {
	type Output = Vector;
	fn mul(self, rhs: isize) -> Vector {
		Vector { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
	}
}

impl Div<isize> for Vector {
	type Output = Vector;
	fn div(self, rhs: isize) -> Vector {
		Vector { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
	}
}

impl Eq for Vector { }

impl PartialEq for Vector {
	fn eq(&self, other: &Vector) -> bool {
		self.x == other.x && self.y == other.y && self.z == other.z
	}
}

#[derive(Debug, Clone)]
struct Particle {
	position: Vector,
	velocity: Vector,
	acceleration: Vector
}

impl Particle {
	fn tick(&mut self) {
		self.velocity = self.velocity.clone() + self.acceleration.clone();
		self.position = self.position.clone() + self.velocity.clone();
	}
}

impl FromStr for Particle {
	type Err = String;
	fn from_str(s: &str) -> Result<Particle, String> { // p=<5528,2008,1661>, v=<-99,-78,-62>, a=<-17,-2,-2>
		let vectors: Vec<Vector> = s.split(", ").map(|s| {
			let parts: Vec<&str> = s.split("=").collect();
			let st: &str = parts[1];
			st.parse().unwrap()
		}).collect();
		Ok(Particle { position: vectors[0].clone(), velocity: vectors[1].clone(), acceleration: vectors[2].clone() })
	}
}

impl FromStr for Vector {
	type Err = String;
	fn from_str(s: &str) -> Result<Vector, String> {
		let components: Vec<isize> = s.replace("<", "").replace(">", "").split(",").map(|s| s.parse().unwrap()).collect();
		Ok(Vector { x: components[0], y: components[1], z: components[2] })
	}
}

struct Simulation {
	particles: Vec<Particle>
}

impl Simulation {

	fn tick(&mut self, remove: bool) {
		let mut positions: HashMap<String, Vec<usize>> = HashMap::new();
		let mut particles = &mut self.particles;
		let len = particles.len();
		for i in 0..len {
			let mut particle = &mut particles[i];
			particle.tick();
			if positions.contains_key(&particle.position.key()) {
				let mut particles = positions[&particle.position.key()].clone();
				particles.push(i);
				positions.insert(particle.position.key(), particles);
			} else {
				positions.insert(particle.position.key(), vec!(i));
			}
		}
		if remove {
			let mut p: Vec<usize> = positions.values().filter(|indices| indices.len() > 1).flat_map(|location| location).map(|n| *n).collect();
			p.sort();
			let len = p.len();
			for i in 0..len {
				let index = p[i];
				particles.remove(index - i);
			}
		}
	}
	fn nearest(&self) -> usize {
		let distances: Vec<isize> = self.particles.iter().map(|p| p.position.manhattan_distance()).collect();
		let mut min = distances[0];
		let mut index = 0;
		for (i, v) in distances.iter().enumerate() {
			if *v < min {
				min = *v;
				index = i;
			}
		}
		index
	}
}

impl Puzzle for Solution {
	fn solve(lines: Vec<&str>) -> Vec<u32> {
		let particles: Vec<Particle> = lines.iter().map(|line| line.parse().unwrap()).collect();
		let mut simulation = Simulation { particles: particles.clone() };
		let mut emulation = Simulation { particles };
		for _ in 0..1_000 {
			simulation.tick(false);
			emulation.tick(true);
		}
		return vec!(simulation.nearest() as u32, emulation.particles.len() as u32);
	}
	fn index() -> i8 {
		20
	}
}