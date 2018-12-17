//! Day eleven (Chronal Charge)

use std::ops::{Index, IndexMut};

const AXIS: usize = 300;

/// A (300x300) power grid of some sort.
pub struct Grid {
    /// The serial number of the grid.
    pub serial: u32,
    cells: [i32; AXIS * AXIS],
}

/// Returns the power level for a given point and serial number.
pub fn power(serial: u32, x: u32, y: u32) -> i32 {
    let rack = x + 10;
    let p = rack * ((rack * y) + serial);
    ((p % 1000) / 100) as i32 - 5
}

impl Grid {
    /// Creates a new grid with the given seed (serial number);
    pub fn populate(serial: u32) -> Self {
        let mut grid = Grid {
            serial: 0,
            cells: [0; AXIS * AXIS],
        };
        for j in 0..AXIS {
            let row = &mut grid[j];
            let y = (j + 1) as u32;
            for (i, v) in row.iter_mut().take(AXIS).enumerate() {
                let x = (i + 1) as u32;
                *v = power(serial, x, y);
            }
        }
        grid
    }
    /// Returns the (1-indexed) location and value of the highest power level.
    pub fn max(&self) -> ((u32, u32), i32) {
        let max = self.cells.iter().enumerate().max_by(|a, b| a.1.cmp(&b.1));
        if let Some((index, value)) = max {
            let index = index as u32;
            let axis = AXIS as u32;
            let x = index % axis + 1;
            let y = index / axis;
            ((x, y), *value)
        } else {
            ((1, 1), self.cells[0])
        }
    }
    /// Returns the (1-indexed) origin of the nxn square with the highest total power (and the
    /// power).
    pub fn max_sized(&self, n: usize) -> ((usize, usize), i32) {
        let mut highest = i32::min_value();
        let mut point = (1, 1);
        for j in 0..=(AXIS - n) {
            for i in 0..=(AXIS - n) {
                let mut sum: i32 = self[j][i..(i + n)].iter().sum();
                for k in 1..n {
                    sum += self[j + k][i..(i + n)].iter().sum::<i32>();
                }
                if sum > highest {
                    highest = sum;
                    point = (i + 1, j + 1);
                }
            }
        }
        (point, highest)
    }
    /// Finds the nxn square with the highest total power.
    pub fn max_power_square(&self) -> ((usize, usize), usize, i32) {
        let mut max = i32::min_value();
        let mut pos = (1, 1);
        let mut size = 1;
        for n in 1..=AXIS {
            let (p, v) = self.max_sized(n);
            if v > max {
                max = v;
                pos = p;
                size = n
            }
        }
        (pos, size, max)
    }
}

impl Index<usize> for Grid {
    type Output = [i32];
    /// Returns a row from the grid.
    fn index(&self, index: usize) -> &Self::Output {
        let index = index as usize;
        let start = index * AXIS;
        let next = (index + 1) * AXIS;
        &self.cells[start..next]
    }
}

impl IndexMut<usize> for Grid {
    /// Returns a row from the grid.
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        let index = index as usize;
        let start = index * AXIS;
        let next = (index + 1) * AXIS;
        &mut self.cells[start..next]
    }
}

/// Solve the puzzle using the input in `puzzles/11.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/11.txt");
    let mut lines = input.lines();
    let serial = lines
        .next()
        .expect("No serial number given.")
        .parse()
        .expect("Non-numeric grid serial number.");
    let grid = Grid::populate(serial);
    let (origin, _) = grid.max_sized(3);
    let (point, size, _) = grid.max_power_square();
    println!(
        "Day eleven solutions: {},{}; {},{},{}",
        origin.0, origin.1, point.0, point.1, size
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn power_level() {
        assert_eq!(power(8, 3, 5), 4);
        assert_eq!(power(57, 122, 79), -5);
        assert_eq!(power(39, 217, 196), 0);
        assert_eq!(power(71, 101, 153), 4);
    }
    /*#[test]
    fn square() {
        let grid = Grid::populate(18);
        let (origin, size, power) = grid.max_power_square();
        assert_eq!(origin, (90, 269));
        assert_eq!(size, 16);
        assert_eq!(power, 113);
    }*/
}
