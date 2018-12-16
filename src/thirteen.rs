//! Day thirteen (Mine Cart Madness)

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Track {
    /// A straight rail.
    Path,
    /// A curve (left/right).
    Corner(bool),
    /// An intersection where two perpendicular tracks meet on the grid.
    Intersection,
}

/// Represents a direction on the grid (of tracks).
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    /// The direction toward the "top" of the grid.
    Up,
    /// The direction toward the "bottom" of the grid.
    Down,
    /// The direction toward the "left" of the grid.
    Left,
    /// The direction toward the "right" of the grid.
    Right,
}

/// Encodes what turn to favor next.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Turn {
    /// The cart should turn left at the next intersection.
    Left,
    /// The cart should go straight at the next intersection.
    Straight,
    /// The cart should turn right at the next intersection.
    Right,
}

impl Turn {
    /// Gets the next direction the cart should turn at an intersection.
    pub fn next(self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

impl Default for Turn {
    fn default() -> Self {
        Turn::Left
    }
}

/// Represents a distance (dx, dy) from the top left of the grid.
#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Point(usize, usize);

impl Point {
    /// Moves in the given direction from this point.
    pub fn advance(self, delta: Direction) -> Point {
        match delta {
            Direction::Up => Point(self.0, self.1 - 1),
            Direction::Down => Point(self.0, self.1 + 1),
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Right => Point(self.0 + 1, self.1),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{},{}", self.0, self.1)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Direction {
    /// The next direction, moving clockwise.
    pub fn cw(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    /// The next direction, moving counter-clockwise.
    pub fn ccw(self) -> Self {
        self.cw().cw().cw()
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        use self::Direction::*;
        match c {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' | _ => Right,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

/// Represents a mine cart.
#[derive(Clone, Default, Eq, PartialEq)]
struct Cart {
    pos: Point,
    dir: Direction,
    bias: Turn,
    ded: bool,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Cart {
    /// Advances the cart according to the rules of the given grid.
    fn advance(self, grid: &Grid) -> Cart {
        use self::Direction::*;
        if self.ded {
            return self;
        }
        let pos = self.pos.advance(self.dir);
        let dir = match grid[&pos] {
            Track::Path => self.dir,
            Track::Corner(right) => match (right, self.dir) {
                (true, Up) | (true, Down) | (false, Right) | (false, Left) => self.dir.cw(),
                _ => self.dir.ccw(),
            },
            Track::Intersection => match self.bias {
                Turn::Left => self.dir.ccw(),
                Turn::Straight => self.dir,
                Turn::Right => self.dir.cw(),
            },
        };

        let bias = if grid[&pos] == Track::Intersection {
            self.bias.next()
        } else {
            self.bias
        };

        Cart {
            pos,
            dir,
            bias,
            ded: false,
        }
    }
}

type Grid = HashMap<Point, Track>;

#[derive(Default)]
struct Sim {
    grid: Grid,
    carts: Vec<Cart>,
}

impl Sim {
    /// Returns the current number of intact carts.
    pub fn intact(&self) -> usize {
        self.carts.iter().filter(|c| !c.ded).count()
    }
    /// Runs the next tick of the simulation, returning the position of any crash that happened.
    pub fn tick(&mut self) -> Option<Point> {
        self.carts.sort();
        let mut crash = None;
        for i in 0..self.carts.len() {
            let cart = self.carts[i].clone();
            if cart.ded {
                continue;
            }
            let cart = cart.advance(&self.grid);
            self.carts[i] = cart;
            for j in 0..self.carts.len() {
                if j == i {
                    continue;
                }
                if self.carts[i].pos == self.carts[j].pos {
                    if self.carts[i].ded || self.carts[j].ded {
                        continue;
                    }
                    self.carts[i].ded = true;
                    self.carts[j].ded = true;
                    crash = Some(self.carts[i].pos);
                }
            }
        }
        crash
    }
    /// Returns the location of the next crash that occurs, if applicable.
    pub fn next_crash(&mut self) -> Option<Point> {
        let mut collision = None;
        while collision.is_none() && self.intact() > 1 {
            collision = self.tick();
        }
        collision
    }
    /// Returns the location of the last crash that occurs, if applicable.
    pub fn last_crash(&mut self) -> Option<Point> {
        while self.intact() > 1 {
            self.tick();
        }
        self.carts.iter().find(|c| !c.ded).map(|c| c.pos)
    }
}

#[derive(Debug)]
enum FromStrErr {
    /// A character we don't understand was encountered.
    UnknownCharacter(char),
}

impl FromStr for Sim {
    type Err = FromStrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut carts = Vec::new();
        let mut grid = HashMap::new();
        for (j, line) in s.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '^' | 'v' | '<' | '>' => {
                        grid.insert(Point(i, j), Track::Path);
                        let dir = c.into();
                        let mut cart = Cart::default();
                        cart.pos = Point(i, j);
                        cart.dir = dir;
                        carts.push(cart);
                    }
                    '\\' => {
                        grid.insert(Point(i, j), Track::Corner(false));
                    }
                    '/' => {
                        grid.insert(Point(i, j), Track::Corner(true));
                    }
                    '+' => {
                        grid.insert(Point(i, j), Track::Intersection);
                    }
                    '-' | '|' => {
                        grid.insert(Point(i, j), Track::Path);
                    }
                    ' ' => {}
                    c => {
                        return Err(FromStrErr::UnknownCharacter(c));
                    }
                }
            }
        }
        Ok(Sim { carts, grid })
    }
}

/// Solve the puzzle using the input in `puzzles/13.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/13.txt");
    let mut sim = input.parse::<Sim>().expect("Parse failure.");
    let first = sim.next_crash();
    let last = sim.last_crash();
    println!(
        "Day thirteen solutions: {}; {}",
        first.unwrap_or_default(),
        last.unwrap_or_default()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn sample_input() {
        let input = "/->-\\        \n|   |  /----\\\n| /-+--+-\\  |\n| | |  | v  |\n\\-+-/  \\-+--/\n  \\------/";
        let mut sim = input.parse::<Sim>().unwrap();
        assert_eq!(sim.next_crash(), Some(Point(7, 3)));
        let input = "/>-<\\  \n|   |  \n| /<+-\\\n| | | v\n\\>+</ |\n  |   ^\n  \\<->/";
        sim = input.parse::<Sim>().unwrap();
        assert_eq!(sim.last_crash(), Some(Point(6, 4)));
    }
}
