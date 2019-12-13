use std::str::FromStr;

fn fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn part1(modules: &[u32]) -> u32 {
    modules.iter().copied().map(fuel).sum()
}

fn part2(modules: &[u32]) -> u32 {
    modules
        .iter()
        .copied()
        .map(|module| {
            (1..)
                .scan(module, |state, _| {
                    let x = fuel(*state);
                    *state = x;
                    if x > 0 {
                        Some(x)
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

pub(crate) fn run(input: String) -> [String; 2] {
    let modules = input
        .lines()
        .filter_map(|x| u32::from_str(x).ok())
        .collect::<Vec<_>>();
    let part1 = format!("{}", part1(&modules));
    let part2 = format!("{}", part2(&modules));
    [part1, part2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        assert_eq!(part1(&[12]), 2);
        assert_eq!(part1(&[14]), 2);
        assert_eq!(part1(&[1969]), 654);
        assert_eq!(part1(&[100756]), 33583);
    }
    #[test]
    fn recursive() {
        assert_eq!(part2(&[14]), 2);
        assert_eq!(part2(&[1969]), 966);
        assert_eq!(part2(&[100756]), 50346);
    }
}
