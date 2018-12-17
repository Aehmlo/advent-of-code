//! Day fifteen (Beverage Bandits)

use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::ops::{Add, Index, IndexMut, Sub};
use std::str::FromStr;

/// Represents a vector in (Cartesian) 2-space.
pub struct Vector {
    dy: i16,
    dx: i16,
}

impl Vector {
    /// The Manhattan length of the vector.
    pub fn length(&self) -> usize {
        (self.dx.abs() + self.dy.abs()) as usize
    }
}

/// Represents a point in (Cartesian) 2-space.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    y: u16,
    x: u16,
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    /// Constructs a new point at `(x, y)`.
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    /// Returns the point immediately above this point.
    fn up(self) -> Self {
        Point {
            x: self.x,
            y: self.y.saturating_sub(1),
        }
    }
    /// Returns the point immediately below this point.
    fn down(self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    /// Returns the point immediately left of this point.
    fn left(self) -> Self {
        Point {
            x: self.x.saturating_sub(1),
            y: self.y,
        }
    }
    /// Returns the point immediately right of this point.
    fn right(self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    /// Returns the (four) adjacent points to this point (left, right, down, up).
    fn adjacent(self) -> [Self; 4] {
        [self.left(), self.up(), self.right(), self.down()]
    }
    /// Returns the distance between this point and another.
    fn distance(self, other: Point) -> usize {
        (other - self).length()
    }
    /// Indicates whether this point is adjacent to the given point (i.e. one space away).
    fn is_adjacent(self, other: Point) -> bool {
        self.distance(other) == 1
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        Vector {
            dy: (other.y as i16) - (self.y as i16),
            dx: (other.x as i16) - (self.x as i16),
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, other: Vector) -> Point {
        Point {
            x: ((self.x as i16) + other.dx) as u16,
            y: ((self.y as i16) + other.dy) as u16,
        }
    }
}

/// Represents a space on the game grid.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Space {
    Open,
    Occupied(Team),
    Wall,
}

/// Represents an allegiance (either goblin or elf).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Team {
    Goblin,
    Elf,
}

/// Represents a participant in the combat.
#[derive(Clone)]
pub struct Player {
    fealty: Team,
    health: u16,
    attack: u16,
    position: Point,
}

impl Player {
    /// Whether the player is still alive (has nonzero health).
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{:?} [{}]",
            if self.fealty == Team::Elf { 'E' } else { 'G' },
            self.position,
            self.health
        )
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            fealty: Team::Elf,
            health: 200,
            attack: 3,
            position: Point::new(0, 0),
        }
    }
}

/// Encodes the game grid.
#[derive(Clone)]
pub struct Grid {
    spaces: Vec<Space>,
    axis: u16,
}

impl Grid {
    /// Marks the passed point as open.
    pub fn vacate(&mut self, point: Point) {
        self[point] = Space::Open;
    }
    /// Claims the passed point for the appropriate team.
    pub fn place(&mut self, player: &Player, point: Point) {
        self[point] = Space::Occupied(player.fealty);
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.spaces.len() {
            let c = match self.spaces[i] {
                Space::Open => '.',
                Space::Wall => '#',
                Space::Occupied(team) => match team {
                    Team::Elf => 'E',
                    Team::Goblin => 'G',
                },
            };
            if i > 0 && i % (self.axis as usize) == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Index<Point> for Grid {
    type Output = Space;
    fn index(&self, index: Point) -> &Space {
        &self.spaces[(index.y * self.axis + index.x) as usize]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut Space {
        &mut self.spaces[(index.y * self.axis + index.x) as usize]
    }
}

/// Represents a skirmish.
#[derive(Clone)]
pub struct Fight {
    players: Vec<Player>,
    grid: Grid,
}

// Thanks to bertptrs for the inspiration.
fn find_move(
    grid: &Grid,
    player: &Player,
    targets: impl Iterator<Item = (usize, Player)>,
) -> Option<Point> {
    let positions = targets.map(|(_, p)| p.position).collect::<HashSet<_>>();
    let mut path: HashMap<Point, Point> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, player.position)));
    while let Some(Reverse((depth, point))) = queue.pop() {
        let possible = point.adjacent();
        for maybe in &possible {
            if !path.contains_key(maybe) && grid[*maybe] != Space::Wall {
                if positions.contains(maybe) {
                    return path.remove(&point);
                } else if grid[*maybe] == Space::Open {
                    let previous = *path.get(&point).unwrap_or(maybe);
                    path.insert(*maybe, previous);
                    queue.push(Reverse((depth + 1, *maybe)));
                }
            }
        }
    }
    None
}

fn find_attack(
    player: &Player,
    mut targets: impl Iterator<Item = (usize, Player)>,
) -> Option<(usize, Player)> {
    targets.find(|(_, p)| p.position.is_adjacent(player.position))
}

impl Fight {
    /// Whether the fight is still going.
    pub fn is_ongoing(&self) -> bool {
        let elves = self
            .players
            .iter()
            .filter(|p| p.fealty == Team::Elf && p.is_alive())
            .count();
        let goblins = self
            .players
            .iter()
            .filter(|p| p.fealty == Team::Goblin && p.is_alive())
            .count();
        elves != 0 && goblins != 0
    }
    /// Runs the next round of combat.
    pub fn tick(&mut self) -> bool {
        self.players.sort_by(|a, b| a.position.cmp(&b.position));
        for i in 0..self.players.len() {
            let roster = self.players.clone();
            let mut actor = roster[i].clone();
            if !actor.is_alive() {
                continue;
            }
            let opposition = match actor.fealty {
                Team::Goblin => Team::Elf,
                Team::Elf => Team::Goblin,
            };
            let mut targets = roster
                .into_iter()
                .enumerate()
                .filter(|(_, p)| p.fealty == opposition && p.is_alive())
                .collect::<Vec<_>>();
            if targets.is_empty() {
                return false;
            }
            targets.sort_by(|a, b| {
                a.1.health
                    .cmp(&b.1.health)
                    .then(a.1.position.cmp(&b.1.position))
            });
            // Move if necessary/appropriate/possible.
            if let Some(point) = find_move(&self.grid, &actor, targets.clone().into_iter()) {
                self.players[i].position = point;
                self.grid.vacate(actor.position);
                self.grid.place(&actor, point);
                actor.position = point;
            }
            if let Some((index, _)) = find_attack(&actor, targets.into_iter()) {
                let target = &mut self.players[index];
                target.health = target.health.saturating_sub(actor.attack);
                if target.health == 0 {
                    self.grid.vacate(target.position);
                }
            }
        }
        true
    }
    /// Sets the attack power of the elves to something other than the default (3).
    pub fn upgrade_elves(&mut self, attack: u16) {
        for elf in self.players.iter_mut().filter(|p| p.fealty == Team::Elf) {
            elf.attack = attack;
        }
    }
}

impl FromStr for Fight {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut grid = Vec::new();
        let mut players = Vec::new();
        let mut axis = 0;
        let lines = s.lines().filter(|l| !l.is_empty()).enumerate();
        for (j, line) in lines {
            axis = line.len();
            for (i, c) in line.chars().enumerate() {
                match c {
                    'E' => {
                        let mut elf = Player::default();
                        elf.position = Point::new(i as u16, j as u16);
                        players.push(elf);
                        grid.push(Space::Occupied(Team::Elf));
                    }
                    'G' => {
                        let mut goblin = Player::default();
                        goblin.position = Point::new(i as u16, j as u16);
                        goblin.fealty = Team::Goblin;
                        players.push(goblin);
                        grid.push(Space::Occupied(Team::Goblin));
                    }
                    '#' => grid.push(Space::Wall),
                    '.' => grid.push(Space::Open),
                    _ => unreachable!(),
                }
            }
        }
        let axis = axis as u16;
        Ok(Fight {
            grid: Grid { spaces: grid, axis },
            players,
        })
    }
}

/// Returns the outcome from running the fight with default settings.
pub fn get_outcome(fight: &mut Fight) -> usize {
    let mut ticks = 0;
    while fight.tick() {
        ticks += 1;
    }
    let total_health = fight.players.iter().map(|p| p.health).sum::<u16>();
    ticks * (total_health as usize)
}

/// Returns the outcome from the fight in which the elves win decisively with minimal attack power.
pub fn optimize(fight: &Fight) -> usize {
    let elves = fight
        .players
        .iter()
        .filter(|p| p.fealty == Team::Elf)
        .count();
    for strength in 3.. {
        let mut fight = fight.clone();
        fight.upgrade_elves(strength);
        let mut ticks = 0;
        while fight.tick() {
            ticks += 1;
        }
        if fight
            .players
            .iter()
            .filter(|p| p.fealty == Team::Elf && p.is_alive())
            .count()
            == elves
        {
            let total_health = fight.players.iter().map(|p| p.health).sum::<u16>();
            return ticks * (total_health as usize);
        }
    }
    0
}

/// Solve the puzzle using the input in `puzzles/15.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/15.txt");
    let mut fight = input.parse::<Fight>().expect("Failed to parse.");
    let next = fight.clone();
    println!(
        "Day fifteen solutions: {}, {}",
        get_outcome(&mut fight),
        optimize(&next)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn above() {
        let one = Point::new(5, 1);
        let two = Point::new(140, 2);
        assert!(one < two);
    }
    #[test]
    fn below() {
        let one = Point::new(0, 2);
        let two = Point::new(12, 0);
        assert!(one > two);
    }
    #[test]
    fn left() {
        let one = Point::new(0, 0);
        let two = Point::new(1, 0);
        assert!(one < two);
    }
    #[test]
    fn right() {
        let one = Point::new(140, 0);
        let two = Point::new(12, 0);
        assert!(one > two);
    }
    #[test]
    fn combined() {
        let points = vec![
            Point::new(2, 1),
            Point::new(4, 1),
            Point::new(1, 2),
            Point::new(3, 2),
            Point::new(5, 2),
            Point::new(2, 3),
            Point::new(4, 3),
        ];
        let mut sorted = points.clone();
        sorted.sort();
        assert_eq!(points, sorted);
    }
    #[test]
    fn outcome() {
        let input = "#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######\n";
        let mut fight = input.parse::<Fight>().unwrap();
        assert_eq!(get_outcome(&mut fight), 27730);
        let input = "#######\n#G..#E#\n#E#E.E#\n#G.##.#\n#...#E#\n#...E.#\n#######\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(get_outcome(&mut fight), 36334);
        let input = "#######\n#E..EG#\n#.#G.E#\n#E.##E#\n#G..#.#\n#..E#.#\n#######\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(get_outcome(&mut fight), 39514);
        let input = "#######\n#E.G#.#\n#.#G..#\n#G.#.G#\n#G..#.#\n#...E.#\n#######\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(get_outcome(&mut fight), 27755);
        let input = "#######\n#.E...#\n#.#..G#\n#.###.#\n#E#G#G#\n#...#G#\n#######\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(get_outcome(&mut fight), 28944);
        let input = "#########\n#G......#\n#.E.#...#\n#..##..G#\n#...##..#\n#...#...#\n#.G...G.#\n#.....G.#\n#########\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(get_outcome(&mut fight), 18740);
    }
    #[test]
    fn optimal() {
        let input = "#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######\n";
        let mut fight = input.parse::<Fight>().unwrap();
        assert_eq!(optimize(&fight), 4988);
        let input = "#######\n#E..EG#\n#.#G.E#\n#E.##E#\n#G..#.#\n#..E#.#\n#######\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(optimize(&fight), 31284);
        let input = "#######\n#E.G#.#\n#.#G..#\n#G.#.G#\n#G..#.#\n#...E.#\n#######\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(optimize(&fight), 3478);
        let input = "#######\n#.E...#\n#.#..G#\n#.###.#\n#E#G#G#\n#...#G#\n#######\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(optimize(&fight), 6474);
        let input = "#########\n#G......#\n#.E.#...#\n#..##..G#\n#...##..#\n#...#...#\n#.G...G.#\n#.....G.#\n#########\n";
        fight = input.parse::<Fight>().unwrap();
        assert_eq!(optimize(&fight), 1140);
    }
}
