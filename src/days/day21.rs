// Day 21: Step Counter

use std::collections::HashSet;
use std::str::FromStr;
use std::time::Instant;

use num::Integer;

use crate::framework::grid::{GridShape, IntVector};
use crate::framework::{format_duration, Day};
use crate::prelude::*;

fn puzzle_input() -> Result<GardenMap> {
    include_str!("./day21_input.txt").parse()
}

pub struct Day21;

impl Day for Day21 {
    fn day_number(&self) -> u8 {
        21
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?.gardens_reachable(64).to_string().pipe(Ok)
        }))
    }

    #[cfg(feature = "slow_solutions")]
    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?
                .gardens_reachable(26501365)
                .to_string()
                .pipe(Ok)
        }))
    }

    #[cfg(not(feature = "slow_solutions"))]
    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GardenMap {
    tiles: Vec<Tile>,
    shape: GridShape,
    start_position: IntVector,
}

impl GardenMap {
    fn get(&self, coord: IntVector) -> Tile {
        let looped_coord = IntVector::new(
            coord.x.rem_euclid(self.shape.width as isize),
            coord.y.rem_euclid(self.shape.height as isize),
        );
        self.tiles[self.shape.arr_index(looped_coord)]
    }

    fn gardens_reachable(&self, num_steps: usize) -> usize {
        let mut last_checkpoint = Instant::now();

        let mut visited_odd = HashSet::<IntVector>::new();
        let mut visited_even = HashSet::<IntVector>::new();

        let mut frontier = HashSet::<IntVector>::new();
        frontier.insert(self.start_position);
        visited_even.insert(self.start_position);
        for i in 0..num_steps {
            let mut new_frontier = HashSet::<IntVector>::with_capacity(frontier.capacity());
            for coord in &frontier {
                for neighbor in coord.cardinal_neighbors() {
                    match self.get(neighbor) {
                        Tile::GardenPlot => {
                            if !visited_even.contains(&neighbor) && !visited_odd.contains(&neighbor)
                            {
                                new_frontier.insert(neighbor);
                                let next_step = i + 1;
                                if next_step.is_even() {
                                    visited_even.insert(neighbor);
                                } else {
                                    visited_odd.insert(neighbor);
                                }
                            }
                        }
                        Tile::Rock => {}
                    }
                }
            }
            frontier = new_frontier;
            if num_steps % 5000 == 0 {
                let now = Instant::now();
                let duration = format_duration(&now.duration_since(last_checkpoint));
                println!(
                    "Step {}: {} visited, {} frontier ({} elapsed since last checkpoint)",
                    i,
                    visited_even.len() + visited_odd.len(),
                    frontier.len(),
                    duration,
                );
                last_checkpoint = now;
            }
        }

        if num_steps.is_even() {
            visited_even.len()
        } else {
            visited_odd.len()
        }
    }
}

impl FromStr for GardenMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start_position: Option<IntVector> = None;
        let (shape, chars) = GridShape::parse_char_grid(s)?;
        let tiles = chars
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                match c {
                    '.' => Tile::GardenPlot,
                    '#' => Tile::Rock,
                    'S' => {
                        if start_position.is_some() {
                            return Err(anyhow!("Multiple start positions found"));
                        }
                        start_position = Some(shape.coordinate_for_index(i));
                        Tile::GardenPlot
                    }
                    _ => return Err(anyhow!("Invalid character in map: {}", c)),
                }
                .pipe(Ok)
            })
            .collect::<Result<Vec<_>>>()?;
        let start_position = start_position.ok_or_else(|| anyhow!("No start position found"))?;
        Ok(Self {
            tiles,
            shape,
            start_position,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day21.part1().unwrap().unwrap(), "3847".to_string(),);
    }

    fn sample_input() -> GardenMap {
        indoc! {"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        "}
        .parse()
        .unwrap()
    }

    #[test]
    fn test_gardens_reachable() {
        let map = sample_input();
        assert_eq!(map.gardens_reachable(6), 16);
    }

    #[test]
    fn test_infinite_gardens() {
        let map = sample_input();
        assert_eq!(map.gardens_reachable(10), 50);
        assert_eq!(map.gardens_reachable(50), 1594);
        assert_eq!(map.gardens_reachable(100), 6536);
        assert_eq!(map.gardens_reachable(500), 167004);
        assert_eq!(map.gardens_reachable(1000), 668697);
        // the below are still too slow to run!
        // assert_eq!(map.gardens_reachable(5000), 16733044);
    }
}
