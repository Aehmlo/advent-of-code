use std::{
    collections::VecDeque,
    convert::{TryFrom, TryInto},
};

struct Digits {
    inner: VecDeque<u32>,
}

impl Digits {
    fn new(val: u32) -> Self {
        let inner = format!("{}", val)
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<VecDeque<_>>();
        Self { inner }
    }
}

impl Iterator for Digits {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_front()
    }
}

impl DoubleEndedIterator for Digits {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.pop_back()
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

enum Status {
    Running,
    Stopped,
}

#[derive(Clone)]
struct Computer {
    pub(super) memory: Vec<i32>,
    cursor: usize,
    input: i32,
    output: Vec<i32>,
}

#[derive(Debug)]
enum Error {
    SignedOpcode,
    UnknownOpcode(u32),
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
        Self {
            memory,
            cursor: 0,
            input: 1,
            output: Vec::new(),
        }
    }
    fn args(&self, op: &Operation) -> Vec<i32> {
        let size = op.kind.arg_count();
        if size == 0 {
            Vec::new()
        } else {
            let mut modes = op.modes.clone();
            let memory = &self.memory[(self.cursor + 1)..=(self.cursor + size)];
            while modes.len() < memory.len() {
                modes.push(Mode::default());
            }
            let mut args = memory
                .iter()
                .zip(modes)
                .map(|(arg, mode)| match mode {
                    Mode::Immediate => *arg,
                    Mode::Position => self.memory[*arg as usize],
                })
                .collect::<Vec<_>>();
            if let Some(index) = op.kind.protected() {
                args[index] = memory[index];
            }
            args
        }
    }
    fn step(&mut self) -> Result<Status, Error> {
        let opcode: u32 = self.memory[self.cursor].try_into()?;
        let op = opcode.try_into()?;
        let args = self.args(&op);
        let width = args.len();
        let mut update_cursor = true;
        match op.kind {
            OperationKind::Add => {
                let dst = args[2] as usize;
                self.memory[dst] = args[0] + args[1];
            }
            OperationKind::Mul => {
                let dst = args[2] as usize;
                self.memory[dst] = args[0] * args[1];
            }
            OperationKind::Store => {
                let dst = args[0] as usize;
                self.memory[dst] = self.input;
            }
            OperationKind::Return => {
                self.output.push(args[0]);
            }
            OperationKind::JumpIf => {
                if args[0] != 0 {
                    self.cursor = args[1] as usize;
                    update_cursor = false;
                }
            }
            OperationKind::JumpIfNot => {
                if args[0] == 0 {
                    self.cursor = args[1] as usize;
                    update_cursor = false;
                }
            }
            OperationKind::Lt => {
                self.memory[args[2] as usize] = if args[0] < args[1] { 1 } else { 0 };
            }
            OperationKind::Eq => {
                self.memory[args[2] as usize] = if args[0] == args[1] { 1 } else { 0 };
            }
            OperationKind::Halt => return Ok(Status::Stopped),
        };
        // This doesn't run for the Hcf opcode, so it'll always return Stopped
        if update_cursor {
            self.cursor += width + 1;
        }
        Ok(Status::Running)
    }
    fn run(&mut self) -> Result<&[i32], Error> {
        while let Ok(Status::Running) = self.step() {}
        Ok(&self.memory)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Immediate,
    Position,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Position
    }
}

impl TryFrom<u32> for Mode {
    type Error = ();
    fn try_from(num: u32) -> Result<Self, Self::Error> {
        match num {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Operation {
    pub(super) kind: OperationKind,
    pub(super) modes: Vec<Mode>,
}

#[derive(Clone, Copy, Debug)]
enum OperationKind {
    Add,
    Mul,
    Halt,
    Store,
    Return,
    JumpIf,
    JumpIfNot,
    Lt,
    Eq,
}

impl OperationKind {
    fn arg_count(self) -> usize {
        match self {
            Self::Store | Self::Return => 1,
            Self::JumpIf | Self::JumpIfNot => 2,
            Self::Add | Self::Mul | Self::Lt | Self::Eq => 3,
            Self::Halt => 0,
        }
    }
    fn protected(self) -> Option<usize> {
        match self {
            Self::Add | Self::Mul | Self::Eq | Self::Lt => Some(2),
            Self::Store => Some(0),
            Self::Return | Self::Halt | Self::JumpIf | Self::JumpIfNot => None,
        }
    }
}

#[derive(Debug)]
struct TryFromError {
    opcode: u32,
}

impl std::fmt::Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unknown opcode: {}", self.opcode)
    }
}

impl std::error::Error for TryFromError {}

impl TryFrom<u32> for Operation {
    type Error = TryFromError;
    fn try_from(opcode: u32) -> Result<Self, Self::Error> {
        let kind = (match opcode % 100 {
            1 => Ok(OperationKind::Add),
            2 => Ok(OperationKind::Mul),
            3 => Ok(OperationKind::Store),
            4 => Ok(OperationKind::Return),
            5 => Ok(OperationKind::JumpIf),
            6 => Ok(OperationKind::JumpIfNot),
            7 => Ok(OperationKind::Lt),
            8 => Ok(OperationKind::Eq),
            99 => Ok(OperationKind::Halt),
            _ => Err(TryFromError { opcode }),
        })?;
        let modes = opcode
            .digits()
            .rev()
            .skip(2)
            .map(|digit| digit.try_into().unwrap_or_default())
            .collect();
        Ok(Self { kind, modes })
    }
}

fn run_diagnostic(memory: &[i32], id: i32) -> String {
    let memory = memory.iter().copied().collect();
    let mut comp = Computer::new(memory);
    comp.input = id;
    comp.run().unwrap();
    let mut output = comp.output;
    let last = output.pop().expect("No output");
    for o in output {
        assert_eq!(o, 0);
    }
    format!("{}", last)
}

fn part1(memory: &[i32]) -> String {
    run_diagnostic(memory, 1)
}

fn part2(memory: &[i32]) -> String {
    run_diagnostic(memory, 5)
}

pub(crate) fn run(input: String) -> [String; 2] {
    let memory = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| str::parse::<i32>(x).ok())
        .collect::<Vec<_>>();
    let one = part1(&memory);
    let five = part2(&memory);
    [one, five]
}
