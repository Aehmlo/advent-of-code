//! Day two (Inventory Management System)

use std::collections::HashMap;

/// Returns whether the given string has any characters repeated twice or thrice.
pub fn has_reps(id: &str) -> (bool, bool) {
    let mut seen = HashMap::with_capacity(26);
    for c in id.chars() {
        *seen.entry(c).or_insert(0) += 1;
    }
    let double = seen.values().any(|&n| n == 2);
    let triple = seen.values().any(|&n| n == 3);
    (double, triple)
}

/// Part 1: calculate a checksum based on repeated (twice and thrice) digits.
pub fn checksum<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    let mut twos = 0;
    let mut threes = 0;
    for (d, t) in input.map(|i| has_reps(i)) {
        if d {
            twos += 1;
        }
        if t {
            threes += 1;
        }
    }
    twos * threes
}

/// Given two strings, returns whether they differ by `n` characters.
pub fn differ_by(n: usize, a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() == n
}

/// Returns the common letters (in common positions) of the two given strings.
pub fn common_letters(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

/// Part 2: Find IDs which differ by a single position and return the common part.
pub fn find_close(input: &str) -> Option<String> {
    for (pos, id) in input.lines().enumerate() {
        for other in input.lines().skip(pos + 1) {
            if differ_by(1, id, other) {
                return Some(common_letters(id, other));
            }
        }
    }
    None
}

/// Solve the puzzle using the input in `puzzles/2.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/2.txt");
    println!(
        "Day two solutions: {}, {}",
        checksum(input.lines()),
        find_close(&input).expect("Failed to solve part 2.")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_part_one() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n";
        assert_eq!(checksum(input.lines()), 12);
    }
    #[test]
    fn sample_part_two() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!(&find_close(&input).unwrap(), "fgij");
    }
}
