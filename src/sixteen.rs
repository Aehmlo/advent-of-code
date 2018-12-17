//! Day sixteen (Chronal Classification)

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Index;
use std::str::FromStr;

/// Represents an instruction issued to the processor.
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut parts = s.split(' ').map(|p| {
            p.trim()
                .parse::<usize>()
                .expect("Non-numeric digit encountered.")
        });
        Ok(Self {
            opcode: parts.next().expect("No opcode."),
            a: parts.next().expect("No a."),
            b: parts.next().expect("No b."),
            c: parts.next().expect("No c."),
        })
    }
}

/// Represents an action taken by the processor.
///
/// Actions consist of an instruction, an input state, and an output state.
struct Action {
    before: Processor,
    act: Instruction,
    after: Processor,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut lines = s.lines();
        let before = lines
            .next()
            .expect("No initial state.")
            .split(':')
            .nth(1)
            .expect("No second part of initial state.")
            .parse::<Processor>()
            .unwrap();
        let act = lines
            .next()
            .expect("No action.")
            .parse::<Instruction>()
            .unwrap();
        let after = lines
            .next()
            .expect("No initial state.")
            .split(':')
            .nth(1)
            .expect("No second part of final state.")
            .parse::<Processor>()
            .unwrap();
        Ok(Self { before, act, after })
    }
}

/// Represents the weird processor we're working with.
///
/// Luckily, we can simulate this thing in software!
///
/// To read the registers of the processor, use its
/// [`Index` implementation](#impl-Index%3Cusize%3E). To mutate them, use the methods below.
#[derive(Clone, Eq, PartialEq)]
pub struct Processor {
    /// The underlying registers of the processor.
    registers: [usize; 4],
}

impl Processor {
    /// Stores into register `target` the contents of register `source`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 17);
    /// processor.setr(1, 0);
    /// assert_eq!(processor[1], 17);
    /// ```
    pub fn setr(&mut self, target: usize, source: usize) {
        let value = self.registers[source];
        self.seti(target, value);
    }
    /// Stores `value` into register `register`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 23);
    /// assert_eq!(processor[0], 23);
    /// ```
    pub fn seti(&mut self, register: usize, value: usize) {
        self.registers[register] = value;
    }
    /// Adds contents of registers `a` and `b` and stores the result in register `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 17);
    /// processor.seti(1, 12);
    /// processor.addr(0, 1, 2);
    /// assert_eq!(processor[2], 29);
    /// ```
    pub fn addr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.addi(a, value, target);
    }
    /// Adds `value` to contents of `a`, storing the result in register `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 17);
    /// processor.addi(0, 13, 1);
    /// assert_eq!(processor[1], 30);
    /// ```
    pub fn addi(&mut self, a: usize, value: usize, target: usize) {
        let init = self.registers[a];
        self.seti(target, init + value);
    }
    /// Multiplies the contents of registers `a` and `b`, storing the result in register
    /// `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 6);
    /// processor.seti(1, 3);
    /// processor.mulr(0, 1, 2);
    /// assert_eq!(processor[2], 18);
    /// ```
    pub fn mulr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.muli(a, value, target);
    }
    /// Multiplies `value` and the contents of register `a`, storing the result in register
    /// `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 6);
    /// processor.muli(0, 3, 2);
    /// assert_eq!(processor[2], 18);
    /// ```
    pub fn muli(&mut self, a: usize, value: usize, target: usize) {
        let init = self.registers[a];
        self.seti(target, init * value);
    }
    /// Performs a bitwise and of the contents of registers `a` and `b`, storing the result in
    /// register `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 5);
    /// processor.seti(1, 3);
    /// processor.banr(0, 1, 2);
    /// assert_eq!(processor[2], 1);
    /// ```
    pub fn banr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.bani(a, value, target);
    }
    /// Performs a bitwise and of `value` and the contents of register `a`, storing the result in
    /// register `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 5);
    /// processor.bani(0, 3, 2);
    /// assert_eq!(processor[2], 1);
    /// ```
    pub fn bani(&mut self, a: usize, value: usize, target: usize) {
        let init = self.registers[a];
        self.seti(target, init & value);
    }
    /// Performs a bitwise or of the contents of registers `a` and `b`, storing the result in
    /// register `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 5);
    /// processor.seti(1, 3);
    /// processor.borr(0, 1, 2);
    /// assert_eq!(processor[2], 7);
    /// ```
    pub fn borr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.bori(a, value, target);
    }
    /// Performs a bitwise or of `value` and the contents of register `a`, storing the result in
    /// register `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 5);
    /// processor.bori(0, 3, 2);
    /// assert_eq!(processor[2], 7);
    /// ```
    pub fn bori(&mut self, a: usize, value: usize, target: usize) {
        let init = self.registers[a];
        self.seti(target, init | value);
    }
    /// Checks whether the contents of register `a` is greater than the value `b`, storing the
    /// result of the comparison (`1` or `0`) in `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 1);
    /// processor.gtri(0, 0, 1);
    /// processor.gtri(0, 1, 2);
    /// processor.gtri(0, 2, 3);
    /// assert_eq!(processor[1], 1);
    /// assert_eq!(processor[2], 0);
    /// assert_eq!(processor[3], 0);
    /// ```
    pub fn gtri(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[a];
        self.seti(target, (value > b) as usize);
    }
    /// Checks whether the value `a` is greater than the contents of register `b`, storing the
    /// result of the comparison (`1` or `0`) in `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 1);
    /// processor.gtir(0, 0, 1);
    /// processor.gtir(1, 0, 2);
    /// processor.gtir(2, 0, 3);
    /// assert_eq!(processor[1], 0);
    /// assert_eq!(processor[2], 0);
    /// assert_eq!(processor[3], 1);
    /// ```
    pub fn gtir(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.seti(target, (a > value) as usize);
    }
    /// Checks whether the contents of register `a` is greater than the contents of register
    /// `b`, storing the result of the comparison (`1` or `0`) in `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 1);
    /// processor.seti(1, 2);
    /// processor.gtrr(1, 0, 2);
    /// processor.gtrr(0, 1, 3);
    /// assert_eq!(processor[2], 1);
    /// assert_eq!(processor[3], 0);
    /// ```
    pub fn gtrr(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.gtri(a, value, target);
    }
    /// Checks whether the value `a` is equal to the contents of register `b`, storing the
    /// result of the comparison (`1` or `0`) in `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 1);
    /// processor.eqri(0, 0, 1);
    /// processor.eqri(1, 0, 2);
    /// assert_eq!(processor[1], 0);
    /// assert_eq!(processor[2], 1);
    /// ```
    pub fn eqri(&mut self, a: usize, b: usize, target: usize) {
        let value = self.registers[b];
        self.seti(target, (value == a) as usize);
    }
    /// Checks whether the contents of register `a` is equal to the value `b`, storing the
    /// result of the comparison (`1` or `0`) in `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 1);
    /// processor.eqir(0, 0, 1);
    /// processor.eqir(0, 1, 2);
    /// assert_eq!(processor[1], 0);
    /// assert_eq!(processor[2], 1);
    /// ```
    pub fn eqir(&mut self, a: usize, b: usize, target: usize) {
        self.eqri(b, a, target);
    }
    /// Checks whether the contents of register `a` is equal to the contents of register `b`,
    /// storing the result of the comparison (`1` or `0`) in `target`.
    ///
    /// ### Examples
    ///
    /// ```
    ///# extern crate advent_of_code;
    ///# use advent_of_code::sixteen::*;
    /// let mut processor = Processor::default();
    /// processor.seti(0, 1);
    /// processor.seti(1, 0);
    /// processor.eqrr(0, 1, 2);
    /// processor.seti(1, 1);
    /// processor.eqrr(0, 1, 3);
    /// assert_eq!(processor[2], 0);
    /// assert_eq!(processor[3], 1);
    /// ```
    pub fn eqrr(&mut self, a: usize, b: usize, target: usize) {
        let a = self.registers[a];
        let b = self.registers[b];
        self.seti(target, (a == b) as usize);
    }
    /// Runs the opcode by index number.
    ///
    /// This method is pretty much only for internal use, but the indices are in source order.
    pub fn run(&mut self, o: usize, a: usize, b: usize, c: usize) {
        match o {
            0 => self.addr(a, b, c),
            1 => self.addi(a, b, c),
            2 => self.mulr(a, b, c),
            3 => self.muli(a, b, c),
            4 => self.banr(a, b, c),
            5 => self.bani(a, b, c),
            6 => self.borr(a, b, c),
            7 => self.bori(a, b, c),
            8 => self.setr(c, a),
            9 => self.seti(c, a),
            10 => self.gtir(a, b, c),
            11 => self.gtri(a, b, c),
            12 => self.gtrr(a, b, c),
            13 => self.eqir(a, b, c),
            14 => self.eqri(a, b, c),
            15 => self.eqrr(a, b, c),
            _ => unreachable!(),
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self { registers: [0; 4] }
    }
}

impl Index<usize> for Processor {
    type Output = usize;
    fn index(&self, index: usize) -> &usize {
        &self.registers[index]
    }
}

impl FromStr for Processor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut registers = s.split(',').map(|s| {
            (s.trim().replace((|c| c == ']' || c == '['), ""))
                .parse::<usize>()
                .unwrap()
        });
        Ok(Self {
            registers: [
                registers.next().unwrap(),
                registers.next().unwrap(),
                registers.next().unwrap(),
                registers.next().unwrap(),
            ],
        })
    }
}

impl fmt::Debug for Processor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self[0], self[1], self[2], self[3])
    }
}

fn reduce(
    possibilities: &mut HashMap<usize, HashSet<usize>>,
    map: &mut HashMap<usize, usize>,
) -> bool {
    let mut changed = false;
    let ps = possibilities.clone();
    let known = ps.iter().filter_map(|(i, p)| {
        if p.iter().count() == 1 {
            Some((i, p.iter().next().unwrap()))
        } else {
            None
        }
    });
    for (opcode, id) in known {
        changed = true;
        map.insert(*opcode, *id);
        for (_, p) in possibilities.iter_mut() {
            p.remove(&id);
        }
    }
    changed
}

/// Solve the puzzle using the input in `puzzles/16.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/16.txt");
    let mut parts = input.split("\n\n\n");
    let mut actions = parts.next().unwrap().lines().collect::<Vec<_>>();
    actions.push("\n");
    let len = actions.len();
    let mut acts = Vec::new();
    for i in 0..(len / 4) {
        let lower = 4 * i;
        let upper = lower + 4;
        let action = &actions[lower..upper];
        let mut s = action[0].to_owned();
        s.push_str("\n");
        s.push_str(action[1]);
        s.push_str("\n");
        s.push_str(action[2]);
        s.push_str("\n");
        s.push_str(action[3]);
        s.push_str("\n");
        let action = s.parse::<Action>().expect("Failed to parse action.");
        acts.push(action);
    }
    let mut three = 0;
    let mut possibilities: HashMap<usize, HashSet<usize>> = HashMap::new();
    for act in acts {
        let input = act.before;
        let inst = act.act;
        let a = inst.a;
        let b = inst.b;
        let c = inst.c;
        let opcode = inst.opcode;
        let output = act.after;
        let mut ct = 0;
        for i in 0..16 {
            let mut res = input.clone();
            res.run(i, a, b, c);
            if res == output {
                possibilities.entry(opcode).or_default().insert(i);
                ct += 1;
            }
        }
        if ct >= 3 {
            three += 1;
        }
    }
    let mut map = HashMap::new();
    while reduce(&mut possibilities, &mut map) {}
    let mut machine = Processor::default();
    for line in parts.next().unwrap().lines().filter(|l| !l.is_empty()) {
        let mut instr = line
            .parse::<Instruction>()
            .expect("Failed to parse instruction.");
        let o = map[&instr.opcode];
        let a = instr.a;
        let b = instr.b;
        let c = instr.c;
        machine.run(o, a, b, c);
    }
    println!("Day sixteen solutions: {}, {}", three, machine[0]);
}
