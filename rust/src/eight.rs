use std::convert::TryFrom;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub(crate) fn run(input: String) -> [String; 2] {
    let pixels = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let part1 = part1(&pixels);
    let part2 = part2(&pixels);
    [part1, part2]
}

fn part1(pixels: &[u32]) -> String {
    let layer = pixels
        .chunks(WIDTH * HEIGHT)
        .min_by_key(|x| x.iter().filter(|i| **i == 0).count())
        .unwrap();
    let ones = layer.iter().filter(|i| **i == 1).count();
    let twos = layer.iter().filter(|i| **i == 2).count();
    format!("{}", ones * twos)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pixel {
    Transparent,
    White,
    Black,
}

impl Pixel {
    fn is_opaque(self) -> bool {
        !self.is_transparent()
    }
    fn is_transparent(self) -> bool {
        self == Self::Transparent
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Transparent => " ",
                Self::Black => "◼️",
                Self::White => "◻️",
            }
        )
    }
}

impl TryFrom<u32> for Pixel {
    type Error = ();
    fn try_from(x: u32) -> Result<Self, Self::Error> {
        match x {
            0 => Ok(Self::Black),
            1 => Ok(Self::White),
            2 => Ok(Self::Transparent),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Image {
    pixels: Vec<Pixel>,
    width: usize,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        dbg!(self.pixels.len());
        for row in 0..(self.pixels.len() / self.width) {
            for col in 0..self.width {
                let pixel = self[(row, col)];
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![Pixel::Transparent; width * height],
            width,
        }
    }
}

impl std::ops::Index<(usize, usize)> for Image {
    type Output = Pixel;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.pixels[row * self.width + col]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Image {
    fn index_mut(
        &mut self,
        (row, col): (usize, usize),
    ) -> &mut <Self as std::ops::Index<(usize, usize)>>::Output {
        &mut self.pixels[row * self.width + col]
    }
}

fn part2(pixels: &[u32]) -> String {
    let mut image = Image::new(WIDTH, HEIGHT);
    for layer in pixels.chunks(WIDTH * HEIGHT) {
        for col in 0..WIDTH {
            for row in 0..HEIGHT {
                let spec = Pixel::try_from(layer[row * WIDTH + col]).expect("Invalid color");
                if spec.is_opaque() {
                    if image[(row, col)].is_transparent() {
                        image[(row, col)] = spec;
                    }
                }
            }
        }
    }
    format!("\n{}", image)
}
