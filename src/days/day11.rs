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
            Ok(puzzle_input()?.expand_once().pair_distances().to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            Ok(puzzle_input()?
                .expand(1_000_000)
                .pair_distances()
                .to_string())
        }))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Image {
    galaxies: Vec<IntVector>,
    shape: GridShape,
}

impl Image {
    fn galaxy_at(&self, coord: IntVector) -> bool {
        self.galaxies.contains(&coord)
    }

    fn expand_once(&self) -> Image {
        self.expand(2) // that is, every empty line becomes 2 empty lines
    }

    fn expand(&self, expansion_factor: usize) -> Image {
        assert!(expansion_factor > 0);
        let expand_to = expansion_factor - 1; // the number of new lines to add
        let rows: Box<[Box<[bool]>]> = (0..self.shape.height)
            .map(|y| {
                (0..self.shape.width)
                    .map(|x| self.galaxy_at(IntVector::new(x as isize, y as isize)))
                    .collect()
            })
            .collect();
        let expanded_row_indices: Vec<usize> = rows
            .into_iter()
            .enumerate()
            .filter_map(|(y, row)| {
                if row.iter().all(|pixel| !pixel) {
                    Some(y)
                } else {
                    None
                }
            })
            .collect();
        let columns: Box<[Box<[bool]>]> = (0..self.shape.width)
            .map(|x| {
                (0..self.shape.height)
                    .map(|y| self.galaxy_at(IntVector::new(x as isize, y as isize)))
                    .collect()
            })
            .collect();
        let expanded_column_indices: Vec<usize> = columns
            .into_iter()
            .enumerate()
            .filter_map(|(x, column)| {
                if column.iter().all(|pixel| !pixel) {
                    Some(x)
                } else {
                    None
                }
            })
            .collect();
        let expanded = Image {
            shape: GridShape {
                width: self.shape.width + expanded_column_indices.len() * expand_to,
                height: self.shape.height + expanded_row_indices.len() * expand_to,
            },
            galaxies: self
                .galaxies
                .iter()
                .map(|galaxy| {
                    let rows_expanded_before = expanded_row_indices
                        .iter()
                        .filter(|row_index| galaxy.y >= **row_index as isize)
                        .count();
                    let columns_expanded_before = expanded_column_indices
                        .iter()
                        .filter(|column_index| galaxy.x >= **column_index as isize)
                        .count();
                    IntVector::new(
                        galaxy.x + (columns_expanded_before * expand_to) as isize,
                        galaxy.y + (rows_expanded_before * expand_to) as isize,
                    )
                })
                .collect(),
        };
        expanded
    }

    fn galaxy_pairs(&self) -> Vec<(IntVector, IntVector)> {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|combo| (*combo[0], *combo[1]))
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
        let galaxies = pixels
            .iter()
            .enumerate()
            .filter(|(_, &pixel)| pixel)
            .map(|(index, _)| shape.coordinate_for_index(index))
            .collect::<Vec<_>>();
        Ok(Self { galaxies, shape })
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .shape
                .format_char_grid(self.shape.coord_iter().map(|coord| {
                    if self.galaxy_at(coord) {
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
            super::Day11.part1().unwrap().unwrap(),
            "9609130".to_string(),
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            super::Day11.part2().unwrap().unwrap(),
            "702152204842".to_string(),
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
        let result = input.expand_once();
        assert_eq!(result.to_string(), expected);
    }

    #[test]
    fn test_noop_expand() {
        let input = sample_input();
        let result = input.expand(1);
        assert_eq!(result.to_string(), input.to_string());
    }

    #[test]
    fn test_galaxy_pairs() {
        assert_eq!(sample_input().expand_once().galaxy_pairs().len(), 36);
    }

    #[test]
    fn test_pair_distances() {
        assert_eq!(sample_input().expand_once().pair_distances(), 374);
    }

    #[test]
    fn test_triple_expansion() {
        let input = sample_input();
        let expected = indoc! {"
            .....#..........
            ...........#....
            #...............
            ................
            ................
            ................
            ..........#.....
            .#..............
            ...............#
            ................
            ................
            ................
            ...........#....
            #.....#.........
        "};
        let result = input.expand(3);
        println!("{}", result.to_string());
        assert_eq!(result.to_string(), expected);
    }

    #[test]
    fn test_larger_expansion() {
        assert_eq!(sample_input().expand(10).pair_distances(), 1030);
        assert_eq!(sample_input().expand(100).pair_distances(), 8410);
    }
}
