// Day 14: Parabolic Reflector Dish

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use crate::framework::grid::{GridShape, IntVector, EAST, NORTH, SOUTH, WEST};
use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<Platform> {
    include_str!("./day14_input.txt").parse()
}

pub struct Day14;

impl Day for Day14 {
    fn day_number(&self) -> u8 {
        14
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let mut platform = puzzle_input()?;
            platform.slide_north_fully()?;
            Ok(platform.total_load().to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let mut platform = puzzle_input()?;
            platform.spin_cycle_repeat(1_000_000_000)?;
            Ok(platform.total_load().to_string())
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cube,
}

impl Rock {
    fn from_char(c: char) -> Result<Option<Rock>> {
        match c {
            'O' => Ok(Some(Rock::Round)),
            '#' => Ok(Some(Rock::Cube)),
            '.' => Ok(None),
            c => Err(anyhow!("Unexpected character: {c}")),
        }
    }

    fn to_char(self) -> char {
        match self {
            Rock::Round => 'O',
            Rock::Cube => '#',
        }
    }

    fn cell_to_char(cell: Option<Self>) -> char {
        match cell {
            Some(rock) => rock.to_char(),
            None => '.',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Platform {
    shape: GridShape,
    rocks: Box<[Option<Rock>]>,
}

impl Platform {
    /// Panics if out of bounds
    fn get(&self, coord: IntVector) -> Option<Rock> {
        let index = self.shape.arr_index(coord);
        self.rocks[index]
    }

    fn slide(&mut self, direction: IntVector) -> Result<()> {
        let rocks_to_slide = self
            .rocks
            .iter()
            .enumerate()
            .filter_map(|(i, it)| match it {
                Some(Rock::Round) => Some(i),
                _ => None,
            })
            .filter_map(|i| {
                let coord = self.shape.coordinate_for_index(i);
                let target = coord + direction;
                if self.shape.in_bounds(target) && self.get(target) == None {
                    Some((coord, target))
                } else {
                    None
                }
            })
            .collect_vec();
        for (from, to) in rocks_to_slide {
            self.rocks[self.shape.arr_index(from)] = None;
            self.rocks[self.shape.arr_index(to)] = Some(Rock::Round);
        }
        Ok(())
    }

    fn slide_north_fully(&mut self) -> Result<()> {
        self.slide_fully(NORTH)
    }

    fn slide_fully(&mut self, direction: IntVector) -> Result<()> {
        let mut previous = self.clone();
        loop {
            self.slide(direction)?;
            if *self == previous {
                break;
            }
            previous = self.clone();
        }
        Ok(())
    }

    fn total_load(&self) -> u64 {
        self.rocks
            .iter()
            .enumerate()
            .filter_map(|(i, it)| {
                if *it == Some(Rock::Round) {
                    let coord = self.shape.coordinate_for_index(i);
                    let rows_below = self.shape.height as u64 - coord.y as u64;
                    Some(rows_below)
                } else {
                    None
                }
            })
            .sum()
    }

    fn spin_cycle(&mut self) -> Result<()> {
        self.slide_fully(NORTH)?;
        self.slide_fully(WEST)?;
        self.slide_fully(SOUTH)?;
        self.slide_fully(EAST)?;
        Ok(())
    }

    fn spin_cycle_repeat(&mut self, times: usize) -> Result<()> {
        let mut sequence_so_far = Vec::<Box<[Option<Rock>]>>::new();
        let mut seen_states = HashMap::<u64, usize>::new();
        let mut sequence: Option<(usize, usize)> = None;
        for i in 0..times {
            let state_hash = {
                let mut hasher = DefaultHasher::new();
                self.rocks.hash(&mut hasher);
                hasher.finish()
            };
            if let Some(seen_state) = seen_states.get(&state_hash) {
                sequence = Some((*seen_state, i));
                break;
            }
            seen_states.insert(state_hash, i);
            sequence_so_far.push(self.rocks.clone());
            self.spin_cycle()?;
        }
        if let Some((sequence_start, sequence_end)) = sequence {
            let looping_sequence = &sequence_so_far[sequence_start..sequence_end];
            let cycles_left = times - sequence_end;
            let final_state = &looping_sequence[cycles_left % looping_sequence.len()];
            self.rocks = final_state.clone();
        }
        Ok(())
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (shape, chars) = GridShape::parse_char_grid(s)?;
        let rocks = chars
            .into_iter()
            .map(|c| Rock::from_char(*c))
            .collect::<Result<_>>()?;
        Ok(Platform { shape, rocks })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self
            .shape
            .format_char_grid(self.rocks.iter().map(|cell| Rock::cell_to_char(*cell)));
        f.write_str(&grid)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day14.part1().unwrap().unwrap(), "107430".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::Day14.part2().unwrap().unwrap(), "96317".to_string());
    }

    fn sample_input() -> Platform {
        indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
      "}
        .parse()
        .unwrap()
    }

    #[test]
    fn test_slide() {
        let mut platform = sample_input();
        platform.slide_north_fully().unwrap();
        let expected = indoc! {"
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
        "};
        assert_eq!(platform.to_string(), expected);
    }

    #[test]
    fn test_total_load() {
        let mut platform = sample_input();
        platform.slide_north_fully().unwrap();
        assert_eq!(platform.total_load(), 136);
    }

    #[test]
    fn test_spin_cycle() {
        let mut platform = sample_input();
        platform.spin_cycle().unwrap();
        let expected = indoc! {"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
        "};
        assert_eq!(platform.to_string(), expected);
    }

    #[test]
    fn test_spin_cycle_repeat() {
        let mut platform = sample_input();
        platform.spin_cycle_repeat(1_000_000_000).unwrap();
        assert_eq!(platform.total_load(), 64);
    }
}
