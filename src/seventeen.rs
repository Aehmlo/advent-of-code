//! Day seventeen (Reservoir Research)

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point(usize, usize);

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.1.cmp(&other.1).then(self.0.cmp(&other.0))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

struct Vein(BTreeSet<Point>);

impl FromStr for Vein {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut parts = s.split(' ');
        let first = parts.next().expect("No first part.").replace(',', "");
        let mut first = first.split('=');
        let xy = first
            .next()
            .expect("No first coordinate.")
            .chars()
            .next()
            .unwrap();
        let coord = first
            .next()
            .expect("No first coordinate value.")
            .parse::<usize>()
            .unwrap();
        let mut second = parts.next().expect("No second part").split('=');
        let _ = second.next();
        let mut last = second.next().unwrap().split("..");
        let lower = last
            .next()
            .expect("No lower bound.")
            .parse::<usize>()
            .unwrap();
        let upper = last
            .next()
            .expect("No upper bound.")
            .parse::<usize>()
            .unwrap();
        let mut points = BTreeSet::new();
        if xy == 'x' {
            for j in lower..=upper {
                points.insert(Point(coord, j));
            }
        } else {
            for i in lower..=upper {
                points.insert(Point(i, coord));
            }
        }
        Ok(Vein(points))
    }
}

#[derive(Default)]
struct Ground {
    clay: BTreeSet<Point>,
    settled: Vec<Point>,
    flowing: Vec<Point>,
}

impl FromStr for Ground {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let clay = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<Vein>().unwrap())
            .flat_map(|v| v.0.into_iter())
            .collect::<BTreeSet<_>>();
        Ok(Self {
            clay,
            ..Default::default()
        })
    }
}

impl Ground {
    /// Adds a stream of water from the given point, returning whether the system changed.
    pub fn drip(&mut self, from: Point) -> bool {
        let max_y = self.clay.iter().last().unwrap().1;
        for j in from.1..=max_y {
            let point = Point(from.0, j);
            if self.clay.contains(&point) || self.settled.iter().any(|x| x == &point) {
                let y = j - 1;
                let mut left_stop = false;
                let mut right_stop = false;
                let mut moved = false;
                let mut list = Vec::new();
                for i in (0..from.0).rev() {
                    if self.clay.contains(&Point(i, y)) {
                        left_stop = true;
                        break;
                    }
                    list.push(Point(i, y));
                    self.flowing.push(Point(i, y));
                    if !self.clay.contains(&Point(i, j))
                        && self.settled.iter().find(|x| x == &&(Point(i, j))).is_none()
                    {
                        if self.drip(Point(i, y)) {
                            moved = true;
                        }
                        break;
                    }
                }
                for i in from.0..1000 {
                    if self.clay.contains(&Point(i, y)) {
                        right_stop = true;
                        break;
                    }
                    self.flowing.push(Point(i, y));
                    list.push(Point(i, y));
                    if !self.clay.contains(&Point(i, j))
                        && self.settled.iter().find(|x| x == &&(Point(i, j))).is_none()
                    {
                        if self.drip(Point(i, y)) {
                            moved = true;
                        }
                        break;
                    }
                }
                if left_stop && right_stop {
                    moved = true;
                    let flowing = self
                        .flowing
                        .iter()
                        .filter(|p| list.iter().find(|x| x == p).is_none())
                        .cloned()
                        .collect::<Vec<_>>();
                    self.flowing = flowing;
                    for point in list {
                        self.settled.push(point);
                    }
                }
                return moved;
            } else {
                self.flowing.push(point);
            }
        }
        false
    }
    /// Runs the simulation until a dynamic equilibrium is reached.
    pub fn equil(&mut self) -> (usize, usize) {
        while self.drip(Point(500, 0)) {}
        self.flowing.sort();
        self.flowing.dedup();
        self.settled.sort();
        self.settled.dedup();
        let min_y = self.clay.iter().next().unwrap().1;
        let max_y = self.clay.iter().last().unwrap().1;
        (
            self.flowing
                .iter()
                .filter(|p| p.1 >= min_y && p.1 <= max_y)
                .count(),
            self.settled
                .iter()
                .filter(|p| p.1 >= min_y && p.1 <= max_y)
                .count(),
        )
    }
}

/// Solve the puzzle using the input in `puzzles/17.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/17.txt");
    let mut ground = input.parse::<Ground>().unwrap();
    let water = ground.equil();
    println!(
        "Day seventeen solutions: {}, {}",
        water.0 + water.1,
        water.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input() {
        let input = "x=495, y=2..7\ny=7, x=495..501\nx=501, y=3..7\nx=498, y=2..4\nx=506, y=1..2\nx=498, y=10..13\nx=504, y=10..13\ny=13, x=498..504\n";
        let mut ground = input.parse::<Ground>().unwrap();
        let water = ground.equil();
        assert_eq!(water.0 + water.1, 57);
        assert_eq!(water.1, 29);
    }
}
