// Day 10: Pipe Maze

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::ops::{Add, AddAssign};
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

    fn cardinal_neighbors(self) -> Vec<Self> {
        vec![
            IntVector::new(self.x - 1, self.y),
            IntVector::new(self.x + 1, self.y),
            IntVector::new(self.x, self.y - 1),
            IntVector::new(self.x, self.y + 1),
        ]
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

impl AddAssign for IntVector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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

    fn find_kind_of_start(&self) -> Option<MetalPipe> {
        let start_coord = self.find_start_coordinate()?;
        let points_back_to_start = |coord: IntVector| {
            if let Some(pipe) = self.get(coord) {
                let mut pipe_neighbors = pipe
                    .adjacent_directions()
                    .into_iter()
                    .map(|direction| direction + coord);
                pipe_neighbors.any(|coord| coord == start_coord)
            } else {
                return false;
            }
        };
        let north_points_back = points_back_to_start(start_coord + IntVector::new(0, -1));
        let south_points_back = points_back_to_start(start_coord + IntVector::new(0, 1));
        let east_points_back = points_back_to_start(start_coord + IntVector::new(1, 0));
        let west_points_back = points_back_to_start(start_coord + IntVector::new(-1, 0));
        match (
            north_points_back,
            south_points_back,
            east_points_back,
            west_points_back,
        ) {
            (true, true, false, false) => Some(MetalPipe::NS),
            (false, false, true, true) => Some(MetalPipe::EW),
            (true, false, true, false) => Some(MetalPipe::NE),
            (true, false, false, true) => Some(MetalPipe::NW),
            (false, true, false, true) => Some(MetalPipe::SW),
            (false, true, true, false) => Some(MetalPipe::SE),
            _ => None,
        }
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

    fn find_enclosed_tiles(&self) -> Result<u64> {
        let start = self
            .find_start_coordinate()
            .ok_or(anyhow!("no start found"))?;

        // corners don't have normals, so they'll be None
        let mut loop_inside_normals: HashMap<IntVector, Option<IntVector>> = HashMap::new();
        let mut current_location = start;
        let mut current_direction = IntVector::new(1, 0);
        while !loop_inside_normals.contains_key(&current_location) {
            let current_pipe = if current_location == start {
                self.find_kind_of_start()
                    .ok_or(anyhow!("Can't find the kind of start"))?
            } else {
                self.get(current_location)
                    .ok_or(anyhow!("Broken pipe at {:?}", current_location))?
            };
            let inside_normal = match (current_pipe, current_direction) {
                (MetalPipe::EW, IntVector { x: 1, y: 0 }) => Some(IntVector::new(0, 1)),
                (MetalPipe::EW, IntVector { x: -1, y: 0 }) => Some(IntVector::new(0, -1)),
                (MetalPipe::NS, IntVector { x: 0, y: 1 }) => Some(IntVector::new(-1, 0)),
                (MetalPipe::NS, IntVector { x: 0, y: -1 }) => Some(IntVector::new(1, 0)),
                _ => None,
            };
            loop_inside_normals.insert(current_location, inside_normal);

            let reverse_direction =
                IntVector::new(current_direction.x * -1, current_direction.y * -1);
            let new_direction = current_pipe
                .adjacent_directions()
                .into_iter()
                .find(|&direction| direction != reverse_direction)
                .ok_or(anyhow!("Can't find the next direction to go"))?;
            current_direction = new_direction;
            current_location += current_direction;
        }

        let mut enclosed: HashSet<IntVector> = HashSet::new();
        let immediate_inside_tiles = loop_inside_normals
            .iter()
            .filter_map(|(tile, inside_normal)| {
                if let Some(inside_normal) = inside_normal {
                    Some(*tile + *inside_normal)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut queue = VecDeque::from_iter(immediate_inside_tiles);
        let mut timeout = 100;
        while let Some(next) = queue.pop_front() {
            if timeout < 0 {
                return Err(anyhow!("timeout"));
            } else {
                timeout -= 1;
            }
            if enclosed.contains(&next) || loop_inside_normals.contains_key(&next) {
                // stop if we've already visited this tile or if it's part of the loop
                continue;
            }
            enclosed.insert(next);
            let neighbors = next.cardinal_neighbors();
            for neighbor in neighbors {
                queue.push_back(neighbor);
            }
        }

        for (y, row) in self.tiles.chunks(self.width).enumerate() {
            let row_str = row
                .iter()
                .enumerate()
                .map(|(x, _)| {
                    let loop_normal = loop_inside_normals
                        .get(&IntVector::new(x as Units, y as Units))
                        .map(|found| {
                            found
                                .map(|it| match it {
                                    IntVector { x: 1, y: 0 } => ">",
                                    IntVector { x: -1, y: 0 } => "<",
                                    IntVector { x: 0, y: 1 } => "V",
                                    IntVector { x: 0, y: -1 } => "^",
                                    _ => "X",
                                })
                                .unwrap_or("#".into())
                        });
                    if let Some(loop_normal) = loop_normal {
                        loop_normal.to_owned()
                    } else if enclosed.contains(&IntVector::new(x as Units, y as Units)) {
                        "I".into()
                    } else {
                        ".".into()
                    }
                })
                .join("");
            println!("{}", row_str);
        }

        Ok(enclosed.len() as u64)
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

    fn simple_enclosed() -> Grid {
        let input = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};
        Grid::from_str(input).unwrap()
    }

    fn complex_enclosed() -> Grid {
        let input = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        Grid::from_str(input).unwrap()
    }

    #[test]
    fn test_find_contained_tiles() {
        let grid = simple_enclosed();
        let result = grid.find_enclosed_tiles().unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_find_contained_tiles_complex() {
        let grid = complex_enclosed();
        let result = grid.find_enclosed_tiles().unwrap();
        assert_eq!(result, 10);
    }
}
