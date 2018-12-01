//! Day one (Chronal Calibration).

use std::collections::HashSet;

/// Run only the first part of the simulation. This is useful for inputs that yield diverging
/// series.
pub fn part_one(input: impl Iterator<Item = i64>) -> i64 {
    use std::ops::Add;
    input.fold(0, <i64 as Add>::add)
}

/// Run only the second part of the simulation.
pub fn part_two<I>(input: I) -> i64
where
    I: Iterator<Item = i64> + Clone,
{
    let mut seen: HashSet<i64> = HashSet::new();
    seen.insert(0);
    let mut sum = 0;
    for next in input.cycle() {
        sum += next;
        if !seen.insert(sum) {
            break;
        }
    }
    sum
}

/// Run both parts of the simulation.
pub fn simulate(input: &str) -> [i64; 2] {
    let parsed = input.split("\n").filter_map(|v| v.parse::<i64>().ok());
    let first = part_one(parsed.clone());
    let second = part_two(parsed);
    [first, second]
}

/// Solve the puzzle using the input in `puzzles/1.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/1.txt");
    let result = simulate(input);
    println!("Day one solutions: {}, {}", result[0], result[1]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_one() {
        let input = "+1\n-2\n+3\n+1\n";
        let result = simulate(input);
        assert_eq!(result[0], 3);
        assert_eq!(result[1], 2);
    }
    #[test]
    fn case_two() {
        let input = "+1\n+1\n+1\n";
        let parsed = input.split("\n").filter_map(|v| v.parse::<i64>().ok());
        assert_eq!(part_one(parsed), 3);
    }
    #[test]
    fn case_three() {
        let input = "+1\n+1\n-2\n";
        assert_eq!(simulate(input)[0], 0);
    }
    #[test]
    fn case_four() {
        let input = "-1\n-2\n-3\n";
        let parsed = input.split("\n").filter_map(|v| v.parse::<i64>().ok());
        assert_eq!(part_one(parsed), -6);
    }
    #[test]
    fn case_five() {
        let input = "+1\n-1\n";
        assert_eq!(simulate(input)[1], 0);
    }
    #[test]
    fn case_six() {
        let input = "+3\n+3\n+4\n-2\n-4\n";
        assert_eq!(simulate(input)[1], 10);
    }
    #[test]
    fn case_seven() {
        let input = "-6\n+3\n+8\n+5\n-6\n";
        assert_eq!(simulate(input)[1], 5);
    }
    #[test]
    fn case_eight() {
        let input = "+7\n+7\n-2\n-7\n-4\n";
        assert_eq!(simulate(input)[1], 14);
    }
}
