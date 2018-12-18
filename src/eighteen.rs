//! Day eighteen (Settlers of The North Pole)

use std::collections::HashMap;
use std::str::FromStr;

const HUGE: usize = 1_000_000_000;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Space {
    Open,
    Trees,
    Lumberyard,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Space::Open,
            '|' => Space::Trees,
            '#' => Space::Lumberyard,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Yard {
    grid: Vec<Vec<Space>>,
}

fn neighbors(x: usize, y: usize, grid: &[Vec<Space>]) -> Vec<Space> {
    let mut spaces = Vec::new();
    if y > 0 {
        if x > 0 {
            spaces.push(grid[y - 1][x - 1]);
        }
        spaces.push(grid[y - 1][x]);
        if x < grid[y].len() - 1 {
            spaces.push(grid[y - 1][x + 1]);
        }
    }
    if x > 0 {
        spaces.push(grid[y][x - 1]);
    }
    if x < grid[y].len() - 1 {
        spaces.push(grid[y][x + 1]);
    }
    if y < grid.len() - 1 {
        if x > 0 {
            spaces.push(grid[y + 1][x - 1]);
        }
        spaces.push(grid[y + 1][x]);
        if x < grid[y].len() - 1 {
            spaces.push(grid[y + 1][x + 1]);
        }
    }
    spaces
}

impl Yard {
    fn tick(&mut self) {
        let state = self.grid.clone();
        for i in 0..state.len() {
            for j in 0..state.len() {
                let neighbors = neighbors(i, j, &state);
                match state[j][i] {
                    Space::Open => {
                        if neighbors.iter().filter(|s| s == &&Space::Trees).count() >= 3 {
                            self.grid[j][i] = Space::Trees;
                        }
                    }
                    Space::Trees => {
                        if neighbors
                            .iter()
                            .filter(|s| s == &&Space::Lumberyard)
                            .count()
                            >= 3
                        {
                            self.grid[j][i] = Space::Lumberyard;
                        }
                    }
                    Space::Lumberyard => {
                        if neighbors
                            .iter()
                            .filter(|s| s == &&Space::Lumberyard)
                            .count()
                            < 1
                            || neighbors.iter().filter(|s| s == &&Space::Trees).count() < 1
                        {
                            self.grid[j][i] = Space::Open;
                        }
                    }
                }
            }
        }
    }
    fn value(&self) -> usize {
        let l = self
            .grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|s| s == &&Space::Lumberyard)
            .count();
        let t = self
            .grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|s| s == &&Space::Trees)
            .count();
        l * t
    }
}

impl FromStr for Yard {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut grid = Vec::new();
        for line in s.lines().filter(|l| !l.is_empty()) {
            let mut l = Vec::new();
            for c in line.chars() {
                let s: Space = c.into();
                l.push(s);
            }
            grid.push(l);
        }
        Ok(Self { grid })
    }
}

/// Solve the puzzle using the input in `puzzles/18.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/18.txt");
    let mut yard = input.parse::<Yard>().unwrap();
    let mut seen = HashMap::new();
    for i in 1..10 {
        yard.tick();
        seen.insert(yard.clone(), i);
    }
    let smol = yard.value();
    let mut big = 0;
    for j in 10..=HUGE {
        yard.tick();
        let exists = seen.keys().any(|k| k == &yard);
        if exists {
            let i = seen[&yard];
            let d = j - i;
            let r = HUGE - i;
            let f = r % d;
            big = seen
                .iter()
                .find(|(_, s)| **s == f + i)
                .map(|(y, _)| y.value())
                .unwrap();
            break;
        } else {
            seen.insert(yard.clone(), j);
        }
    }
    println!("Day eighteen solutions: {}, {}", smol, big);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input() {
        let input = ".#.#...|#.\n.....#|##|\n.|..|...#.\n..|#.....#\n#.#|||#|#|\n...#.||...\n.|....|...\n||...#|.#|\n|.||||..|.\n...#.|..|.";
        let mut yard = input.parse::<Yard>().unwrap();
        for _ in 0..10 {
            yard.tick();
        }
        assert_eq!(yard.value(), 1147);
    }
}
