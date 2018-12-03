//! Day three (No Matter How You Slice It)

use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

/// Represents a point on the fabric.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point(usize, usize);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

/// Represents a claimed area of fabric.
pub struct Claim {
    origin: Point,
    width: usize,
    height: usize,
    /// The identifier of the claim.
    pub id: String,
}

impl FromStr for Claim {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut first_two = string.split(" @ ");
        let id = first_two.next().expect("No claim ID given").to_string();
        let spec = first_two.next().expect("No position/size given.");
        let mut spec = spec.split(": ");
        let pos = spec.next().expect("No position given.");
        let size = spec.next().expect("No size given.");
        let mut origin = pos.split(",").map(|e| e.parse::<usize>().unwrap());
        let x = origin.next().expect("No x-offset given.");
        let y = origin.next().expect("No y-offset given.");
        let origin = Point(x, y);
        let mut size = size.split("x").map(|e| e.parse::<usize>().unwrap());
        let width = size.next().expect("No width given.");
        let height = size.next().expect("No height given.");
        Ok(Self {
            origin,
            width,
            height,
            id,
        })
    }
}

impl Claim {
    /// The points a claim, well, claims.
    ///
    /// Each point is assumed to cover a unit area.
    pub fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..self.width {
            for j in 0..self.height {
                points.push(self.origin + Point(i, j));
            }
        }
        points
    }
}

/// Constructs a map of the number of claims on each point.
fn claimed_area(claims: impl Iterator<Item = Claim>) -> HashMap<Point, usize> {
    let mut grid = HashMap::new();
    for point in claims.flat_map(|c| c.points().into_iter()) {
        *grid.entry(point).or_insert(0) += 1;
    }
    grid
}

/// Part 1: find the total overlap area of all claims.
pub fn overlapping_area(grid: &HashMap<Point, usize>) -> usize {
    grid.values().filter(|a| **a > 1).count()
}

/// Part 2: find a claim which does not overlap with any other claim.
pub fn usable_claims(
    claims: impl Iterator<Item = Claim>,
    grid: HashMap<Point, usize>,
) -> impl Iterator<Item = Claim> {
    claims.filter(move |claim| {
        let points = claim.points();
        for point in points {
            if grid[&point] > 1 {
                return false;
            }
        }
        true
    })
}

/// Solve the puzzle using the input in `puzzles/3.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/3.txt");
    let claims = input.lines().map(|l| l.parse::<Claim>().unwrap());
    let grid = claimed_area(claims.clone());
    println!(
        "Day three solutions: {}, {}",
        overlapping_area(&grid),
        usable_claims(claims, grid).next().unwrap().id
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
        let claims = input.lines().map(|l| l.parse::<Claim>().unwrap());
        let grid = claimed_area(claims.clone());
        assert_eq!(overlapping_area(&grid), 4);
        assert_eq!(usable_claims(claims, grid).next().unwrap().id, "#3");
    }
}
