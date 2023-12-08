// Day 8: Haunted Wasteland

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use regex::Regex;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day8;

fn puzzle_input() -> Result<DesertMap> {
    let input = include_str!("./day08_input.txt");
    DesertMap::from_str(input)
}

impl Day for Day8 {
    fn day_number(&self) -> u8 {
        8
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            Ok(puzzle_input()?.steps_to_reach_zzz()?.to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            Ok(puzzle_input()?
                .steps_to_reach_ghostly_destinations()?
                .to_string())
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NodeLabel([u8; 3]);

impl NodeLabel {
    fn is_start(&self) -> bool {
        self.0[2] == 'A' as u8
    }

    fn is_destination(&self) -> bool {
        self.0[2] == 'Z' as u8
    }
}

impl Display for NodeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.0[0].conv::<char>(),
            self.0[1].conv::<char>(),
            self.0[2].conv::<char>(),
        )
    }
}

impl Debug for NodeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl FromStr for NodeLabel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().count() != 3 {
            return Err(anyhow!("Invalid label: {}", s));
        }
        let mut chars = s.chars();
        let mut label = [0; 3];
        for i in 0..3 {
            label[i] = chars
                .next()
                .ok_or_else(|| anyhow!("Invalid label"))?
                .try_into()?;
        }
        Ok(Self(label))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    label: NodeLabel,
    left: NodeLabel,
    right: NodeLabel,
}

#[derive(Debug, Clone)]
struct Network(HashMap<NodeLabel, Node>);

#[derive(Debug, Clone)]
struct DesertMap {
    instructions: Vec<Direction>,
    network: Network,
}

impl DesertMap {
    fn steps_to_reach(&self, destination: NodeLabel) -> Result<u32> {
        let start: NodeLabel = "AAA".parse().unwrap();

        let mut current = start;
        let mut steps = 0;
        let mut instruction_index = 0;
        while current != destination {
            let instruction = self.instructions[instruction_index];
            let node = self
                .network
                .0
                .get(&current)
                .ok_or(anyhow!("Couldn't find a node with label {}", current))?;
            current = match instruction {
                Direction::Right => node.right,
                Direction::Left => node.left,
            };
            steps += 1;
            instruction_index = (instruction_index + 1) % self.instructions.len();
        }
        Ok(steps)
    }

    fn steps_to_reach_zzz(&self) -> Result<u32> {
        let zzz: NodeLabel = "ZZZ".parse().unwrap();
        self.steps_to_reach(zzz)
    }

    fn steps_to_reach_ghostly_destinations(&self) -> Result<u32> {
        let mut current_nodes = self
            .network
            .0
            .keys()
            .copied()
            .filter(|label| label.is_start())
            .collect_vec();
        let mut steps = 0;
        let mut instruction_index = 0;
        while !current_nodes.iter().all(|label| label.is_destination()) {
            let instruction = self.instructions[instruction_index];
            let next_nodes = current_nodes
                .iter()
                .map(|label| {
                    let node = self
                        .network
                        .0
                        .get(label)
                        .ok_or(anyhow!("Couldn't find a node with label {}", label))?;
                    match instruction {
                        Direction::Right => Ok(node.right),
                        Direction::Left => Ok(node.left),
                    }
                })
                .collect::<Result<Vec<_>>>()?;
            current_nodes = next_nodes;
            steps += 1;
            instruction_index = (instruction_index + 1) % self.instructions.len();
        }
        Ok(steps)
    }
}

impl FromStr for DesertMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.replace("\r\n", "\n");
        let (instructions_str, nodes_str) = s.split_once("\n\n").unwrap();
        let instructions = instructions_str
            .chars()
            .map(|char| match char {
                'R' => Ok(Direction::Right),
                'L' => Ok(Direction::Left),
                _ => Err(anyhow!("Invalid direction: {}", char)),
            })
            .collect::<Result<Vec<_>>>()?;
        let node_line_pattern =
            Regex::new("^([A-Z0-9]{3}) = \\(([A-Z0-9]{3}), ([A-Z0-9]{3})\\)$").unwrap();
        let nodes = nodes_str
            .lines()
            .map(|node_line| {
                let captures = node_line_pattern
                    .captures(node_line)
                    .ok_or_else(|| anyhow!("Invalid node: {}", node_line))?;
                let label = captures.get(1).unwrap().as_str().parse()?;
                let left = captures.get(2).unwrap().as_str().parse()?;
                let right = captures.get(3).unwrap().as_str().parse()?;
                Ok(Node { label, left, right })
            })
            .collect::<Result<Vec<_>>>()?;

        let network = Network(nodes.into_iter().map(|node| (node.label, node)).collect());
        Ok(DesertMap {
            instructions,
            network,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day8.part1().unwrap().unwrap(), "19199".to_string(),);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(super::Day8.part2().unwrap().unwrap(), "0".to_string(),);
    // }

    fn sample_input() -> DesertMap {
        let input = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};
        DesertMap::from_str(input).unwrap()
    }

    #[test]
    fn test_parse() {
        let desert_map = sample_input();
        assert_eq!(desert_map.instructions.len(), 2);
        assert_eq!(desert_map.network.0.len(), 7);
        assert!(desert_map.network.0.contains_key(&"AAA".parse().unwrap()));
        assert!(desert_map.network.0.contains_key(&"ZZZ".parse().unwrap()));
    }

    #[test]
    fn test_navigate() {
        let desert_map = sample_input();
        let result = desert_map.steps_to_reach_zzz().unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_navigate_with_looping_instructions() {
        let desert_map = DesertMap::from_str(indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "})
        .unwrap();
        let result = desert_map.steps_to_reach_zzz().unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_navigate_for_ghosts() {
        let desert_map = DesertMap::from_str(indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "})
        .unwrap();
        let result = desert_map.steps_to_reach_ghostly_destinations().unwrap();
        assert_eq!(result, 6);
    }
}
