// Day 13: Point of Incidence

use std::fmt::Display;
use std::str::FromStr;

use crate::framework::grid::{GridShape, IntVector};
use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<Box<[Pattern]>> {
    include_str!("./day13_input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|pattern_str| pattern_str.parse())
        .collect()
}

pub struct Day13;

impl Day for Day13 {
    fn day_number(&self) -> u8 {
        13
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?
                .iter()
                .map(|pattern| pattern.reflection_score())
                .sum::<usize>()
                .to_string()
                .pipe(Ok)
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    shape: GridShape,
    rocks: Box<[bool]>,
}

impl Pattern {
    fn row(&self, y: usize) -> &[bool] {
        let left_coord = self.shape.arr_index(IntVector::new(0, y as isize));
        let right_coord = self
            .shape
            .arr_index(IntVector::new(self.shape.width as isize, y as isize));
        &self.rocks[left_coord..right_coord]
    }
    fn column(&self, x: usize) -> Box<[bool]> {
        // TODO: memoize
        (0..(self.shape.height))
            .map(|y| self.rocks[self.shape.arr_index(IntVector::new(x as isize, y as isize))])
            .collect()
    }

    fn vertical_reflection(&self) -> Option<usize> {
        for x in 1..self.shape.width {
            let reflection_size = usize::min(x, self.shape.width - x);
            let left_columns: Box<[Box<[bool]>]> = (x - reflection_size..x)
                .map(|column_x| self.column(column_x))
                .collect();
            let right_columns: Box<[Box<[bool]>]> = (x..x + reflection_size)
                .rev()
                .map(|column_x| self.column(column_x))
                .collect();

            if left_columns == right_columns {
                return Some(x);
            }
        }
        None
    }

    fn horizontal_reflection(&self) -> Option<usize> {
        for y in 1..self.shape.height {
            let reflection_size = usize::min(y, self.shape.height - y);
            let top_rows: Box<[&[bool]]> = (y - reflection_size..y)
                .map(|row_y| self.row(row_y))
                .collect();
            let bottom_rows: Box<[&[bool]]> = (y..y + reflection_size)
                .rev()
                .map(|row_y| self.row(row_y))
                .collect();

            if top_rows == bottom_rows {
                return Some(y);
            }
        }
        None
    }

    fn reflection_score(&self) -> usize {
        if let Some(vertical) = self.vertical_reflection() {
            return vertical;
        }
        if let Some(horizontal) = self.horizontal_reflection() {
            return horizontal * 100;
        }
        return 0;
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (shape, chars) = GridShape::parse_char_grid(s)?;
        let rocks = chars
            .into_iter()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(anyhow!("invalid character in pattern: {:?}", c)),
            })
            .collect::<Result<_>>()?;

        Ok(Pattern { shape, rocks })
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self
            .shape
            .format_char_grid(self.rocks.iter().map(|bool| match bool {
                true => '#',
                false => '.',
            }));
        f.write_str(&grid)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day13.part1().unwrap().unwrap(), "28895".to_string());
    }

    fn sample_with_vertical() -> Pattern {
        Pattern::from_str(indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
        "})
        .unwrap()
    }

    fn sample_with_horizontal() -> Pattern {
        Pattern::from_str(indoc! {"
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "})
        .unwrap()
    }

    #[test]
    fn test_vertical_reflection() {
        let pattern = sample_with_vertical();
        let reflected = pattern.vertical_reflection();
        assert_eq!(reflected, Some(5));
    }

    #[test]
    fn test_horizontal_reflection() {
        let pattern = sample_with_horizontal();
        let reflected = pattern.horizontal_reflection();
        assert_eq!(reflected, Some(4));
    }

    #[test]
    fn test_reflection_score() {
        assert_eq!(sample_with_vertical().reflection_score(), 5);
        assert_eq!(sample_with_horizontal().reflection_score(), 400);
    }
}
