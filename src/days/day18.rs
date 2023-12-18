// Day 18: Lavaduct Lagoon

use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

use crate::framework::grid::{GridShape, IntVector, EAST, NORTH, SOUTH, WEST};
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
        Some(try_block(move || {
            let mut dig_site = DigSite::from_instructions(&puzzle_input()?);
            let result = dig_site.dig_interior();
            result?;
            Ok(format!("{}", dig_site.capacity()))
        }))
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
    shape: GridShape,
    map: Box<[bool]>,
}

impl DigSite {
    fn from_instructions(instructions: &[DigInstruction]) -> Self {
        let mut dug_tiles = HashSet::<IntVector>::new();
        let mut current_position = IntVector::new(0, 0);

        for instruction in instructions {
            for _ in 0..instruction.distance {
                let delta = match instruction.direction {
                    DigDirection::Up => NORTH,
                    DigDirection::Down => SOUTH,
                    DigDirection::Left => WEST,
                    DigDirection::Right => EAST,
                };
                current_position += delta;
                dug_tiles.insert(current_position);
            }
        }

        let (min_x, max_x) = dug_tiles
            .iter()
            .map(|it| it.x)
            .minmax()
            .into_option()
            .unwrap_or((0, 0));
        let (min_y, max_y) = dug_tiles
            .iter()
            .map(|it| it.y)
            .minmax()
            .into_option()
            .unwrap_or((0, 0));

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        let mut map = vec![false; width * height].into_boxed_slice();
        for coord in dug_tiles {
            let x = (coord.x - min_x) as usize;
            let y = (coord.y - min_y) as usize;
            let index = y * width + x;
            map[index] = true;
        }

        Self {
            shape: GridShape { width, height },
            map,
        }
    }

    fn get(&self, coord: IntVector) -> bool {
        let index = self.shape.arr_index(coord);
        self.map[index]
    }

    fn capacity(&self) -> usize {
        self.map.iter().copied().filter(|it| *it).count()
    }

    fn is_inside_borders(&self, coord: IntVector) -> bool {
        if !self.shape.in_bounds(coord) {
            return false;
        }
        let mut cursor = coord;
        println!("{:?}", cursor);
        if self.get(cursor) {
            // this is a wall
            return false;
        }
        let mut in_wall = false;
        let mut intersections = 0;
        while self.shape.in_bounds(cursor + EAST) {
            cursor += EAST;
            if self.get(cursor) {
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

    fn dig_interior(&mut self) -> Result<()> {
        // let (min_x, max_x) = self
        //     .map
        //     .keys()
        //     .map(|it| it.x)
        //     .minmax()
        //     .into_option()
        //     .unwrap();
        // let (min_y, may_y) = self
        //     .map
        //     .keys()
        //     .map(|it| it.y)
        //     .minmax()
        //     .into_option()
        //     .unwrap();
        // let grid_shape = SignedGridShape {
        //     top_left: IntVector::new(min_x, min_y),
        //     bottom_right: IntVector::new(max_x, may_y),
        // };

        let interior_point = {
            let boundaries = self
                .map
                .iter()
                .copied()
                .enumerate()
                // DIRTY DIRTY hack
                // There's a problem with the top of the puzzle input map
                // where the first row looks like ....######.....
                // and anything left of the wall is considered "inside"
                .rev()
                .filter(|(_, it)| *it)
                .map(|(index, _)| self.shape.coordinate_for_index(index));
            let neighbors = boundaries.flat_map(|coord| coord.cardinal_neighbors());
            neighbors
                .filter(|it| self.is_inside_borders(*it))
                .next()
                .ok_or(anyhow!("No interior points were found"))?
                .to_owned()
        };

        dbg!(&interior_point);

        let mut queue = VecDeque::<IntVector>::new();
        queue.push_back(interior_point);
        // let mut timeout = 10_000_000;
        while let Some(coord) = queue.pop_front() {
            if self.get(coord) {
                continue;
            }
            // timeout -= 1;
            // if timeout == 0 {
            //     return Err(anyhow!("Timeout"));
            // }
            self.map[self.shape.arr_index(coord)] = true;
            let neighbors = coord.cardinal_neighbors();
            for neighbor in neighbors {
                if self.shape.in_bounds(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
        Ok(())
    }
}

impl Display for DigSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .shape
                .format_char_grid(self.shape.coord_iter().map(|coord| {
                    if self.get(coord) {
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
        assert_eq!(super::Day18.part1().unwrap().unwrap(), "34329".to_string(),);
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
        let dig_site = DigSite::from_instructions(&sample_input());
        assert_eq!(dig_site.capacity(), 38);
    }

    #[test]
    fn test_dig_interior() {
        let mut dig_site = DigSite::from_instructions(&sample_input());
        dig_site.dig_interior().unwrap();
        assert_eq!(dig_site.capacity(), 62);
    }
}
