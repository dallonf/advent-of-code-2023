// Day 11: Cosmic Expansion

use std::fmt::Display;
use std::str::FromStr;

use crate::framework::grid::GridShape;
use crate::framework::Day;
use crate::prelude::*;

pub struct Day11;

impl Day for Day11 {
    fn day_number(&self) -> u8 {
        11
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || Ok("Hello, world!".to_string())))
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
            "Hello, world!".to_string(),
        );
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
}
