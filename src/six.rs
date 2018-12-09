//! Day six (Chronal Coordinates)

/// Represents a point on the grid.
pub type Point = (u16, u16);

/// Returns the Manhattan distance between two points.
pub fn distance(one: &Point, other: &Point) -> u16 {
    ((one.0 as i16 - other.0 as i16).abs() + (one.1 as i16 - other.1 as i16).abs()) as u16
}

/// Convenience method to convert an iterator over lines into a vector of points.
pub fn points<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<Point> {
    iter.map(|l| {
        let mut parts = l.split(',').map(|c| c.trim().parse().unwrap());
        let point: Point = (parts.next().unwrap(), parts.next().unwrap());
        point
    }).collect::<Vec<_>>()
}

/// Convenience method to get the bounding box of the grid.
pub fn bounds(points: &mut Vec<Point>) -> (Point, Point) {
    points.sort_by(|a, b| a.1.cmp(&b.1));
    let min_y = points.first().unwrap().1;
    let max_y = points.last().unwrap().1;
    points.sort();
    let min_x = points.first().unwrap().0;
    let max_x = points.last().unwrap().0;
    ((min_x, min_y), (max_x, max_y))
}

/// Finds the largest (non-infinite) area.
pub fn find_biggest(points: &mut Vec<Point>) -> u16 {
    let ((min_x, min_y), (max_x, max_y)) = bounds(points);
    let mut areas: Vec<Option<u16>> = vec![Some(0); points.len()];
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            let point: Point = (i, j);
            let mut distances = points
                .iter()
                .enumerate()
                .map(|(i, p)| (distance(&p, &point), i))
                .collect::<Vec<_>>();
            distances.sort();
            if distances[0].0 != distances[1].0 {
                if i == min_x || i == max_x || j == min_y || j == max_y {
                    areas[distances[0].1] = None;
                } else {
                    let i = distances[0].1;
                    let area = areas[i];
                    if area.is_some() {
                        let a = Some(area.unwrap() + 1);
                        areas[i] = a;
                    }
                }
            }
        }
    }
    areas.sort();
    areas.last().unwrap().unwrap()
}

/// Returns all points with a total distance of `delta` or less.
pub fn close_points(points: &mut Vec<Point>, delta: u16) -> usize {
    let ((min_x, min_y), (max_x, max_y)) = bounds(points);
    let mut total = 0;
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            let distance: u16 = points.iter().map(|p| distance(&p, &(i, j))).sum();
            if distance < delta {
                total += 1;
            }
        }
    }
    total
}

/// Solve the puzzle using the input in `puzzles/6.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/6.txt");
    let mut points = points(input.lines());
    let area = find_biggest(&mut points);
    let count = close_points(&mut points, 10_000);
    println!("Day six solutions: {}, {}", area, count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n";
        let mut points = points(input.lines());
        let area = find_biggest(&mut points);
        assert_eq!(area, 17);
        assert_eq!(close_points(&mut points, 32), 16);
    }
}
