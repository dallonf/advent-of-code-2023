// Day 21: Step Counter

use std::collections::HashSet;
use std::str::FromStr;

use crate::framework::grid::{GridShape, IntVector};
use crate::framework::Day;
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
        self.tiles[self.shape.arr_index(coord)]
    }

    fn gardens_reachable(&self, num_steps: usize) -> usize {
        let mut frontier = HashSet::<IntVector>::new();
        frontier.insert(self.start_position);
        for _ in 0..num_steps {
            let mut new_frontier = HashSet::<IntVector>::with_capacity(frontier.capacity());
            for coord in &frontier {
                for neighbor in coord.cardinal_neighbors() {
                    match self.get(neighbor) {
                        Tile::GardenPlot => {
                            new_frontier.insert(neighbor);
                        }
                        Tile::Rock => {}
                    }
                }
            }
            frontier = new_frontier;
        }

        frontier.len()
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
}
