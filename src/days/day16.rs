// Day 16: The Floor Will Be Lava

use std::str::FromStr;

use crate::framework::grid::{Direction, GridShape, IntVector};
use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<Contraption> {
    include_str!("./day16_input.txt").parse()
}

pub struct Day16;

impl Day for Day16 {
    fn day_number(&self) -> u8 {
        16
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?
                .energized_tiles_top_left()
                .to_string()
                .pipe(Ok)
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?.max_energized_tiles().to_string().pipe(Ok)
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    position: IntVector,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    /// "/"
    MirrorClockwise,
    /// "\\"
    MirrorCounterClockwise,
    // "-"
    SplitterHorizontal,
    // "|"
    SplitterVertical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Contraption {
    shape: GridShape,
    tiles: Box<[Tile]>,
}

impl Contraption {
    fn energized_tiles_top_left(&self) -> usize {
        self.energized_tiles(Beam {
            direction: Direction::East,
            position: IntVector::new(0, 0),
        })
    }

    fn energized_tiles(&self, starting_beam: Beam) -> usize {
        let mut energized_tiles_direction_bitmask = Box::<[u8]>::from(vec![0; self.tiles.len()]);

        let mut current_beams: Vec<Beam> = vec![starting_beam];

        while !current_beams.is_empty() {
            let mut next_beams: Vec<Beam> = vec![];
            for beam in current_beams.iter() {
                if !self.shape.in_bounds(beam.position) {
                    continue;
                }
                let tile_index = self.shape.arr_index(beam.position);
                if energized_tiles_direction_bitmask[tile_index] & beam.direction as u8 != 0 {
                    // Already energized from this direction.
                    continue;
                }
                energized_tiles_direction_bitmask[tile_index] |= beam.direction as u8;

                let current_tile = self.tiles[tile_index];
                match current_tile {
                    Tile::Empty => next_beams.push(Beam {
                        position: beam.position + beam.direction.into(),
                        direction: beam.direction,
                    }),
                    Tile::MirrorClockwise => {
                        let new_direction = match beam.direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::North,
                            Direction::South => Direction::West,
                            Direction::West => Direction::South,
                        };
                        next_beams.push(Beam {
                            position: beam.position + new_direction.into(),
                            direction: new_direction,
                        });
                    }
                    Tile::MirrorCounterClockwise => {
                        let new_direction = match beam.direction {
                            Direction::North => Direction::West,
                            Direction::East => Direction::South,
                            Direction::South => Direction::East,
                            Direction::West => Direction::North,
                        };
                        next_beams.push(Beam {
                            position: beam.position + new_direction.into(),
                            direction: new_direction,
                        });
                    }
                    Tile::SplitterHorizontal => {
                        let new_directions = match beam.direction {
                            Direction::North | Direction::South => {
                                vec![Direction::West, Direction::East]
                            }
                            original_direction => vec![original_direction],
                        };
                        for new_direction in new_directions {
                            next_beams.push(Beam {
                                position: beam.position + new_direction.into(),
                                direction: new_direction,
                            });
                        }
                    }
                    Tile::SplitterVertical => {
                        let new_directions = match beam.direction {
                            Direction::East | Direction::West => {
                                vec![Direction::North, Direction::South]
                            }
                            original_direction => vec![original_direction],
                        };
                        for new_direction in new_directions {
                            next_beams.push(Beam {
                                position: beam.position + new_direction.into(),
                                direction: new_direction,
                            });
                        }
                    }
                }
            }
            current_beams = next_beams;
        }

        energized_tiles_direction_bitmask
            .iter()
            .filter(|&&directions| directions != 0)
            .count()
    }

    fn max_energized_tiles(&self) -> usize {
        let north_beams = (0..self.shape.width).map(|x| Beam {
            position: IntVector::new(x as isize, 0),
            direction: Direction::South,
        });
        let south_beams = (0..self.shape.width).map(|x| Beam {
            position: IntVector::new(x as isize, self.shape.height as isize - 1),
            direction: Direction::North,
        });
        let east_beams = (0..self.shape.height).map(|y| Beam {
            position: IntVector::new(self.shape.width as isize - 1, y as isize),
            direction: Direction::West,
        });
        let west_beams = (0..self.shape.height).map(|y| Beam {
            position: IntVector::new(0, y as isize),
            direction: Direction::East,
        });
        let all_beams = north_beams
            .chain(south_beams)
            .chain(east_beams)
            .chain(west_beams)
            .collect_vec();

        all_beams
            .par_iter()
            .map(|&beam| self.energized_tiles(beam))
            .max()
            .unwrap_or(0)
    }
}

impl FromStr for Contraption {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (shape, chars) = GridShape::parse_char_grid(s)?;
        let tiles = chars
            .iter()
            .map(|&c| match c {
                '.' => Ok(Tile::Empty),
                '/' => Ok(Tile::MirrorClockwise),
                '\\' => Ok(Tile::MirrorCounterClockwise),
                '-' => Ok(Tile::SplitterHorizontal),
                '|' => Ok(Tile::SplitterVertical),
                _ => Err(anyhow!("Invalid tile: {}", c)),
            })
            .collect::<Result<_>>()?;

        Ok(Self { shape, tiles })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day16.part1().unwrap().unwrap(), "7728".to_string(),);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::Day16.part2().unwrap().unwrap(), "8061".to_string(),);
    }

    fn sample_input() -> Contraption {
        let input = indoc! {r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "};
        input.parse().unwrap()
    }

    #[test]
    fn test_energized_tiles() {
        let contraption = sample_input();
        let result = contraption.energized_tiles_top_left();
        assert_eq!(result, 46);
    }

    #[test]
    fn test_max_energized_tiles() {
        let contraption = sample_input();
        let result = contraption.max_energized_tiles();
        assert_eq!(result, 51);
    }
}
