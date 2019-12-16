use std::{
    collections::HashSet,
    convert::TryFrom,
    ops::{Add, Div, Mul},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
    fn distance(self) -> u16 {
        (self.x.abs() + self.y.abs()) as u16
    }
    fn decompose(self) -> Vec<Point> {
        assert!(self.x == 0 || self.y == 0);
        let magnitude = self.distance() as i16;
        vec![self / magnitude; magnitude as usize]
    }
    fn unit(direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(0, 1),
            Direction::Down => Self::new(0, -1),
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Self> for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i16> for Point {
    type Output = Self;
    fn mul(self, c: i16) -> Self::Output {
        Self {
            x: c * self.x,
            y: c * self.y,
        }
    }
}

impl Div<i16> for Point {
    type Output = Self;
    fn div(self, c: i16) -> Self::Output {
        Self {
            x: self.x / c,
            y: self.y / c,
        }
    }
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.parse::<Direction>()?;
        let amount = s[1..].parse::<i16>().map_err(|_| ())?;
        Ok(Point::unit(direction) * amount)
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'R' => Ok(Self::Right),
            'L' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().next().ok_or(()).and_then(Direction::try_from)
    }
}

fn part1(paths: &[Vec<Point>]) -> String {
    let closest = paths
        .iter()
        .map(|path| {
            path.iter()
                .flat_map(|pt| pt.decompose())
                .scan(Point::new(0, 0), |pos, delta| {
                    let new = *pos + delta;
                    *pos = new;
                    Some(new)
                })
                .collect::<HashSet<_>>()
        })
        .fold(HashSet::new(), |common, set| {
            if common.is_empty() {
                set
            } else {
                common.intersection(&set).copied().collect()
            }
        })
        .iter()
        .map(|pt| pt.distance())
        .min();
    format!("{}", closest.unwrap())
}

fn part2(paths: &[Vec<Point>]) -> String {
    let mut paths = paths.iter().map(|path| {
        path.iter()
            .flat_map(|pt| pt.decompose())
            .scan(Point::new(0, 0), |pos, delta| {
                let new = *pos + delta;
                *pos = new;
                Some(new)
            })
            .enumerate()
            .collect::<Vec<_>>()
    });
    let one = paths.next().unwrap();
    let two = paths.next().unwrap();
    for (i, p) in one {
        for (j, q) in two.iter() {
            if p == *q {
                return format!("{}", i + j + 2);
            }
        }
    }
    String::new()
}

pub(crate) fn run(input: String) -> [String; 2] {
    let points = input
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<Point>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    let closest = part1(&points);
    let fewest = part2(&points);
    [closest, fewest]
}
