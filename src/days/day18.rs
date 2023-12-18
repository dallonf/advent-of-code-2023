// Day 18: Lavaduct Lagoon

use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
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
        let interior_point = {
            let boundaries = self
                .map
                .iter()
                .copied()
                .enumerate()
                // DIRTY DIRTY hack
                // There's a problem handling the top of the puzzle input map
                // where the first row looks like ....######.....
                // and anything left of the wall is considered "inside".
                // bottom row is fine tho...
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

        let mut queue = VecDeque::<IntVector>::new();
        queue.push_back(interior_point);
        while let Some(coord) = queue.pop_front() {
            if self.get(coord) {
                continue;
            }

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

struct VirtualDigSite {
    lines: Vec<DigLine>,
    shape: SignedGridShape,
}

impl VirtualDigSite {
    fn from_instructions(instructions: &[DigInstruction]) -> VirtualDigSite {
        let mut current_position = IntVector::new(0, 0);
        let mut bound_top_left = current_position;
        let mut bound_bottom_right = current_position;
        let lines = instructions
            .iter()
            .map(|instruction| {
                let direction = match instruction.direction {
                    DigDirection::Up => Direction::North,
                    DigDirection::Down => Direction::South,
                    DigDirection::Left => Direction::West,
                    DigDirection::Right => Direction::East,
                };
                let length = instruction.distance;
                let start = current_position;
                current_position += direction.conv::<IntVector>() * length as isize;
                if current_position.x < bound_top_left.x {
                    bound_top_left.x = current_position.x;
                }
                if current_position.x > bound_bottom_right.x {
                    bound_bottom_right.x = current_position.x;
                }
                if current_position.y < bound_top_left.y {
                    bound_top_left.y = current_position.y;
                }
                if current_position.y > bound_bottom_right.y {
                    bound_bottom_right.y = current_position.y;
                }
                DigLine {
                    start,
                    direction,
                    length,
                }
            })
            .collect();
        VirtualDigSite {
            lines,
            shape: SignedGridShape::new(bound_top_left, bound_bottom_right),
        }
    }

    fn subsector(&self, new_bounds: SignedGridShape) -> Self {
        let lines = self
            .lines
            .iter()
            .filter_map(|line| {
                let start_contained = new_bounds.is_in_bounds(line.start);
                let end = line.start + line.direction.as_vector() * line.length as isize;
                let end_contained = new_bounds.is_in_bounds(end);

                if !start_contained && !end_contained {
                    return None;
                }
                if start_contained && end_contained {
                    return Some(line.clone());
                }

                let new_start = if start_contained {
                    line.start
                } else {
                    match line.direction {
                        Direction::North => IntVector::new(line.start.x, new_bounds.bottom_right.y),
                        Direction::South => IntVector::new(line.start.x, new_bounds.top_left.y),
                        Direction::East => IntVector::new(new_bounds.top_left.x, line.start.y),
                        Direction::West => IntVector::new(new_bounds.bottom_right.x, line.start.y),
                    }
                };
                let start_difference = new_start.manhattan_distance(line.start);
                let new_length = line.length - start_difference;
                let new_end = match line.direction {
                    Direction::North => IntVector::new(
                        new_start.x,
                        (new_start.y - new_length as isize).max(new_bounds.top_left.y),
                    ),
                    Direction::South => IntVector::new(
                        new_start.x,
                        (new_start.y + new_length as isize).min(new_bounds.bottom_right.y),
                    ),
                    Direction::East => IntVector::new(
                        (new_start.x + new_length as isize).min(new_bounds.bottom_right.x),
                        new_start.y,
                    ),
                    Direction::West => IntVector::new(
                        (new_start.x - new_length as isize).max(new_bounds.top_left.x),
                        new_start.y,
                    ),
                };
                let new_length = new_start.manhattan_distance(new_end);

                Some(DigLine {
                    start: new_start,
                    direction: line.direction,
                    length: new_length,
                })
            })
            .collect_vec();

        VirtualDigSite {
            lines,
            shape: new_bounds,
        }
    }

    fn perimeter(&self) -> usize {
        self.lines.iter().map(|line| line.length).sum()
    }

    fn area(&self) -> usize {
        // it's square if all the contained lines are along the edge
        let is_square = self.lines.iter().all(|line| match line.direction {
            Direction::North | Direction::South => {
                let line_x = line.start.x;
                line_x == self.shape.top_left.x || line_x == self.shape.bottom_right.x
            }
            Direction::East | Direction::West => {
                let line_y = line.start.y;
                line_y == self.shape.top_left.y || line_y == self.shape.bottom_right.y
            }
        });
        if is_square {
            return self.shape.bottom_right.x.abs_diff(self.shape.top_left.x)
                + 1 * self.shape.bottom_right.y.abs_diff(self.shape.top_left.y)
                + 1;
        }

        let midpoint = IntVector::new(
            (self.shape.bottom_right.x + self.shape.top_left.x) / 2,
            (self.shape.bottom_right.y + self.shape.top_left.y) / 2,
        );

        let split_vertex = self
            .lines
            .iter()
            // .filter(|line| {
            //     // don't choose a vertex right on the edge
            //     line.start.x != self.shape.bottom_right.x
            //         && line.start.y != self.shape.bottom_right.y
            //         && line.start.x != self.shape.top_left.x
            //         && line.start.y != self.shape.top_left.y
            // })
            .min_by_key(|line| line.start.manhattan_distance(midpoint))
            .expect("nothing to split on");

        dbg!(split_vertex);
        let subsector_1_shape = match split_vertex.direction {
            Direction::North => SignedGridShape::new(
                self.shape.top_left,
                IntVector::new(self.shape.bottom_right.x, split_vertex.start.y + 1),
            ),
            Direction::South => SignedGridShape::new(
                IntVector::new(self.shape.top_left.x, split_vertex.start.y),
                self.shape.bottom_right,
            ),
            Direction::East => SignedGridShape::new(
                IntVector::new(split_vertex.start.x, self.shape.top_left.y),
                self.shape.bottom_right,
            ),
            Direction::West => SignedGridShape::new(
                self.shape.top_left,
                IntVector::new(split_vertex.start.x + 1, self.shape.bottom_right.y),
            ),
        };
        if subsector_1_shape == self.shape {
            panic!("shape didn't change");
        }
        let subsector_1 = self.subsector(subsector_1_shape);

        println!("{}", subsector_1);

        return subsector_1.area();
        return 0;
    }
}

impl Display for VirtualDigSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = self
            .shape
            .format_char_grid(self.shape.coord_iter().map(|coord| {
                let matching_line = self.lines.iter().find(|line| line.contains(coord));
                let matching_line_direction = matching_line.map(|it| it.direction);
                match matching_line_direction {
                    Some(Direction::North) => '^',
                    Some(Direction::South) => 'V',
                    Some(Direction::East) => '>',
                    Some(Direction::West) => '<',
                    None => '.',
                }
            }));
        f.write_str(&formatted)
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

    #[test]
    fn test_virtual_perimeter() {
        let virtual_dig_site = VirtualDigSite::from_instructions(&sample_input());
        assert_eq!(virtual_dig_site.perimeter(), 38);
    }

    #[test]
    fn test_virtual_area() {
        let virtual_dig_site = VirtualDigSite::from_instructions(&sample_input());
        assert_eq!(virtual_dig_site.area(), 62);
    }
}
