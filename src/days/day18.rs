// Day 18: Lavaduct Lagoon

use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

use crate::framework::grid::{IntVector, SignedGridShape, EAST, NORTH, SOUTH, WEST};
use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<Vec<DigInstruction>> {
    let input = include_str!("./day18_input.txt");
    parse_instructions(input)
}

pub struct Day18;

impl Day for Day18 {
    fn day_number(&self) -> u8 {
        18
    }

    fn part1(&self) -> Option<Result<String>> {
        if cfg!(feature = "slow_solutions") {
            Some(try_block(move || {
                let mut dig_site = DigSite::new();
                let instructions = puzzle_input()?;
                dig_site.dig_all(&instructions);
                let result = dig_site.dig_borders_and_interior(&instructions);
                println!("{}", dig_site);
                result?;
                Ok(format!("{}", dig_site.capacity()))
            }))
        } else {
            None
        }
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DigDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DigInstruction {
    direction: DigDirection,
    distance: usize,
    hex_color: String,
}

impl FromStr for DigInstruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_whitespace().collect_vec();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid instruction: {}", s));
        }
        let direction = match parts[0] {
            "U" => DigDirection::Up,
            "D" => DigDirection::Down,
            "L" => DigDirection::Left,
            "R" => DigDirection::Right,
            c => return Err(anyhow!("Invalid direction: {}", c)),
        };
        let distance: usize = parts[1].parse()?;
        let hex_color = parts[2].replace(|e| e == '(' || e == ')' || e == '#', "");
        Ok(Self {
            direction,
            distance,
            hex_color,
        })
    }
}

fn parse_instructions(input: &str) -> Result<Vec<DigInstruction>> {
    input
        .lines()
        .map(|line| line.parse::<DigInstruction>())
        .collect()
}

struct DigSite {
    map: HashMap<IntVector, bool>,
    current_position: IntVector,
}

impl DigSite {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            current_position: IntVector::new(0, 0),
        }
    }

    fn bounds(&self) -> (IntVector, IntVector) {
        let (min_x, max_x) = self
            .map
            .keys()
            .map(|it| it.x)
            .minmax()
            .into_option()
            .unwrap_or((0, 0));
        let (min_y, max_y) = self
            .map
            .keys()
            .map(|it| it.y)
            .minmax()
            .into_option()
            .unwrap_or((0, 0));
        (IntVector::new(min_x, min_y), IntVector::new(max_x, max_y))
    }

    fn capacity(&self) -> usize {
        self.map.len()
    }

    fn is_inside_borders(&self, coord: IntVector) -> bool {
        let (min, max) = self.bounds();
        let mut cursor = coord;
        if self.map.get(&cursor).is_some() {
            // this is a wall
            return false;
        }
        let mut in_wall = false;
        let mut intersections = 0;
        while cursor.x < max.x {
            cursor += EAST;
            if self.map.get(&cursor).is_some() {
                if !in_wall {
                    intersections += 1;
                }
                in_wall = true;
            } else {
                in_wall = false;
            }
        }

        let is_inside = intersections % 2 == 1;
        is_inside
    }

    fn dig(&mut self, instruction: &DigInstruction) {
        for _ in 0..instruction.distance {
            let delta = match instruction.direction {
                DigDirection::Up => NORTH,
                DigDirection::Down => SOUTH,
                DigDirection::Left => WEST,
                DigDirection::Right => EAST,
            };
            self.current_position += delta;
            self.map.insert(self.current_position, true);
        }
    }

    fn dig_all(&mut self, instructions: &[DigInstruction]) {
        for instruction in instructions {
            self.dig(instruction);
        }
    }

    fn dig_interior(&mut self) -> Result<()> {
        let (min_x, max_x) = self
            .map
            .keys()
            .map(|it| it.x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, may_y) = self
            .map
            .keys()
            .map(|it| it.y)
            .minmax()
            .into_option()
            .unwrap();
        let grid_shape = SignedGridShape {
            top_left: IntVector::new(min_x, min_y),
            bottom_right: IntVector::new(max_x, may_y),
        };

        let interior_point = {
            let boundaries = self.map.keys();
            let neighbors = boundaries.flat_map(|coord| coord.cardinal_neighbors());
            neighbors
                .filter(|it| self.map.get(it).is_none() && self.is_inside_borders(*it))
                .next()
                .ok_or(anyhow!("No interior points were found"))?
                .to_owned()
        };

        let mut queue = VecDeque::<IntVector>::new();
        queue.push_back(interior_point);
        // let mut timeout = 10_000_000;
        while let Some(coord) = queue.pop_front() {
            // timeout -= 1;
            // if timeout == 0 {
            //     return Err(anyhow!("Timeout"));
            // }
            self.map.insert(coord, true);
            let neighbors = coord.cardinal_neighbors();
            for neighbor in neighbors {
                if self.map.get(&neighbor).is_none() && grid_shape.is_in_bounds(coord) {
                    queue.push_back(neighbor);
                }
            }
        }
        Ok(())
    }

    fn dig_borders_and_interior(&mut self, instructions: &[DigInstruction]) -> Result<()> {
        self.dig_all(instructions);
        self.dig_interior()?;
        Ok(())
    }
}

impl Display for DigSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.bounds();
        let signed_grid_shape = SignedGridShape {
            top_left: min,
            bottom_right: max,
        };
        f.write_str(
            &signed_grid_shape.format_char_grid(signed_grid_shape.coord_iter().map(|coord| {
                if self.map.get(&coord).is_some() {
                    '#'
                } else {
                    '.'
                }
            })),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            super::Day18.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }

    fn sample_input() -> Vec<DigInstruction> {
        indoc! {"
          R 6 (#70c710)
          D 5 (#0dc571)
          L 2 (#5713f0)
          D 2 (#d2c081)
          R 2 (#59c680)
          D 2 (#411b91)
          L 5 (#8ceee2)
          U 2 (#caa173)
          L 1 (#1b58a2)
          U 2 (#caa171)
          R 2 (#7807d2)
          U 3 (#a77fa3)
          L 2 (#015232)
          U 2 (#7a21e3)
      "}
        .pipe(parse_instructions)
        .unwrap()
    }

    #[test]
    fn test_parse() {
        let input = "R 6 (#70c710)";
        let expected = DigInstruction {
            direction: DigDirection::Right,
            distance: 6,
            hex_color: "70c710".to_string(),
        };
        assert_eq!(input.parse::<DigInstruction>().unwrap(), expected);
    }

    #[test]
    fn test_dig_sides() {
        let mut dig_site = DigSite::new();
        let instructions = sample_input();
        dig_site.dig_all(&instructions);
        assert_eq!(dig_site.capacity(), 38);
    }

    #[test]
    fn test_dig_interior() {
        let mut dig_site = DigSite::new();
        let instructions = sample_input();
        dig_site.dig_borders_and_interior(&instructions).unwrap();
        assert_eq!(dig_site.capacity(), 62);
    }
}
