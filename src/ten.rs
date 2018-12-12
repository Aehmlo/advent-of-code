//! Day ten (The Stars Align)

use std::iter::repeat;
use std::str::FromStr;

/// Represents a point in the sky (in 2D).
pub type Point = (i32, i32);
/// Represents a veloicty in the sky (in 2D).
pub type Velocity = (i32, i32);

/// Represents a star in the sky.
#[derive(Clone)]
pub struct Particle(Point, Velocity);

impl FromStr for Particle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('<');
        let _ = parts.next();
        let mut s = parts.next().unwrap().split(',');
        let mut v = parts.next().unwrap().split(',');
        let x = s
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_numeric() || c == &'-')
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let y = s
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_numeric() || c == &'-')
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let vx = v
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_numeric() || c == &'-')
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let vy = v
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_numeric() || c == &'-')
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        Ok(Particle((x, y), (vx, vy)))
    }
}

/// Simulates the specified system, finding the time of minimum spread.
#[derive(Clone)]
pub struct Simulation {
    /// The current state of the simulation.
    pub state: Vec<Particle>,
    /// The current elapsed time.
    pub time: u32,
}

impl Simulation {
    /// Runs the next tick of the simulation, advancing the time by 1.
    pub fn tick(&mut self) {
        self.time += 1;
        for thing in self.state.iter_mut() {
            (thing.0).0 += (thing.1).0;
            (thing.0).1 += (thing.1).1;
        }
    }
    /// Returns the current bounding box of the system (x, y, width, height).
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        let mut points = self.state.iter().map(|p| p.0).collect::<Vec<_>>();
        points.sort_by(|a, b| a.0.cmp(&b.0));
        let (min_x, max_x) = (points.first().unwrap().0, points.last().unwrap().0);
        points.sort_by(|a, b| a.1.cmp(&b.1));
        let (min_y, max_y) = (points.first().unwrap().1, points.last().unwrap().1);
        (min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
    }
    /// Draws the current system state to stdout (positions only).
    pub fn draw(&self) {
        let (min_x, min_y, width, height) = self.bounds();
        let mut lines = repeat(repeat(' ').take(width as usize))
            .take(height as usize)
            .map(|i| i.collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for point in self.state.iter().map(|p| p.0) {
            lines[(point.1 - min_y) as usize][(point.0 - min_x) as usize] = 'x';
        }
        for line in lines {
            for point in line {
                print!("{}", point);
            }
            println!("");
        }
    }
}

impl FromStr for Simulation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let particles = s
            .lines()
            .filter_map(|l| {
                if l.is_empty() {
                    None
                } else {
                    Some(l.parse::<Particle>().unwrap())
                }
            })
            .collect::<Vec<_>>();
        Ok(Self {
            state: particles,
            time: 0,
        })
    }
}

/// Finds the time in the simulation where the smallest bounding box occurs, returning the state.
pub fn find_smallest(sim: &mut Simulation) -> Simulation {
    let bounds = sim.bounds();
    let mut min_height = bounds.3;
    let mut best = sim.clone();
    loop {
        sim.tick();
        let bounds = sim.bounds();
        if bounds.3 < min_height {
            min_height = bounds.3;
            best = sim.clone();
        } else {
            break;
        }
    }
    best
}

/// Solve the puzzle using the input in `puzzles/10.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/10.txt");
    let mut sim = input.parse::<Simulation>().unwrap();
    let sim = find_smallest(&mut sim);
    println!("Day ten solutions: {}", sim.time);
    sim.draw();
}
