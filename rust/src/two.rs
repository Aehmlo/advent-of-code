use std::convert::{TryFrom, TryInto};

static EMPTY: &'static [i32] = &[];

enum Status {
    Running,
    Stopped,
}

#[derive(Clone)]
struct Computer {
    memory: Vec<i32>,
    cursor: usize,
}

#[derive(Debug)]
enum Error {
    SignedOpcode,
    UnknownOpcode(u8),
}

impl From<TryFromError> for Error {
    fn from(err: TryFromError) -> Self {
        Self::UnknownOpcode(err.opcode)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self::SignedOpcode
    }
}

impl Computer {
    fn new(memory: Vec<i32>) -> Self {
        Self { memory, cursor: 0 }
    }
    fn args(&self, op: Operation) -> &[i32] {
        let size = op.arg_count();
        if size > 0 {
            &self.memory[(self.cursor + 1)..=(self.cursor + size)]
        } else {
            EMPTY
        }
    }
    fn step(&mut self) -> Result<Status, Error> {
        let opcode: u8 = self.memory[self.cursor].try_into()?;
        let op = opcode.try_into()?;
        let args = self.args(op);
        let width = args.len();
        match op {
            Operation::Add => {
                let dst = args[2] as usize;
                self.memory[dst] = self.memory[args[0] as usize] + self.memory[args[1] as usize];
            }
            Operation::Mul => {
                let dst = args[2] as usize;
                self.memory[dst] = self.memory[args[0] as usize] * self.memory[args[1] as usize];
            }
            Operation::Hcf => return Ok(Status::Stopped),
        };
        // This doesn't run for the Hcf opcode, so it'll always return Stopped
        self.cursor += width + 1;
        Ok(Status::Running)
    }
    fn run(&mut self) -> Result<&[i32], Error> {
        while let Ok(Status::Running) = self.step() {}
        Ok(&self.memory)
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Hcf,
}

impl Operation {
    fn arg_count(self) -> usize {
        match self {
            Self::Add | Self::Mul => 3,
            Self::Hcf => 0,
        }
    }
}

#[derive(Debug)]
struct TryFromError {
    opcode: u8,
}

impl std::fmt::Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unknown opcode: {}", self.opcode)
    }
}

impl std::error::Error for TryFromError {}

impl TryFrom<u8> for Operation {
    type Error = TryFromError;
    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        match opcode {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            99 => Ok(Self::Hcf),
            _ => Err(TryFromError { opcode }),
        }
    }
}

fn part1(memory: &[i32]) -> String {
    let mut program = memory.iter().copied().collect::<Vec<_>>();
    program[1] = 12;
    program[2] = 2;
    let mut comp = Computer::new(program);
    let memory = comp.run().unwrap();
    format!("{}", memory[0])
}

fn part2(memory: &[i32], target: i32) -> String {
    let program = memory.iter().copied().collect::<Vec<_>>();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = program.clone();
            memory[1] = noun;
            memory[2] = verb;
            let mut comp = Computer::new(memory);
            if let Ok(memory) = comp.run() {
                if memory[0] == target {
                    return format!("{}", 100 * noun + verb);
                }
            }
        }
    }
    String::new()
}

pub(crate) fn run(input: String) -> [String; 2] {
    let program = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| str::parse::<i32>(x).ok())
        .collect::<Vec<_>>();
    let root = part1(&program);
    let comb = part2(&program, 19_690_720);
    [root, comb]
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run_program(memory: Vec<i32>) -> i32 {
        let mut comp = Computer::new(memory);
        comp.run().unwrap()[0]
    }
    #[test]
    fn part1() {
        assert_eq!(
            run_program(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            3500
        );
        assert_eq!(run_program(vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
    }
}
