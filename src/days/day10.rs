// Day 10: Pipe Maze

use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;

use tap::Pipe;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day10;

fn puzzle_input() -> Result<Grid> {
    let input = include_str!("./day10_input.txt");
    Grid::from_str(input)
}

impl Day for Day10 {
    fn day_number(&self) -> u8 {
        10
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?
                .find_farthest_distance_from_start()
                .map(|d| d.to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum MetalPipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
}

impl MetalPipe {
    fn from_char(c: char) -> Result<Option<Self>> {
        match c {
            '|' => Ok(Some(MetalPipe::NS)),
            '-' => Ok(Some(MetalPipe::EW)),
            'L' => Ok(Some(MetalPipe::NE)),
            'J' => Ok(Some(MetalPipe::NW)),
            '7' => Ok(Some(MetalPipe::SW)),
            'F' => Ok(Some(MetalPipe::SE)),
            'S' => Ok(Some(MetalPipe::Start)),
            '.' => Ok(None),
            c => Err(anyhow!("Invalid pipe character: {}", c)),
        }
    }

    fn to_char(self) -> char {
        match self {
            MetalPipe::NS => '|',
            MetalPipe::EW => '-',
            MetalPipe::NE => 'L',
            MetalPipe::NW => 'J',
            MetalPipe::SW => '7',
            MetalPipe::SE => 'F',
            MetalPipe::Start => 'S',
        }
    }

    fn adjacent_directions(self) -> Vec<IntVector> {
        match self {
            MetalPipe::NS => vec![IntVector::new(0, -1), IntVector::new(0, 1)],
            MetalPipe::EW => vec![IntVector::new(-1, 0), IntVector::new(1, 0)],
            MetalPipe::NE => vec![IntVector::new(0, -1), IntVector::new(1, 0)],
            MetalPipe::NW => vec![IntVector::new(-1, 0), IntVector::new(0, -1)],
            MetalPipe::SW => vec![IntVector::new(0, 1), IntVector::new(-1, 0)],
            MetalPipe::SE => vec![IntVector::new(1, 0), IntVector::new(0, 1)],
            MetalPipe::Start => vec![
                IntVector::new(0, -1),
                IntVector::new(0, 1),
                IntVector::new(-1, 0),
                IntVector::new(1, 0),
            ],
        }
    }
}

type Units = i32;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct IntVector {
    x: Units,
    y: Units,
}

impl IntVector {
    fn new(x: Units, y: Units) -> Self {
        Self { x, y }
    }
}

impl Add for IntVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    width: usize,
    tiles: Vec<Option<MetalPipe>>,
}

impl Grid {
    fn get(&self, coord: IntVector) -> Option<MetalPipe> {
        let index = self.index(coord);
        self.tiles.get(index).copied().flatten()
    }

    fn index(&self, coord: IntVector) -> usize {
        (coord.y * self.width as Units + coord.x) as usize
    }

    fn coordinate_for_index(&self, index: usize) -> IntVector {
        let x = index % self.width;
        let y = index / self.width;
        IntVector::new(x as Units, y as Units)
    }

    fn find_start_coordinate(&self) -> Option<IntVector> {
        self.tiles
            .iter()
            .position(|tile| tile == &Some(MetalPipe::Start))
            .map(|index| self.coordinate_for_index(index))
    }

    fn find_farthest_distance_from_start(&self) -> Result<usize> {
        let start = self
            .find_start_coordinate()
            .ok_or(anyhow!("no start found"))?;

        let mut visited_with_distances: HashMap<IntVector, usize> = HashMap::new();
        visited_with_distances.insert(start, 0);

        let mut queue: VecDeque<(IntVector, usize)> = VecDeque::new();

        let adjacent_to_start = MetalPipe::Start
            .adjacent_directions()
            .into_iter()
            .map(|direction| start + direction)
            .filter(|neighbor_coord| {
                let pipe = if let Some(pipe) = self.get(*neighbor_coord) {
                    pipe
                } else {
                    return false;
                };
                let mut pipe_neighbors = pipe
                    .adjacent_directions()
                    .into_iter()
                    .map(|direction| direction + *neighbor_coord);
                let points_back_to_start = pipe_neighbors.any(|coord| coord == start);
                points_back_to_start
            });
        for coord in adjacent_to_start {
            queue.push_back((coord, 1));
        }

        while let Some((coord, distance)) = queue.pop_front() {
            // skip if we've already visited this coord with a shorter distance
            if visited_with_distances
                .get(&coord)
                .map(|visited_distance| *visited_distance <= distance)
                .unwrap_or(false)
            {
                continue;
            }

            visited_with_distances.insert(coord, distance);

            let neighbors = self
                .get(coord)
                .ok_or(anyhow!("Broken pipe at {:?}", coord))?
                .adjacent_directions();
            for neighbor in neighbors {
                let neighbor_coord = coord + neighbor;
                let neighbor_distance = distance + 1;
                queue.push_back((neighbor_coord, neighbor_distance));
            }
        }

        visited_with_distances
            .values()
            .max()
            .copied()
            .unwrap()
            .pipe(Ok)
    }
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
        let tiles: Vec<Option<MetalPipe>> = chars
            .into_iter()
            .map(|c| MetalPipe::from_char(c))
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
        assert_eq!(super::Day10.part1().unwrap().unwrap(), "7097".to_string(),);
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

    fn simple_input() -> Grid {
        let input = indoc! {"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};
        Grid::from_str(input).unwrap()
    }

    fn complex_input() -> Grid {
        let input = indoc! {"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "};
        Grid::from_str(input).unwrap()
    }

    #[test]
    fn test_find_start_position() {
        let grid = simple_input();
        let result = grid.find_start_coordinate().unwrap();
        assert_eq!(result, IntVector::new(1, 1));
    }

    #[test]
    fn test_find_farthest_distance_simple() {
        let grid = simple_input();
        let result = grid.find_farthest_distance_from_start().unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_find_farthest_distance_complex() {
        let grid = complex_input();
        let result = grid.find_farthest_distance_from_start().unwrap();
        assert_eq!(result, 8);
    }
}
