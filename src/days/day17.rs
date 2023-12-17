// Day 17: Clumsy Crucible

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::framework::grid::{Direction, GridShape, IntVector};
use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> CityMap {
    include_str!("./day17_input.txt").parse().unwrap()
}

pub struct Day17;

impl Day for Day17 {
    fn day_number(&self) -> u8 {
        17
    }

    fn part1(&self) -> Option<Result<String>> {
        if cfg!(feature = "slow_solutions") {
            Some(try_block(move || {
                puzzle_input()
                    .find_minimal_heat_loss(SimpleCrucible)
                    .ok_or(anyhow!("No path found"))?
                    .to_string()
                    .pipe(Ok)
            }))
        } else {
            None
        }
    }

    fn part2(&self) -> Option<Result<String>> {
        // if cfg!(feature = "slow_solutions") {
        Some(try_block(move || {
            puzzle_input()
                .find_minimal_heat_loss(UltraCrucible)
                .ok_or(anyhow!("No path found"))?
                .to_string()
                .pipe(Ok)
        }))
        // } else {
        //     None
        // }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PathfindingNode {
    position: IntVector,
    direction: Direction,
    length_of_straight_line: u8,
}

trait Crucible {
    fn valid_moves(
        &self,
        from_node: PathfindingNode,
        destination_position: IntVector,
    ) -> Vec<PathfindingNode>;
}

struct SimpleCrucible;

impl Crucible for SimpleCrucible {
    fn valid_moves(&self, from_node: PathfindingNode, _: IntVector) -> Vec<PathfindingNode> {
        from_node
            .position
            .cardinal_neighbors_with_directions()
            .into_iter()
            .filter_map(|(neighbor_position, direction)| {
                if direction == from_node.direction.opposite() {
                    return None;
                }
                if from_node.length_of_straight_line >= 3 && direction == from_node.direction {
                    return None;
                }
                let new_length_of_straight_line = if direction == from_node.direction {
                    from_node.length_of_straight_line + 1
                } else {
                    1
                };
                let node = PathfindingNode {
                    position: neighbor_position,
                    direction,
                    length_of_straight_line: new_length_of_straight_line,
                };
                Some(node)
            })
            .collect()
    }
}

struct UltraCrucible;

impl Crucible for UltraCrucible {
    fn valid_moves(
        &self,
        from_node: PathfindingNode,
        destination_position: IntVector,
    ) -> Vec<PathfindingNode> {
        from_node
            .position
            .cardinal_neighbors_with_directions()
            .into_iter()
            .filter_map(|(neighbor_position, direction)| {
                if direction == from_node.direction.opposite() {
                    return None;
                }
                if from_node.length_of_straight_line < 4 && (direction != from_node.direction) {
                    // We can't turn yet.
                    return None;
                }
                if from_node.length_of_straight_line >= 10 && direction == from_node.direction {
                    return None;
                }
                let new_length_of_straight_line = if direction == from_node.direction {
                    from_node.length_of_straight_line + 1
                } else {
                    1
                };
                if new_length_of_straight_line < 4 && (neighbor_position == destination_position) {
                    // We can't stop yet.
                    return None;
                }
                let node = PathfindingNode {
                    position: neighbor_position,
                    direction,
                    length_of_straight_line: new_length_of_straight_line,
                };
                Some(node)
            })
            .collect()
    }
}

struct CityMap {
    shape: GridShape,
    heat_loss_for_blocks: Box<[u8]>,
}

impl CityMap {
    fn heat_loss_for_block(&self, coord: IntVector) -> Option<u8> {
        if self.shape.in_bounds(coord) {
            Some(self.heat_loss_for_blocks[self.shape.arr_index(coord)])
        } else {
            None
        }
    }

    fn find_minimal_heat_loss<TCrucible: Crucible>(&self, crucible: TCrucible) -> Option<u64> {
        let destination_position = IntVector::new(
            self.shape.width as isize - 1,
            self.shape.height as isize - 1,
        );

        let mut distances = HashMap::<PathfindingNode, u64>::new();
        let mut paths = HashMap::<PathfindingNode, Vec<PathfindingNode>>::new();
        let mut visited_nodes = HashSet::<PathfindingNode>::new();

        let start_node_e = PathfindingNode {
            position: IntVector::new(0, 0),
            direction: Direction::East,
            length_of_straight_line: 1,
        };
        let start_node_s = PathfindingNode {
            position: IntVector::new(0, 0),
            direction: Direction::South,
            length_of_straight_line: 1,
        };

        distances.insert(start_node_e, 0);
        paths.insert(start_node_e, vec![start_node_e]);
        distances.insert(start_node_s, 0);
        paths.insert(start_node_s, vec![start_node_s]);

        fn best_node(
            distances: &HashMap<PathfindingNode, u64>,
            visited_nodes: &HashSet<PathfindingNode>,
            _paths: &HashMap<PathfindingNode, Vec<PathfindingNode>>,
        ) -> Option<(PathfindingNode, u64)> {
            let result = distances
                .iter()
                .filter(|(node, _)| !visited_nodes.contains(node))
                .min_by_key(|(_, distance)| *distance)
                .map(|(node, distance)| (*node, *distance))?;

            // let options = distances
            //     .iter()
            //     .filter(|(node, _)| !visited_nodes.contains(node))
            //     .filter(|(_, distance)| **distance == result.1)
            //     .collect_vec();

            // if options.len() > 1 {
            //     println!(
            //         "multiple options are available for the next step: {:#?}",
            //         options
            //     );
            // }

            Some(result)
        }

        while let Some((node, current_distance)) = best_node(&distances, &visited_nodes, &paths) {
            visited_nodes.insert(node);
            if node.position == destination_position {
                break;
            }

            let neighboring_nodes_and_immediate_heat_loss = crucible
                .valid_moves(node, destination_position)
                .into_iter()
                .filter_map(|to_node| {
                    let heat_loss = self.heat_loss_for_block(to_node.position)?;
                    Some((to_node, heat_loss))
                })
                .filter(|(node, _)| !visited_nodes.contains(node));

            for (neighboring_node, immediate_heat_loss) in neighboring_nodes_and_immediate_heat_loss
            {
                let distance = current_distance + immediate_heat_loss as u64;
                let mut new_path = paths.get(&node).unwrap().clone();
                new_path.push(neighboring_node);
                if let Some(existing_distance) = distances.get_mut(&neighboring_node) {
                    if distance < *existing_distance {
                        *existing_distance = distance;
                        paths.insert(neighboring_node, new_path);
                    }
                } else {
                    distances.insert(neighboring_node, distance);
                    paths.insert(neighboring_node, new_path);
                }
            }
        }

        let (final_node, distance_to_destination) = distances
            .iter()
            .filter(|(node, _)| {
                node.position == destination_position && visited_nodes.contains(node)
            })
            .min_by_key(|(_, distance)| *distance)
            .map(|(final_node, distance)| (*final_node, *distance))?;

        let final_path = paths.get(&final_node).unwrap();
        self.shape
            .format_char_grid(self.shape.coord_iter().map(|coord| {
                if let Some(path_node) = final_path.iter().find(|node| node.position == coord) {
                    match path_node.direction {
                        Direction::North => '^',
                        Direction::East => '>',
                        Direction::South => 'v',
                        Direction::West => '<',
                    }
                    // if path_node.length_of_straight_line >= 10 {
                    //     return '+';
                    // }
                    // path_node.length_of_straight_line.to_string().chars().next().unwrap()
                } else {
                    '.'
                }
            }))
            .pipe(|it| println!("{}", it));

        Some(distance_to_destination)
    }
}

impl FromStr for CityMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (shape, chars) = GridShape::parse_char_grid(s)?;
        let heat_loss_for_blocks = chars
            .iter()
            .map(|&c| {
                let digit = c.to_digit(10);
                if let Some(digit) = digit {
                    Ok(digit as u8)
                } else {
                    Err(anyhow!("Invalid character in input: {:?}", c))
                }
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            shape,
            heat_loss_for_blocks,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "slow_solutions")]
    #[test]
    fn test_part1() {
        assert_eq!(super::Day17.part1().unwrap().unwrap(), "1238".to_string(),);
    }

    fn sample_input() -> CityMap {
        indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "}
        .parse()
        .unwrap()
    }

    #[test]
    fn find_minimal_heat_loss() {
        assert_eq!(
            sample_input().find_minimal_heat_loss(SimpleCrucible),
            Some(102)
        );
    }

    #[test]
    fn test_ultra_crucible() {
        assert_eq!(
            sample_input().find_minimal_heat_loss(UltraCrucible),
            Some(94)
        );
    }

    #[test]
    fn test_ultra_crucible_unfortunate() {
        let city_map: CityMap = indoc! {"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "}
        .parse()
        .unwrap();
        assert_eq!(city_map.find_minimal_heat_loss(UltraCrucible), Some(71));
    }
}
