// Day 10: Pipe Maze

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

use tap::Pipe;

use crate::framework::grid::{GridShape, IntVector, EAST, NORTH, SOUTH, WEST};
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
        Some(try_block(move || {
            puzzle_input()?.find_enclosed_tiles().map(|d| d.to_string())
        }))
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    shape: GridShape,
    tiles: Vec<Option<MetalPipe>>,
}

impl Grid {
    fn get(&self, coord: IntVector) -> Option<MetalPipe> {
        if !self.in_bounds(coord) {
            return None;
        }
        let index = self.shape.arr_index(coord);
        self.tiles.get(index).copied().flatten()
    }

    fn in_bounds(&self, coord: IntVector) -> bool {
        coord.x >= 0
            && coord.x < self.shape.width as isize
            && coord.y >= 0
            && coord.y < self.shape.height as isize
    }

    fn find_start_coordinate(&self) -> Option<IntVector> {
        self.tiles
            .iter()
            .position(|tile| tile == &Some(MetalPipe::Start))
            .map(|index| self.shape.coordinate_for_index(index))
    }

    fn find_kind_of_start(&self) -> Result<MetalPipe> {
        let start_coord = self
            .find_start_coordinate()
            .ok_or(anyhow!("no start coordinate exists"))?;
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
            (false, true, true, false) => Ok(MetalPipe::SE),
            (false, true, false, true) => Ok(MetalPipe::SW),
            // We don't support every kind of start, because the inputs don't require that
            (north, south, east, west) => Err(anyhow!(
                "Invalid start: {}{}{}{}",
                if north { "N" } else { "" },
                if south { "S" } else { "" },
                if east { "E" } else { "" },
                if west { "W" } else { "" },
            )),
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
        let attempt_1 = try_find_enclosed_tiles(self, false);
        if let Ok(result) = attempt_1 {
            return Ok(result);
        }
        let attempt_2 = try_find_enclosed_tiles(self, true);
        return attempt_2.map_err(|err| match err {
            FindEnclosedTilesError::OutOfBounds => anyhow!("Both directions went out of bounds"),
            FindEnclosedTilesError::Other(err) => err,
        });

        enum FindEnclosedTilesError {
            OutOfBounds,
            Other(anyhow::Error),
        }
        impl From<anyhow::Error> for FindEnclosedTilesError {
            fn from(value: anyhow::Error) -> Self {
                FindEnclosedTilesError::Other(value)
            }
        }

        fn try_find_enclosed_tiles(
            grid: &Grid,
            reverse_start_direction: bool,
        ) -> std::result::Result<u64, FindEnclosedTilesError> {
            let start = grid
                .find_start_coordinate()
                .ok_or(anyhow!("no start found"))?;
            let start_kind = grid.find_kind_of_start()?;

            let possible_start_directions: Box<[IntVector]> = start_kind
                .adjacent_directions()
                .iter()
                .map(|direction| direction.inverse())
                .collect();

            let start_direction = if reverse_start_direction {
                possible_start_directions[1]
            } else {
                possible_start_directions[0]
            };

            fn inside_directions(pipe: MetalPipe, direction: IntVector) -> Result<Vec<IntVector>> {
                // clockwise from the direction we're coming from
                match (pipe, direction) {
                    (MetalPipe::NS, NORTH) => Ok(vec![EAST]),
                    (MetalPipe::NS, SOUTH) => Ok(vec![WEST]),

                    (MetalPipe::EW, EAST) => Ok(vec![SOUTH]),
                    (MetalPipe::EW, WEST) => Ok(vec![NORTH]),

                    (MetalPipe::NE, WEST) => Ok(vec![]),
                    (MetalPipe::NE, SOUTH) => Ok(vec![SOUTH, WEST]),

                    (MetalPipe::NW, SOUTH) => Ok(vec![]),
                    (MetalPipe::NW, EAST) => Ok(vec![SOUTH, EAST]),

                    (MetalPipe::SW, EAST) => Ok(vec![]),
                    (MetalPipe::SW, NORTH) => Ok(vec![NORTH, EAST]),

                    (MetalPipe::SE, NORTH) => Ok(vec![]),
                    (MetalPipe::SE, WEST) => Ok(vec![NORTH, WEST]),

                    (pipe, direction) => Err(anyhow!(
                        "Can't resolve a {} ({:?}) going in direction {:?}",
                        pipe.to_char(),
                        pipe,
                        direction
                    )),
                }
            }

            let mut just_inside_tiles: HashSet<IntVector> = HashSet::new();
            let mut loop_tiles = HashSet::new();

            loop_tiles.insert(start);
            for start_inside_direction in inside_directions(start_kind, start_direction)? {
                just_inside_tiles.insert(start_inside_direction + start);
            }
            let mut current_direction = start_kind
                .adjacent_directions()
                .into_iter()
                .find(|&direction| direction != start_direction.inverse())
                .ok_or(anyhow!("Can't find the next direction to go from start"))?;
            let mut current_location = start + current_direction;

            while current_location != start {
                let current_pipe = grid
                    .get(current_location)
                    .ok_or(anyhow!("Broken pipe at {:?}", current_location))?;
                loop_tiles.insert(current_location);
                for inside_direction in inside_directions(current_pipe, current_direction)? {
                    just_inside_tiles.insert(current_location + inside_direction);
                }

                let new_direction = current_pipe
                    .adjacent_directions()
                    .into_iter()
                    .find(|&direction| direction != current_direction.inverse())
                    .ok_or(anyhow!("Can't find the next direction to go"))?;
                current_direction = new_direction;
                current_location += current_direction;
            }

            // for (y, row) in grid.tiles.chunks(grid.width).enumerate() {
            //     let row_str = row
            //         .iter()
            //         .enumerate()
            //         .map(|(x, _)| {
            //             let coord = IntVector::new(x as Units, y as Units);
            //             let pipe = if loop_tiles.contains(&coord) {
            //                 Some(match grid.get(coord) {
            //                     Some(pipe) => pipe.to_char(),
            //                     None => 'X',
            //                 })
            //             } else {
            //                 None
            //             };
            //             if let Some(pipe) = pipe {
            //                 pipe.to_owned()
            //             } else if just_inside_tiles.contains(&coord) {
            //                 'I'.into()
            //             } else {
            //                 '.'.into()
            //             }
            //         })
            //         .join("");
            //     println!("{}", row_str);
            // }

            let mut enclosed: HashSet<IntVector> = HashSet::new();
            let mut queue = VecDeque::from_iter(just_inside_tiles.iter().copied());
            while let Some(next) = queue.pop_front() {
                if !grid.in_bounds(next) {
                    return Err(FindEnclosedTilesError::OutOfBounds);
                }
                if enclosed.contains(&next) || loop_tiles.contains(&next) {
                    // stop if we've already visited this tile or if it's part of the loop
                    continue;
                }
                enclosed.insert(next);
                let neighbors = next.cardinal_neighbors();
                for neighbor in neighbors {
                    queue.push_back(neighbor);
                }
            }

            // for (y, row) in grid.tiles.chunks(grid.width).enumerate() {
            //     let row_str = row
            //         .iter()
            //         .enumerate()
            //         .map(|(x, _)| {
            //             let coord = IntVector::new(x as Units, y as Units);
            //             let pipe = if loop_tiles.contains(&coord) {
            //                 Some(match grid.get(coord) {
            //                     Some(pipe) => pipe.to_char(),
            //                     None => 'X',
            //                 })
            //             } else {
            //                 None
            //             };
            //             if let Some(pipe) = pipe {
            //                 pipe.to_owned()
            //             } else if just_inside_tiles.contains(&coord) {
            //                 'I'.into()
            //             } else if enclosed.contains(&coord) {
            //                 'i'.into()
            //             } else {
            //                 '.'.into()
            //             }
            //         })
            //         .join("");
            //     println!("{}", row_str);
            // }

            Ok(enclosed.len() as u64)
        }
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (shape, chars) = GridShape::parse_char_grid(s)?;
        let tiles: Vec<Option<MetalPipe>> = chars
            .into_iter()
            .map(|c| MetalPipe::from_char(*c))
            .collect::<Result<_>>()?;
        Ok(Grid { tiles, shape })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.chunks(self.shape.width) {
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
        assert_eq!(super::Day10.part1().unwrap().unwrap(), "7097".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::Day10.part2().unwrap().unwrap(), "355".to_string());
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
