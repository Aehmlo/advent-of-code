//! Day eight (Memory Maneuver)

use std::str::FromStr;

/// Represents a node in the tree.
#[derive(Default)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    /// Returns the sum of all metadata for this node and its children.
    pub fn sum(&self) -> u32 {
        self.metadata.iter().sum::<u32>() + self.children.iter().map(|c| c.sum()).sum::<u32>()
    }
    /// Returns the value of the node.
    pub fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum::<u32>()
        } else {
            let mut total = 0;
            for index in self.metadata.iter().clone() {
                if let Some(node) = self.children.get(*index as usize - 1) {
                    total += node.value();
                }
            }
            total
        }
    }
}

/// Represents a tree in this insane license validation process.
#[derive(Default)]
pub struct Tree {
    root: Node,
}

impl Tree {
    /// Returns a naïve sum of all metadata within the tree's nodes.
    fn sum(&self) -> u32 {
        self.root.sum()
    }
    /// Returns the value of the root node.
    fn value(&self) -> u32 {
        self.root.value()
    }
}

fn parse_node(iter: &mut impl Iterator<Item = u32>) -> Option<Node> {
    iter.next().map(|children| {
        let mut node = Node::default();
        let metadata = iter.next().expect("Incomplete header");
        for _ in 0..children {
            node.children.extend(parse_node(iter));
        }
        for _ in 0..metadata {
            node.metadata.push(iter.next().expect("Missing metadata"));
        }
        node
    })
}

impl FromStr for Tree {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ').map(|w| w.trim().parse().unwrap());
        Ok(Tree {
            root: parse_node(&mut iter).expect("No root node"),
        })
    }
}

/// Solve the puzzle using the input in `puzzles/8.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/8.txt");
    let tree = input.parse::<Tree>().unwrap();
    println!("Day eight solutions: {}, {}", tree.sum(), tree.value());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn sample_input() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let tree: Tree = input.parse().unwrap();
        assert_eq!(tree.sum(), 138);
        assert_eq!(tree.value(), 66);
    }
}
