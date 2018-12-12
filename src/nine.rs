//! Day nine (Marble Mania)

/// Parses the puzzle input to get the key parameters (players, marbles).
pub fn params(s: &str) -> (u32, u32) {
    let mut parts = s.split(' ').filter_map(|s| s.trim().parse::<u32>().ok());
    (
        parts.next().expect("Missing players"),
        parts.next().expect("Missing points"),
    )
}

/// Simulates a game with the specified number of players and maximum marble.
///
/// Returns which player wins and what their score is.
pub fn play(players: u32, max: u32) -> (usize, u32) {
    #[derive(Copy, Clone, Default)]
    struct Marble {
        value: u32,
        next: usize,
        prev: usize,
    }
    let mut board = Vec::with_capacity(players as usize + 1);
    board.push(Marble::default());
    let mut scores = vec![0; players as usize];
    let mut pos = 0;
    for (marble, elf) in (1..=max).zip((0..players).cycle()) {
        if marble % 23 == 0 {
            for _ in 0..7 {
                pos = board[pos].prev;
            }
            let m = board[pos];
            board[m.next].prev = m.prev;
            board[m.prev].next = m.next;
            scores[elf as usize] += marble + m.value;
            pos = m.next;
        } else {
            pos = board[pos].next;
            let next = board[pos].next;
            let prev = pos;
            let len = board.len();
            board.push(Marble {
                value: marble,
                next,
                prev,
            });
            board[next].prev = len;
            board[prev].next = len;
            pos = len;
        }
    }
    scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(i, v)| (i, *v))
        .unwrap_or_default()
}

/// Solve the puzzle using the input in `puzzles/9.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/9.txt");
    let (players, max) = params(&input);
    println!(
        "Day nine solutions: {}, {}",
        play(players, max).1,
        play(players, max * 100).1
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn case_one() {
        let input = "10 players; last marble is worth 1618 points";
        let params = params(&input);
        let (_, score) = play(params.0, params.1);
        assert_eq!(score, 8_317);
    }
    #[test]
    pub fn case_two() {
        let input = "13 players; last marble is worth 7999 points";
        let params = params(&input);
        let (_, score) = play(params.0, params.1);
        assert_eq!(score, 146_373);
    }
    #[test]
    pub fn case_three() {
        let input = "17 players; last marble is worth 1104 points";
        let params = params(&input);
        let (_, score) = play(params.0, params.1);
        assert_eq!(score, 2_764);
    }
    #[test]
    pub fn case_four() {
        let input = "21 players; last marble is worth 6111 points";
        let params = params(&input);
        let (_, score) = play(params.0, params.1);
        assert_eq!(score, 54_718);
    }
    #[test]
    pub fn case_five() {
        let input = "30 players; last marble is worth 5807 points";
        let params = params(&input);
        let (_, score) = play(params.0, params.1);
        assert_eq!(score, 37_305);
    }
}
