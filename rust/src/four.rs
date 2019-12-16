struct Digits {
    inner: Vec<u32>,
}

impl Digits {
    fn new(val: u32) -> Self {
        let inner = format!("{}", val)
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        Self { inner }
    }
}

impl Digits {
    fn windows(&self, size: usize) -> std::slice::Windows<u32> {
        self.inner.windows(size)
    }
}

impl Iterator for Digits {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}

trait DigitsExt: Sized {
    fn digits(self) -> Digits;
}

impl DigitsExt for u32 {
    fn digits(self) -> Digits {
        Digits::new(self)
    }
}

fn has_repeat(number: u32) -> bool {
    number.digits().windows(2).any(|xs| xs[0] == xs[1])
}

fn has_unique_repeat(number: u32) -> bool {
    number
        .digits()
        .map(|x| {
            number
                .digits()
                .skip_while(|d| *d != x)
                .take_while(|d| *d == x)
                .count()
        })
        .any(|l| l == 2)
}

fn has_six_digits(number: u32) -> bool {
    (100_000..1_000_000).contains(&number)
}

fn has_increasing_digits(number: u32) -> bool {
    number.digits().windows(2).all(|xs| xs[0] <= xs[1])
}

fn is_valid_password(number: u32) -> bool {
    has_repeat(number) && has_six_digits(number) & has_increasing_digits(number)
}

fn is_valid_ignoring_chunks(number: u32) -> bool {
    has_unique_repeat(number) && has_six_digits(number) & has_increasing_digits(number)
}

pub(crate) fn run(input: String) -> [String; 2] {
    let mut range = input
        .lines()
        .next()
        .unwrap()
        .split('-')
        .map(|x| x.parse::<u32>().unwrap());
    let (l, u) = (range.next().unwrap(), range.next().unwrap());
    let range = l..=u;
    let part1 = range.clone().filter(|x| is_valid_password(*x)).count();
    let part1 = format!("{}", part1);
    let part2 = range.filter(|x| is_valid_ignoring_chunks(*x)).count();
    let part2 = format!("{}", part2);
    [part1, part2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn passwords() {
        assert!(is_valid_password(111111));
        assert!(!is_valid_password(223450));
        assert!(!is_valid_password(123789));
    }
    #[test]
    fn passwords_chunks() {
        assert!(is_valid_ignoring_chunks(112233));
        assert!(is_valid_ignoring_chunks(111122));
        assert!(!is_valid_ignoring_chunks(123444));
    }
}
