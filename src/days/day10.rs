// Day 10 - Template

use std::fmt::Display;
use std::str::FromStr;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day10;

impl Day for Day10 {
    fn day_number(&self) -> u8 {
        10
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || Ok("Hello, world!".to_string())))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    /// Starting position (S)
    Unknown,
}

impl Pipe {
    fn from_char(c: char) -> Result<Option<Self>> {
        match c {
            '|' => Ok(Some(Pipe::NS)),
            '-' => Ok(Some(Pipe::EW)),
            'L' => Ok(Some(Pipe::NE)),
            'J' => Ok(Some(Pipe::NW)),
            '7' => Ok(Some(Pipe::SW)),
            'F' => Ok(Some(Pipe::SE)),
            'S' => Ok(Some(Pipe::Unknown)),
            '.' => Ok(None),
            c => Err(anyhow!("Invalid pipe character: {}", c)),
        }
    }

    fn to_char(self) -> char {
        match self {
            Pipe::NS => '|',
            Pipe::EW => '-',
            Pipe::NE => 'L',
            Pipe::NW => 'J',
            Pipe::SW => '7',
            Pipe::SE => 'F',
            Pipe::Unknown => 'S',
        }
    }
}

type Units = i32;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vector {
    x: Units,
    y: Units,
}

impl Vector {
    fn new(x: Units, y: Units) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    width: usize,
    tiles: Vec<Option<Pipe>>,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect::<Vec<_>>();
        let width = lines.first().ok_or(anyhow!("empty grid"))?.chars().count();
        let chars: Vec<char> = lines
            .iter()
            .map(|line| {
                let chars = line.chars().collect::<Vec<char>>();
                if chars.len() != width {
                    return Err(anyhow!(
                        "inconsistent line width - expected {}, got {} ({})",
                        width,
                        chars.len(),
                        line
                    ));
                }
                Ok(chars)
            })
            .collect::<Result<Vec<Vec<char>>>>()?
            .into_iter()
            .flatten()
            .collect();
        let tiles: Vec<Option<Pipe>> = chars
            .into_iter()
            .map(|c| Pipe::from_char(c))
            .collect::<Result<_>>()?;
        Ok(Grid { tiles, width })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.chunks(self.width) {
            for tile in row {
                match tile {
                    Some(pipe) => write!(f, "{}", pipe.to_char())?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            super::Day10.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }

    #[test]
    fn test_parse() {
        let input = indoc! {"
          7-F7-
          .FJ|7
          SJLL7
          |F--J
          LJ.LJ
        "};
        let grid = Grid::from_str(input).unwrap();
        assert_eq!(grid.to_string(), input);
    }
}
