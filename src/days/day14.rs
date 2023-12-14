// Day 14: Parabolic Reflector Dish

use std::fmt::Display;
use std::str::FromStr;

use crate::framework::grid::{GridShape, IntVector, NORTH};
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
        None
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

    fn slide_north(&mut self) -> Result<()> {
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
                let north = coord + NORTH;
                if self.shape.in_bounds(north) && self.get(north) == None {
                    Some((coord, north))
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
        let mut previous = self.clone();
        loop {
            self.slide_north()?;
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
        assert_eq!(super::Day14.part1().unwrap().unwrap(), "107430".to_string(),);
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
}
