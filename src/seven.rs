//! Day seven (The Sum of Its Parts)

use std::collections::{BTreeSet, HashMap, HashSet};

/// Represents a step in the sleigh-building process.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step(char);

impl From<char> for Step {
    fn from(c: char) -> Self {
        if !c.is_alphabetic() {
            panic!("Steps must have alphabetic identifiers (found {}).", c);
        }
        Step(c)
    }
}

impl Into<char> for Step {
    fn into(self) -> char {
        self.0
    }
}

impl Step {
    /// Returns the time necessary to complete the step.
    pub fn time(&self) -> u16 {
        let c: char = (*self).into();
        c as u16 - 'A' as u16 + 1
    }
}

/// Creates a dependency tree for the given input string.
pub fn tree(input: &str) -> HashMap<Step, Vec<Step>> {
    let mut tree: HashMap<Step, Vec<Step>> = HashMap::new();
    for line in input.lines() {
        let req = line.chars().nth(5).unwrap();
        let step = line.chars().nth(36).unwrap();
        tree.entry(step.into()).or_default().push(req.into());
        // Create the other step too, or we'll never know about steps with no dependencies.
        tree.entry(req.into()).or_default();
    }
    tree
}

/// Resolves the dependency tree to give the order in which to proceed.
pub fn resolve(tree: &HashMap<Step, Vec<Step>>) -> impl Iterator<Item = Step> {
    let mut remaining = tree.keys().clone().collect::<BTreeSet<_>>();
    let mut resolved: HashSet<Step> = HashSet::new();
    let mut order = Vec::new();
    while !remaining.is_empty() {
        let left = remaining.clone();
        for step in left.iter() {
            let check = tree
                .get(&step)
                .map(|l| l.iter().all(|s| resolved.contains(s)))
                .unwrap_or(true);
            if check {
                let step = **step;
                order.push(step);
                resolved.insert(step);
                remaining.remove(&step);
                break;
            }
        }
    }
    order.into_iter()
}

/// Returns the order of steps to perform as a string.
pub fn steps(tree: &HashMap<Step, Vec<Step>>) -> String {
    resolve(&tree)
        .map(|s| {
            let c: char = s.into();
            c
        }).collect()
}

/// Represents a worker on the sleigh construction project.
#[derive(Clone, Default)]
pub struct Worker {
    /// The amount of time left for this worker.
    time: u16,
    /// The current task this worker is attempting.
    task: Option<Step>,
}

impl Worker {
    /// Runs the next minute in the simulation for this worker.
    pub fn tick(&mut self) {
        if self.time > 0 {
            self.time -= 1;
        }
    }
    /// Assigns the given job to the worker.
    pub fn assign(&mut self, job: Step, base_time: u16) {
        let time = job.time();
        self.time += time + base_time;
        self.task = Some(job);
    }
}

/// Returns the necessary time to complete the given work with the given number of workers.
pub fn time(tree: &HashMap<Step, Vec<Step>>, no_workers: usize, base_time: u16) -> u16 {
    let mut remaining = tree.keys().clone().collect::<BTreeSet<_>>();
    let mut done = HashSet::new();
    let mut workers: Vec<Worker> = vec![Worker::default(); no_workers];

    let mut elapsed = 0;

    loop {
        elapsed += 1;
        let mut idle = Vec::new();

        for worker in workers.iter_mut() {
            worker.tick();
            if worker.time == 0 {
                if let Some(task) = worker.task.take() {
                    done.insert(task);
                }
                idle.push(worker);
            }
        }

        if remaining.is_empty() && idle.len() == no_workers {
            return elapsed - 1;
        }

        if idle.is_empty() {
            continue;
        }

        let left = remaining.clone();
        for step in left.iter() {
            let check = tree
                .get(&step)
                .map(|l| l.iter().all(|s| done.contains(s)))
                .unwrap_or(true);
            if check {
                if let Some(worker) = idle.pop() {
                    worker.assign(**step, base_time);
                    remaining.remove(*step);
                }
            }
        }
    }
}

/// Solve the puzzle using the input in `puzzles/7.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/7.txt");
    let tree = tree(&input);
    println!(
        "Day seven solutions: {}, {}",
        steps(&tree),
        time(&tree, 5, 60)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input() {
        let input = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.";
        let tree = tree(&input);
        assert_eq!(&steps(&tree), "CABDFE");
        assert_eq!(time(&tree, 2, 0), 15);
    }
}
