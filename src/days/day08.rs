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
    fn path(&self, starting_node: NodeLabel) -> DesertPathIterator {
        DesertPathIterator {
            map: self,
            current_node: starting_node,
            instruction_index: 0,
        }
    }

    fn steps_to_reach(&self, destination: NodeLabel) -> Result<u32> {
        let start: NodeLabel = "AAA".parse().unwrap();
        for (i, result) in self.path(start).enumerate() {
            let (node, _direction) = result?;
            if node == destination {
                return Ok(i as u32);
            }
        }
        return Err(anyhow!("No path found"));
    }

    fn steps_to_reach_zzz(&self) -> Result<u32> {
        let zzz: NodeLabel = "ZZZ".parse().unwrap();
        self.steps_to_reach(zzz)
    }

    fn find_loop(&self, starting_node: NodeLabel) -> Result<PathLoop> {
        let mut current_node = starting_node;
        let mut sequence: Vec<NodeLabel> = vec![];
        let mut seen_steps = HashMap::<(NodeLabel, usize), usize>::new();
        let mut instruction_index = 0;
        loop {
            if seen_steps.contains_key(&(current_node, instruction_index)) {
                break;
            }
            seen_steps.insert((current_node, instruction_index), sequence.len());
            let instruction = self.instructions[instruction_index];
            let node = self
                .network
                .0
                .get(&current_node)
                .ok_or(anyhow!("Couldn't find a node with label {}", current_node))?;
            sequence.push(current_node);
            current_node = match instruction {
                Direction::Right => node.right,
                Direction::Left => node.left,
            };
            instruction_index = (instruction_index + 1) % self.instructions.len();

            if sequence.len() > self.instructions.len() * 100 {
                return Err(anyhow!("Didn't find a loop after 100 iterations"));
            }
        }
        let loop_start = *seen_steps.get(&(current_node, instruction_index)).unwrap();
        let init = sequence[0..loop_start]
            .iter()
            .copied()
            .collect_vec()
            .into_boxed_slice();
        let loop_sequence = sequence[loop_start..]
            .iter()
            .copied()
            .collect_vec()
            .into_boxed_slice();
        Ok(PathLoop {
            init,
            sequence: loop_sequence,
        })
    }

    fn steps_to_reach_ghostly_destinations(&self) -> Result<usize> {
        let starting_nodes = self
            .network
            .0
            .keys()
            .copied()
            .filter(|label| label.is_start())
            .collect_vec();
        let loops: Vec<PathLoop> = starting_nodes
            .par_iter()
            .map(|it| self.find_loop(*it))
            .collect::<Result<_>>()?;
        let times_til_first_destination = loops.into_iter().map(|path_loop| {
            let first_destination = path_loop
                .sequence
                .iter()
                .enumerate()
                .find_map(|(i, label)| {
                    if label.is_destination() {
                        Some(i)
                    } else {
                        None
                    }
                })
                .ok_or(anyhow!("No destinations in loop"))?;
            Ok(path_loop.init.len() + first_destination)
        });
        times_til_first_destination
            .reduce(|a, b| match (a, b) {
                (Ok(a), Ok(b)) => Ok(num::integer::lcm(a, b)),
                (Err(err), Ok(_)) => Err(err),
                (Ok(_), Err(err)) => Err(err),
                (Err(err1), Err(_)) => Err(err1),
            })
            .ok_or(anyhow!("No loops found"))
            .and_then(|it| it)
    }

    #[cfg(feature = "slow_solutions")]
    fn steps_to_reach_ghostly_destinations_brute_force(&self) -> Result<usize> {
        let starting_nodes = self
            .network
            .0
            .keys()
            .copied()
            .filter(|label| label.is_start())
            .collect_vec();
        let loops: Vec<PathLoop> = starting_nodes
            .par_iter()
            .map(|it| self.find_loop(*it))
            .collect::<Result<_>>()?;

        #[derive(Debug, Clone)]
        struct LoopInfo {
            init_length: usize,
            sequence_length: usize,
            destination_indices: Vec<usize>,
        }
        let loops_info = loops
            .into_iter()
            .map(|it| LoopInfo {
                init_length: it.init.len(),
                sequence_length: it.sequence.len(),
                destination_indices: it
                    .sequence
                    .iter()
                    .enumerate()
                    .filter_map(|(i, node)| if node.is_destination() { Some(i) } else { None })
                    .collect_vec(),
            })
            .collect_vec();

        if loops_info.len() == 0 {
            return Err(anyhow!("No loops found"));
        }

        let mut current_step = loops_info.iter().map(|it| it.init_length).max().unwrap();
        const CHECKPOINT_DISTANCE: usize = 10_000_000_000;
        let mut last_checkpoint = CHECKPOINT_DISTANCE;
        loop {
            if current_step > last_checkpoint + CHECKPOINT_DISTANCE {
                last_checkpoint += CHECKPOINT_DISTANCE;
            }
            let loops_with_relative_index = loops_info
                .iter()
                .map(|loop_info| {
                    let index_after_init = current_step - loop_info.init_length;
                    let index = index_after_init % loop_info.sequence_length;
                    (index, loop_info)
                })
                .collect_vec();

            let at_destination_in_each_loop =
                loops_with_relative_index
                    .iter()
                    .all(|(relative_index, loop_info)| {
                        loop_info.destination_indices.contains(relative_index)
                    });
            if at_destination_in_each_loop {
                break;
            }

            let next_step = loops_with_relative_index
                .iter()
                .flat_map(|(relative_index, loop_info)| {
                    // 0 relative index in absolute space
                    let absolute_index_of_loop_start = {
                        let index_after_init = current_step - loop_info.init_length;
                        let loops_so_far = index_after_init / loop_info.sequence_length;
                        loop_info.init_length + (loops_so_far * loop_info.sequence_length)
                    };
                    loop_info
                        .destination_indices
                        .iter()
                        .map(move |destination_index| {
                            if destination_index <= relative_index {
                                // if we've already passed this detination, give the index in the next loop
                                absolute_index_of_loop_start
                                    + loop_info.sequence_length
                                    + destination_index
                            } else {
                                absolute_index_of_loop_start + destination_index
                            }
                        })
                })
                .min()
                .unwrap();
            current_step = next_step;
        }

        Ok(current_step)
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

#[derive(Debug)]
struct DesertPathIterator<'a> {
    map: &'a DesertMap,
    current_node: NodeLabel,
    instruction_index: usize,
}

impl<'a> Iterator for DesertPathIterator<'a> {
    /// The current item, and the next direction to go
    type Item = Result<(NodeLabel, Direction)>;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.map.instructions[self.instruction_index];
        let node = self
            .map
            .network
            .0
            .get(&self.current_node)
            .ok_or(anyhow!("Couldn't find a node with label"));
        let node = match node {
            Ok(node) => node,
            Err(err) => return Some(Err(err)),
        };
        self.current_node = match instruction {
            Direction::Right => node.right,
            Direction::Left => node.left,
        };
        self.instruction_index = (self.instruction_index + 1) % self.map.instructions.len();
        Some(Ok((node.label, instruction)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PathLoop {
    init: Box<[NodeLabel]>,
    sequence: Box<[NodeLabel]>,
}

impl<'a> IntoIterator for &'a PathLoop {
    type Item = NodeLabel;
    type IntoIter = PathLoopIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PathLoopIterator {
            path_loop: self,
            index: 0,
        }
    }
}

#[derive(Debug)]
struct PathLoopIterator<'a> {
    path_loop: &'a PathLoop,
    index: usize,
}

impl<'a> Iterator for PathLoopIterator<'a> {
    type Item = NodeLabel;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index < self.path_loop.init.len() {
            Some(self.path_loop.init[self.index])
        } else {
            let loop_index = self.index - self.path_loop.init.len();
            Some(self.path_loop.sequence[loop_index % self.path_loop.sequence.len()])
        };
        self.index += 1;
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day8.part1().unwrap().unwrap(), "19199".to_string(),);
    }

    #[test]
    fn test_part2() {
        let result = super::Day8.part2().unwrap().unwrap();
        let result: u64 = result.parse().unwrap();
        assert!(result > 1677130951, "{} > 1677130951", result);
        assert!(result > 1677130952, "{} > 1677130952", result); // just in case it was an off-by-one error :P
        assert_ne!(result, 12457759249955183594);
        assert_ne!(result, 13333977633595132672);
        assert_eq!(result, 13663968099527);
    }

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

    fn sample_input_for_ghosts() -> DesertMap {
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        DesertMap::from_str(input).unwrap()
    }

    #[test]
    fn test_navigate_for_ghosts() {
        let desert_map = sample_input_for_ghosts();
        let result = desert_map.steps_to_reach_ghostly_destinations().unwrap();
        assert_eq!(result, 6);
    }

    #[cfg(feature = "slow_solutions")]
    #[test]
    fn test_navigate_for_ghosts_brute_force() {
        let desert_map = sample_input_for_ghosts();
        let result = desert_map
            .steps_to_reach_ghostly_destinations_brute_force()
            .unwrap();
        assert_eq!(result, 6);
    }

    #[cfg(feature = "slow_solutions")]
    #[test]
    fn test_part_two_brute_force() {
        let desert_map = puzzle_input().unwrap();
        let result = desert_map
            .steps_to_reach_ghostly_destinations_brute_force()
            .unwrap();
        assert_eq!(result, 13663968099527);
    }

    #[test]
    fn test_loop_equivalence() {
        let desert_map = sample_input_for_ghosts();
        let loop1 = desert_map
            .find_loop(NodeLabel::from_str("11A").unwrap())
            .unwrap();
        let path1 = desert_map.path(NodeLabel::from_str("11A").unwrap());
        for (i, (path_node, loop_node)) in path1.zip(&loop1).take(100).enumerate() {
            assert_eq!(path_node.unwrap().0, loop_node, "step {}", i);
        }

        let loop1 = desert_map
            .find_loop(NodeLabel::from_str("22A").unwrap())
            .unwrap();
        let path1 = desert_map.path(NodeLabel::from_str("22A").unwrap());
        for (i, (path_node, loop_node)) in path1.zip(&loop1).take(100).enumerate() {
            assert_eq!(path_node.unwrap().0, loop_node, "step {}", i);
        }
    }

    #[test]
    fn test_assumptions() {
        let puzzle_input = puzzle_input().unwrap();
        let sample_input = sample_input_for_ghosts();

        let puzzle_starting_nodes = puzzle_input
            .network
            .0
            .keys()
            .copied()
            .filter(|label| label.is_start())
            .collect_vec();
        let puzzle_loops = puzzle_starting_nodes
            .iter()
            .map(|starting_node| puzzle_input.find_loop(*starting_node))
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let sample_loops = sample_input
            .network
            .0
            .keys()
            .copied()
            .filter(|label| label.is_start())
            .map(|starting_node| sample_input.find_loop(starting_node))
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let all_loops = puzzle_loops.iter().chain(sample_loops.iter()).collect_vec();

        for path_loop in &all_loops {
            let starting_node = path_loop.init[0];
            let destinations_in_init = path_loop
                .init
                .iter()
                .copied()
                .enumerate()
                .filter(|(_, label)| label.is_destination())
                .collect_vec();
            let destinations_in_sequence = path_loop
                .sequence
                .iter()
                .copied()
                .enumerate()
                .filter(|(_, label)| label.is_destination())
                .collect_vec();
            assert_eq!(
                destinations_in_init.len(),
                0,
                "path starting at {} should not have destinations in init. found: {:?}",
                starting_node,
                destinations_in_init
            );
            assert!(
                destinations_in_sequence.len() >= 1,
                "path starting at {} should have at least one destination in sequence. found: {:?}",
                starting_node,
                destinations_in_sequence
            );

            // I have no idea why this might be the case, but it sure is useful
            assert!(destinations_in_sequence.iter().any(|destination| {
                path_loop.sequence.len() == path_loop.init.len() + destination.0
            }));
        }
    }
}
