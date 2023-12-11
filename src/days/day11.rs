// Day 11: Cosmic Expansion

use std::fmt::Display;
use std::str::FromStr;

use crate::framework::grid::{GridShape, IntVector};
use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<Image> {
    Image::from_str(include_str!("./day11_input.txt"))
}

pub struct Day11;

impl Day for Day11 {
    fn day_number(&self) -> u8 {
        11
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            Ok(puzzle_input()?.expand().pair_distances().to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Image {
    pixels: Box<[bool]>,
    shape: GridShape,
}

impl Image {
    fn get(&self, coord: IntVector) -> bool {
        self.pixels[self.shape.arr_index(coord)]
    }

    fn expand(&self) -> Image {
        let rows: Box<[Box<[bool]>]> = (0..self.shape.height)
            .map(|y| Box::from(&self.pixels[(y * self.shape.width)..((y + 1) * self.shape.width)]))
            .collect();
        let mut num_new_rows = 0;
        let new_rows: Box<[Box<[bool]>]> = rows
            .into_iter()
            .flat_map(|row| {
                if row.iter().all(|pixel| !pixel) {
                    num_new_rows += 1;
                    vec![row.clone(), row.clone()]
                } else {
                    vec![row.clone()]
                }
            })
            .collect();
        let expanded = Image {
            pixels: new_rows
                .into_iter()
                .flat_map(|row| row.into_iter())
                .copied()
                .collect::<Box<[_]>>(),
            shape: GridShape {
                width: self.shape.width,
                height: self.shape.height + num_new_rows,
            },
        };
        let columns: Box<[Box<[bool]>]> = (0..expanded.shape.width)
            .map(|x| {
                expanded
                    .pixels
                    .iter()
                    .skip(x)
                    .step_by(expanded.shape.width)
                    .copied()
                    .collect::<Box<[bool]>>()
            })
            .collect();
        let mut num_new_columns = 0;
        let new_columns: Box<[Box<[bool]>]> = columns
            .into_iter()
            .flat_map(|column| {
                if column.iter().all(|pixel| !pixel) {
                    num_new_columns += 1;
                    vec![column.clone(), column.clone()]
                } else {
                    vec![column.clone()]
                }
            })
            .collect();
        let expanded = Image {
            shape: GridShape {
                width: expanded.shape.width + num_new_columns,
                height: expanded.shape.height,
            },
            pixels: {
                let mut pixels =
                    Vec::<bool>::with_capacity(expanded.shape.width * expanded.shape.height);
                for y in 0..expanded.shape.height {
                    for x in 0..(expanded.shape.width + num_new_columns) {
                        pixels.push(new_columns[x][y]);
                    }
                }
                pixels.into_boxed_slice()
            },
        };
        expanded
    }

    fn galaxy_pairs(&self) -> Vec<(IntVector, IntVector)> {
        let galaxies: Vec<IntVector> = self
            .pixels
            .iter()
            .enumerate()
            .filter(|(_, &pixel)| pixel)
            .map(|(index, _)| self.shape.coordinate_for_index(index))
            .collect();
        galaxies
            .into_iter()
            .combinations(2)
            .map(|combo| (combo[0], combo[1]))
            .collect()
    }

    fn pair_distances(&self) -> usize {
        let pairs = self.galaxy_pairs();
        pairs
            .into_iter()
            .map(|(a, b)| a.manhattan_distance(b))
            .sum()
    }
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (shape, chars) = GridShape::parse_char_grid(s)?;
        let pixels = chars
            .into_iter()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(anyhow!("invalid character: {}", c)),
            })
            .collect::<Result<Vec<_>>>()?
            .into_boxed_slice();
        Ok(Self { pixels, shape })
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .shape
                .format_char_grid(self.pixels.iter().map(|&b| if b { '#' } else { '.' })),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            super::Day11.part1().unwrap().unwrap(),
            "9609130".to_string(),
        );
    }

    fn sample_input() -> Image {
        let input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};
        Image::from_str(input).unwrap()
    }

    #[test]
    fn test_parsing() {
        let input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};
        let image = Image::from_str(input).unwrap();
        let reformatted = image.to_string();
        assert_eq!(reformatted, input);
    }

    #[test]
    fn test_expand() {
        let input = sample_input();
        let expected = indoc! {"
            ....#........
            .........#...
            #............
            .............
            .............
            ........#....
            .#...........
            ............#
            .............
            .............
            .........#...
            #....#.......
        "};
        let result = input.expand();
        assert_eq!(result.to_string(), expected);
    }

    #[test]
    fn test_galaxy_pairs() {
        assert_eq!(sample_input().expand().galaxy_pairs().len(), 36);
    }

    #[test]
    fn test_pair_distances() {
        assert_eq!(sample_input().expand().pair_distances(), 374);
    }
}
