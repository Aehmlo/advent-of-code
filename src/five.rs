//! Day five (Alchemical Reduction)

/// Returns whether a pair of monomers is reactive (will annihilate).
pub fn pair_reacts(one: &char, other: &char) -> bool {
    one != other && one.to_ascii_lowercase() == other.to_ascii_lowercase()
}

/// Simulates the polymerization reaction.
pub fn polymerize(structure: &str) -> String {
    let mut polymer = String::new();
    for c in structure.chars() {
        if let Some(l) = polymer.chars().last() {
            if pair_reacts(&l, &c) {
                polymer.pop();
            } else {
                polymer.push(c);
            }
        } else {
            polymer.push(c);
        }
    }
    polymer
}

/// Returns all primary structures resulting from removing all monomers of one type.
pub fn mutate(reference: &str) -> Vec<String> {
    let mut structures = Vec::new();
    for i in 0..26 {
        let c = (97u8 + i) as char;
        let structure = reference
            .chars()
            .filter(|e| e.to_ascii_lowercase() != c)
            .collect();
        structures.push(structure);
    }
    structures
}

/// Try mutating the input, seeing what gives the shortest resultant polymer.
pub fn shortest_len(reference: &str) -> usize {
    let others = mutate(&reference);
    let mut shortest = reference.len();
    for p in others {
        let p = polymerize(&p);
        let len = p.len();
        if len < shortest {
            shortest = len;
        }
    }
    shortest
}

/// Solve the puzzle using the input in `puzzles/5.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/5.txt");
    let input = input.lines().next().expect("No lines in input file.");
    let polymer = polymerize(&input);
    println!(
        "Day five solutions: {}, {}",
        polymer.len(),
        shortest_len(&input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn polymerization() {
        assert_eq!(polymerize("aA"), "");
        assert_eq!(polymerize("abBA"), "");
        assert_eq!(polymerize("abAB"), "abAB");
        assert_eq!(polymerize("aabAAB"), "aabAAB");
        assert_eq!(polymerize("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }
    #[test]
    fn optimization() {
        assert_eq!(shortest_len("dabAcCaCBAcCcaDA"), 4);
    }
}
