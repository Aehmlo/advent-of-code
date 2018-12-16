//! Day fourteen (Chocolate Charts)

type Score = usize;
type Index = usize;

enum Combined {
    Two(Score, Score),
    One(Score),
}

fn combine(one: Score, two: Score) -> Combined {
    let sum = one + two;
    let ones = sum % 10;
    let tens = sum / 10;
    if tens > 0 {
        Combined::Two(tens, ones)
    } else {
        Combined::One(ones)
    }
}

fn extend(board: &mut Vec<Score>, tail: Combined) {
    match tail {
        Combined::One(one) => board.push(one),
        Combined::Two(one, two) => {
            board.push(one);
            board.push(two);
        }
    }
}

fn tick(one: Index, two: Index, board: &mut Board) -> (Index, Index) {
    let (a, b) = (board.0[one], board.0[two]);
    let result = combine(a, b);
    extend(&mut board.0, result);
    ((a + one + 1) % board.0.len(), (b + two + 1) % board.0.len())
}

struct Board(Vec<Score>);

impl Default for Board {
    fn default() -> Self {
        Board(vec![3, 7])
    }
}

fn after(board: &mut Board, pos: usize) -> usize {
    let mut one = 0;
    let mut two = 1;
    while board.0.len() < pos + 11 {
        let new = tick(one, two, board);
        one = new.0;
        two = new.1;
    }
    board.0[pos..(pos + 10)]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10_usize.pow(i as u32))
        .sum::<usize>()
}

fn before(board: &mut Board, target: usize) -> usize {
    let mut one = 0;
    let mut two = 1;
    let digits = (target as f64).log(10.0).ceil() as usize;
    let old = board.0.len();
    loop {
        let new = tick(one, two, board);
        one = new.0;
        two = new.1;
        if board.0.len() < digits {
            continue;
        }
        let thing1 = board
            .0
            .iter()
            .rev()
            .take(digits)
            .enumerate()
            .map(|(i, d)| d * 10_usize.pow(i as u32))
            .sum::<usize>();
        if thing1 == target {
            break board.0.len() - digits;
        }
        if board.0.len() - old > 1 {
            let thing = board
                .0
                .iter()
                .rev()
                .skip(1)
                .take(digits)
                .enumerate()
                .map(|(i, d)| d * 10_usize.pow(i as u32))
                .sum::<usize>();
            if thing == target {
                break board.0.len() - digits - 1;
            }
        }
    }
}

/// Solve the puzzle using the input in `puzzles/14.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/14.txt");
    let spec = input.lines().next().unwrap().parse::<usize>().unwrap();
    let mut board = Board::default();
    let res = after(&mut board, spec);
    board = Board::default();
    let reps = before(&mut board, spec);
    println!("Day fourteen solutions: {}, {}", res, reps);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn forward() {
        let mut board = Board::default();
        assert_eq!(after(&mut board, 9), 5158916779);
        board = Board::default();
        assert_eq!(after(&mut board, 5), 0124515891);
        board = Board::default();
        assert_eq!(after(&mut board, 18), 9251071085);
        board = Board::default();
        assert_eq!(after(&mut board, 2018), 5941429882);
    }
    #[test]
    fn backward() {
        let mut board = Board::default();
        assert_eq!(before(&mut board, 51589), 9);
        board = Board::default();
        assert_eq!(before(&mut board, 92510), 18);
        board = Board::default();
        assert_eq!(before(&mut board, 59414), 2018);
    }
}
