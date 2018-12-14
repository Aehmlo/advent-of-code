//! Day twelve (Subterranean Sustainability)

use std::str::FromStr;

const REPS: u8 = 10;
const FIFTY_BILLION: u64 = 50_000_000_000;

struct Rule {
    cond: [bool; 5],
    res: bool,
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("=>");
        let c = parts.next().expect("Empty rule.").trim();
        let mut condition = c.chars().map(|c| c == '#');
        let cond = [
            condition.next().unwrap(),
            condition.next().unwrap(),
            condition.next().unwrap(),
            condition.next().unwrap(),
            condition.next().unwrap(),
        ];
        let res = parts
            .next()
            .expect("No codomain for rule.")
            .chars()
            .last()
            .expect("Empty substitution for rule.")
            == '#';
        Ok(Self { cond, res })
    }
}

/// A tunnel containing a bunch of pots containing some unknown plants.
pub struct Tunnel {
    pots: Vec<bool>,
    rules: Vec<Rule>,
    padding: usize,
    gen: u64,
}

impl Tunnel {
    /// Populates the next generation.
    pub fn tick(&mut self) -> i64 {
        // Pad the front and back with empty pots.
        for _ in 0..3 {
            self.padding += 1;
            self.pots.insert(0, false);
            self.pots.push(false);
        }
        let mut new = self.pots.clone();
        for index in 2..(self.pots.len() - 2) {
            let slice = &self.pots[(index - 2)..=(index + 2)];
            for rule in &self.rules {
                if rule.cond == slice {
                    new[index] = rule.res;
                }
            }
        }
        self.pots = new;
        self.gen += 1;
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, p)| **p)
            .map(|(i, _)| (i as i64) - (self.padding as i64))
            .sum()
    }
}

/// Runs twenty generations of plant proliferation and returns the sum of indices with plants.
pub fn twenty(tunnel: &mut Tunnel) -> i64 {
    assert!(tunnel.gen < 20);
    while tunnel.gen < 19 {
        let _ = tunnel.tick();
    }
    tunnel.tick()
}

/// Returns the sum of indices with plants after fifty billion generations.
pub fn fifty_billion(tunnel: &mut Tunnel) -> i64 {
    assert!(tunnel.gen < FIFTY_BILLION);
    let mut previous = 0;
    let mut matching = 0;
    let mut reps = 0;
    while tunnel.gen < 49_999_999_999 {
        let newest = tunnel.tick();
        let diff = newest - previous;
        if diff == matching {
            reps += 1;
        } else {
            reps = 0;
        }
        previous = newest;
        matching = diff;
        if reps > REPS {
            return (FIFTY_BILLION - tunnel.gen) as i64 * diff + newest;
        }
    }
    tunnel.tick()
}

impl FromStr for Tunnel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut parts = lines.next().expect("Empty input.").split(' ');
        let _ = parts.next();
        let _ = parts.next();
        let init = parts.next().expect("No initial state found.");
        let state = init.chars().map(|c| c == '#');
        let pots = state.collect::<Vec<_>>();
        let _ = lines.next();
        let rules = lines
            .map(|line| line.parse::<Rule>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            pots,
            rules,
            padding: 0,
            gen: 0,
        })
    }
}

/// Solve the puzzle using the input in `puzzles/12.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/12.txt");
    let mut tunnel = input.parse::<Tunnel>().unwrap();
    let sum = twenty(&mut tunnel);
    let big = fifty_billion(&mut tunnel);
    println!("Day twelve solutions: {}, {}", sum, big);
}
