// Day 18: Lavaduct Lagoon

use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::ops::Range;
use std::str::FromStr;

use crate::framework::grid::{
    Direction, GridShape, IntVector, SignedGridShape, EAST, NORTH, SOUTH, WEST,
};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DigInstruction {
    direction: Direction,
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
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
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
    rows: Vec<Range<isize>>,
    columns: Vec<Range<isize>>,
    compressed_shape: GridShape,
    compressed_map: Box<[bool]>,
    compressed_interior_point: IntVector,
}

impl DigSite {
    fn from_instructions(instructions: &[DigInstruction]) -> Self {
        let mut dig_lines = Vec::<DigLine>::new();

        let mut current_position = IntVector::new(0, 0);
        for instruction in instructions {
            for _ in 0..instruction.distance {
                let delta: IntVector = instruction.direction.into();
                dig_lines.push(DigLine {
                    start: current_position,
                    direction: instruction.direction,
                    length: instruction.distance,
                });

                current_position += delta;
            }
        }

        fn coordinate_ranges(
            coordinates_on_axis: impl IntoIterator<Item = isize>,
        ) -> Vec<Range<isize>> {
            let coordinates_in_order = coordinates_on_axis
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect_vec()
                .pipe(|mut coords| {
                    coords.sort_unstable();
                    coords
                });

            let mut ranges = Vec::<Range<isize>>::new();
            let mut prev_coord = coordinates_in_order[0];
            ranges.push(prev_coord..prev_coord + 1);
            for coord in coordinates_in_order.into_iter().skip(1) {
                if coord - prev_coord > 1 {
                    // gap
                    ranges.push(prev_coord + 1..coord);
                }
                ranges.push(coord..coord + 1);
                prev_coord = coord;
            }
            ranges
        }

        let columns = coordinate_ranges(dig_lines.iter().map(|it| it.start.x));
        let rows = coordinate_ranges(dig_lines.iter().map(|it| it.start.y));

        let compressed_shape = GridShape {
            width: columns.len(),
            height: rows.len(),
        };
        let mut compressed_map = vec![false; compressed_shape.area()].into_boxed_slice();

        let mut current_position_uncompressed = IntVector::new(0, 0);
        for instruction in instructions {
            let current_position_compressed =
                get_compressed_position(current_position_uncompressed, &rows, &columns);
            let end_uncompressed = current_position_uncompressed
                + instruction.direction.as_vector() * instruction.distance as isize;
            let compressed_coords = match instruction.direction {
                Direction::North => {
                    let y_coords = rows
                        .iter()
                        .enumerate()
                        .rev()
                        .skip_while(|(_, range)| !range.contains(&current_position_uncompressed.y))
                        .take_while(|(_, range)| range.start > end_uncompressed.y)
                        .map(|it| IntVector::new(current_position_compressed.x, it.0 as isize))
                        .collect_vec();
                    y_coords
                }
                Direction::South => {
                    let y_coords = rows
                        .iter()
                        .enumerate()
                        .skip_while(|(_, range)| !range.contains(&current_position_uncompressed.y))
                        .take_while(|(_, range)| range.end <= end_uncompressed.y)
                        .map(|it| IntVector::new(current_position_compressed.x, it.0 as isize))
                        .collect_vec();
                    y_coords
                }
                Direction::East => {
                    let x_coords = columns
                        .iter()
                        .enumerate()
                        .skip_while(|(_, range)| !range.contains(&current_position_uncompressed.x))
                        .take_while(|(_, range)| range.end <= end_uncompressed.x)
                        .map(|it| IntVector::new(it.0 as isize, current_position_compressed.y))
                        .collect_vec();
                    x_coords
                }
                Direction::West => {
                    let x_coords = columns
                        .iter()
                        .enumerate()
                        .rev()
                        .skip_while(|(_, range)| !range.contains(&current_position_uncompressed.x))
                        .take_while(|(_, range)| range.start > end_uncompressed.x)
                        .map(|it| IntVector::new(it.0 as isize, current_position_compressed.y))
                        .collect_vec();
                    x_coords
                }
            };

            for coord in compressed_coords {
                compressed_map[compressed_shape.arr_index(coord)] = true;
            }
            current_position_uncompressed = end_uncompressed;
        }

        // lil hack to get the interior point
        // we can assume the dig lines run clockwise and the corners don't get too close to each other
        let dig_line = dig_lines[0];
        let interior_point = dig_line.start
            + match dig_line.direction {
                Direction::North => NORTH + EAST,
                Direction::South => SOUTH + WEST,
                Direction::East => EAST + SOUTH,
                Direction::West => WEST + NORTH,
            };
        let compressed_interior_point = get_compressed_position(interior_point, &rows, &columns);
        if compressed_map[compressed_shape.arr_index(compressed_interior_point)] {
            panic!("Interior point is a wall");
        }

        Self {
            rows,
            columns,
            compressed_shape,
            compressed_map,
            compressed_interior_point,
        }
    }

    fn get_compressed(&self, coord: IntVector) -> bool {
        let index = self.compressed_shape.arr_index(coord);
        self.compressed_map[index]
    }

    fn capacity(&self) -> usize {
        self.compressed_map
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, it)| *it)
            .map(|(compressed_index, _)| {
                let compressed_coord = self.compressed_shape.coordinate_for_index(compressed_index);
                let x_range = self.columns[compressed_coord.x as usize].clone();
                let y_range = self.rows[compressed_coord.y as usize].clone();
                x_range.len() * y_range.len() as usize
            })
            .sum()
    }

    fn dig_interior(&mut self) -> Result<()> {
        let mut queue = VecDeque::<IntVector>::new();
        queue.push_back(self.compressed_interior_point);
        while let Some(coord) = queue.pop_front() {
            if self.get_compressed(coord) {
                continue;
            }

            self.compressed_map[self.compressed_shape.arr_index(coord)] = true;
            let neighbors = coord.cardinal_neighbors();
            for neighbor in neighbors {
                if self.compressed_shape.in_bounds(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
        Ok(())
    }
}

fn get_compressed_position(
    position: IntVector,
    rows: &[Range<isize>],
    columns: &[Range<isize>],
) -> IntVector {
    let x = columns
        .iter()
        .enumerate()
        .find(|(_, range)| range.contains(&position.x))
        .map(|it| it.0 as isize)
        .expect(format!("No column found for {:?}", position).as_str());
    let y = rows
        .iter()
        .enumerate()
        .find(|(_, range)| range.contains(&position.y))
        .map(|it| it.0 as isize)
        .expect(format!("No column found for {:?}", position).as_str());
    IntVector::new(x, y)
}

impl Display for DigSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.compressed_shape.format_char_grid(
            self.compressed_shape.coord_iter().map(|coord| {
                if self.get_compressed(coord) {
                    '#'
                } else {
                    '.'
                }
            }),
        ))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct DigLine {
    start: IntVector,
    direction: Direction,
    length: usize,
}
impl DigLine {
    fn contains(&self, coord: IntVector) -> bool {
        match self.direction {
            Direction::North => {
                coord.x == self.start.x
                    && coord.y <= self.start.y
                    && coord.y >= self.start.y - self.length as isize
            }
            Direction::South => {
                coord.x == self.start.x
                    && coord.y >= self.start.y
                    && coord.y <= self.start.y + self.length as isize
            }
            Direction::East => {
                coord.y == self.start.y
                    && coord.x >= self.start.x
                    && coord.x <= self.start.x + self.length as isize
            }
            Direction::West => {
                coord.y == self.start.y
                    && coord.x <= self.start.x
                    && coord.x >= self.start.x - self.length as isize
            }
        }
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
            direction: Direction::East,
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
