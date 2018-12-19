//! Day nineteen (Go With The Flow)

use std::str::FromStr;

const REPS: usize = 10;

/// Represents our time-travel device's bizarre CPU.
#[derive(Default)]
pub struct Cpu {
    registers: [usize; 6],
    ip: Option<usize>,
}

impl Cpu {
    /// Sets the instruction pointer register.
    pub fn ip(&mut self, register: usize) {
        self.ip = Some(register);
    }
    /// Stores into register `target` the contents of register `source`.
    pub fn setr(&mut self, target: usize, source: usize) {
        let value = self.registers[source];
        self.seti(target, value);
    }
    /// Stores `value` into register `register`.
    pub fn seti(&mut self, register: usize, value: usize) {
        self.registers[register] = value;
    }
    /// Adds contents of registers `a` and `b` and stores the result in register `target`.
    pub fn addr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.addi(a, value, target);
    }
    /// Adds `value` to contents of `a`, storing the result in register `target`.
    pub fn addi(&mut self, a: usize, value: usize, target: usize) {
        let init = self.registers[a];
        self.seti(target, init + value);
    }
    /// Multiplies the contents of registers `a` and `b`, storing the result in register
    /// `target`.
    pub fn mulr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.muli(a, value, target);
    }
    /// Multiplies `value` and the contents of register `a`, storing the result in register
    /// `target`.
    pub fn muli(&mut self, a: usize, value: usize, target: usize) {
        let init = self.registers[a];
        self.seti(target, init * value);
    }
    /// Performs a bitwise and of the contents of registers `a` and `b`, storing the result in
    /// register `target`.
    pub fn banr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.bani(a, value, target);
    }
    /// Performs a bitwise and of `value` and the contents of register `a`, storing the result in
    /// register `target`.
    pub fn bani(&mut self, a: usize, value: usize, target: usize) {
        let init = self.registers[a];
        self.seti(target, init & value);
    }
    /// Performs a bitwise or of the contents of registers `a` and `b`, storing the result in
    /// register `target`.
    pub fn borr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.bori(a, value, target);
    }
    /// Performs a bitwise or of `value` and the contents of register `a`, storing the result in
    /// register `target`.
    pub fn bori(&mut self, a: usize, value: usize, target: usize) {
        let init = self.registers[a];
        self.seti(target, init | value);
    }
    /// Checks whether the contents of register `a` is greater than the value `b`, storing the
    /// result of the comparison (`1` or `0`) in `target`.
    pub fn gtri(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[a];
        self.seti(target, (value > b) as usize);
    }
    /// Checks whether the value `a` is greater than the contents of register `b`, storing the
    /// result of the comparison (`1` or `0`) in `target`.
    pub fn gtir(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.seti(target, (a > value) as usize);
    }
    /// Checks whether the contents of register `a` is greater than the contents of register
    /// `b`, storing the result of the comparison (`1` or `0`) in `target`.
    pub fn gtrr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.gtri(a, value, target);
    }
    /// Checks whether the value `a` is equal to the contents of register `b`, storing the
    /// result of the comparison (`1` or `0`) in `target`.
    pub fn eqri(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.seti(target, (value == a) as usize);
    }
    /// Checks whether the contents of register `a` is equal to the value `b`, storing the
    /// result of the comparison (`1` or `0`) in `target`.
    pub fn eqir(&mut self, a: usize, b: usize, target: usize) {
        self.eqri(b, a, target);
    }
    /// Checks whether the contents of register `a` is equal to the contents of register `b`,
    /// storing the result of the comparison (`1` or `0`) in `target`.
    pub fn eqrr(&mut self, a: usize, b: usize, target: usize) {
        let a = self.registers[a];
        let b = self.registers[b];
        self.seti(target, (a == b) as usize);
    }
    /// Runs the given program. If `early == true`, the value in register 3 is returned once
    /// stable.
    pub fn run(&mut self, program: &Program, early: bool) -> usize {
        let mut stable = 0;
        let mut asymptote = 0;
        let mut i = 0;
        let mut program = program.clone();
        let first = program.instructions.remove(0);
        self.ip(first.a);
        while let Some(instruction) = program.instructions.get(i) {
            self.registers[self.ip.unwrap()] = i;
            match instruction.op.as_str() {
                "#ip" => self.ip(instruction.a),
                "setr" => self.setr(instruction.c.unwrap(), instruction.a),
                "seti" => self.seti(instruction.c.unwrap(), instruction.a),
                "addr" => self.addr(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "addi" => self.addi(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "mulr" => self.mulr(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "muli" => self.muli(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "banr" => self.banr(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "bani" => self.bani(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "borr" => self.borr(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "bori" => self.bori(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "gtri" => self.gtri(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "gtir" => self.gtir(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "gtrr" => self.gtrr(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "eqri" => self.eqri(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "eqir" => self.eqir(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                "eqrr" => self.eqrr(
                    instruction.a,
                    instruction.b.unwrap(),
                    instruction.c.unwrap(),
                ),
                _ => unreachable!(),
            }
            i = self.registers[self.ip.unwrap()];
            i += 1;
            // This is all hacked in, and I'm sure it's different registers for other inputs, but
            // I don't want to write the code to do this by hand.
            if early {
                let value = self.registers[3];
                if value != 0 {
                    if asymptote == value {
                        stable += 1;
                        if stable >= REPS {
                            return asymptote;
                        }
                    } else {
                        asymptote = self.registers[2];
                        stable = 0;
                    }
                }
            }
        }
        self.registers[0]
    }
}

#[derive(Clone)]
struct Instruction {
    op: String,
    a: usize,
    b: Option<usize>,
    c: Option<usize>,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut parts = s.split(' ');
        let op = parts.next().expect("No operation").to_string();
        let a = parts
            .next()
            .expect("No parameters.")
            .parse::<usize>()
            .unwrap();
        let b = parts.next().map(|s| s.parse::<usize>().unwrap());
        let c = parts.next().map(|s| s.parse::<usize>().unwrap());
        Ok(Self { op, a, b, c })
    }
}

/// Represents a program to be run on a CPU.
#[derive(Clone)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let instructions = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self { instructions })
    }
}

/// Solve the puzzle using the input in `puzzles/19.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/19.txt");
    let mut cpu = Cpu::default();
    let program = input.parse::<Program>().unwrap();
    let res = cpu.run(&program, false);
    cpu.registers = [1, 0, 0, 0, 0, 0];
    let m = cpu.run(&program, true);
    let sum = (1..=(m / 2)).filter(|f| m % f == 0).sum::<usize>() + m;
    println!("Day nineteen solutions: {}, {}", res, sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input() {
        let input = "#ip 0\nseti 5 0 1\nseti 6 0 2\naddi 0 1 0\naddr 1 2 3\nsetr 1 0 0\nseti 8 0 4\nseti 9 0 5";
        let mut cpu = Cpu::default();
        let program = input.parse::<Program>().unwrap();
        let res = cpu.run(&program, false);
        assert_eq!(res, 6);
    }
}
