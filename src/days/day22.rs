// Day 22: Sand Slabs

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::framework::grid::{GridShape, IntVector};
use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<World> {
    let input = include_str!("./day22_input.txt");
    input.parse()
}

pub struct Day22;

impl Day for Day22 {
    fn day_number(&self) -> u8 {
        22
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let mut world = puzzle_input()?;
            world.apply_gravity();
            Ok(world.find_non_load_bearing_bricks().to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IntVector3 {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for IntVector3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s
            .split(',')
            .map(|part| part.parse::<isize>().map_err(|err| anyhow!(err)))
            .collect::<Result<Vec<_>>>()?;
        if parts.len() != 3 {
            return Err(anyhow!("Expected 3 parts, got {}", parts.len()));
        }
        Ok(Self {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick(IntVector3, IntVector3);

impl Brick {
    fn lowest_z(&self) -> isize {
        self.0.z.min(self.1.z)
    }

    fn highest_z(&self) -> isize {
        self.0.z.max(self.1.z)
    }

    fn x_range(&self) -> RangeInclusive<isize> {
        self.0.x.min(self.1.x)..=self.0.x.max(self.1.x)
    }

    fn y_range(&self) -> RangeInclusive<isize> {
        self.0.y.min(self.1.y)..=self.0.y.max(self.1.y)
    }

    fn z_range(&self) -> RangeInclusive<isize> {
        self.0.z.min(self.1.z)..=self.0.z.max(self.1.z)
    }
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (a, b) = s.split_once('~').ok_or(anyhow!("Invalid Brick: {}", s))?;
        Ok(Self(a.parse()?, b.parse()?))
    }
}

struct World {
    /// Should always be sorted by lowest_z in ascending order.
    bricks: Vec<(usize, Brick)>,
    /// Key: brick ID, Value: IDs of bricks that this brick supports
    brick_support: HashMap<usize, HashSet<usize>>,
    /// Key: brick ID, Value: IDs of bricks that support this brick
    supported_by: HashMap<usize, HashSet<usize>>,
}

impl World {
    fn new(bricks: Vec<Brick>) -> Self {
        Self {
            bricks: bricks.into_iter().enumerate().collect(),
            brick_support: HashMap::new(),
            supported_by: HashMap::new(),
        }
    }

    fn sort(&mut self) {
        self.bricks.sort_by_key(|(_, brick)| brick.lowest_z());
    }

    fn apply_gravity(&mut self) {
        #[derive(Debug, Clone, Copy)]
        struct BrickCube {
            id: usize,
            position: IntVector3,
        }
        let mut highest_grounded_point = HashMap::<IntVector, BrickCube>::new();
        for (id, brick) in self.bricks.iter_mut() {
            let brick_lowest_z = brick.lowest_z();
            let brick_highest_z = brick.highest_z();
            let horizontal_coords = (brick.0.x..=brick.1.x)
                .flat_map(|x| (brick.0.y..=brick.1.y).map(move |y| IntVector::new(x, y)))
                .collect_vec();

            let (stop_z, bricks_underneath) = {
                let bricks_underneath = horizontal_coords
                    .iter()
                    .filter_map(|vec2d| highest_grounded_point.get(vec2d))
                    .copied()
                    .collect_vec();
                let stop_z = bricks_underneath
                    .iter()
                    .map(|cube| cube.position.z)
                    .max()
                    .unwrap_or(0)
                    + 1;
                let bricks_underneath = bricks_underneath
                    .into_iter()
                    .filter(|cube| cube.position.z == stop_z - 1)
                    .collect_vec();
                (stop_z, bricks_underneath)
            };

            let z_diff = brick_lowest_z - stop_z;
            brick.0.z -= z_diff;
            brick.1.z -= z_diff;
            for coord in horizontal_coords {
                let cube_position = IntVector3 {
                    x: coord.x,
                    y: coord.y,
                    z: brick_highest_z - z_diff,
                };
                highest_grounded_point.insert(
                    coord,
                    BrickCube {
                        id: *id,
                        position: cube_position,
                    },
                );
            }

            for brick_underneath in bricks_underneath {
                self.brick_support
                    .entry(brick_underneath.id)
                    .or_default()
                    .insert(*id);
                self.supported_by
                    .entry(*id)
                    .or_default()
                    .insert(brick_underneath.id);
            }
        }

        self.sort();
    }

    fn get_top_z(&self) -> Option<isize> {
        self.bricks.iter().map(|(_, brick)| brick.highest_z()).max()
    }

    fn is_load_bearing(&self, id: usize) -> Option<bool> {
        let supported_bricks = match { self.brick_support.get(&id) } {
            Some(supported_bricks) => supported_bricks,
            None => return Some(false),
        };
        if supported_bricks.is_empty() {
            return Some(false);
        }
        let is_only_support = supported_bricks.iter().any(|supported_brick_id| {
            let supported_by = self.supported_by.get(supported_brick_id).unwrap();
            supported_by.len() == 1
        });
        Some(is_only_support)
    }

    fn find_non_load_bearing_bricks(&self) -> usize {
        self.bricks
            .iter()
            .filter(|(id, _)| {
                !self
                    .is_load_bearing(*id)
                    .expect("is_load_bearing should always return Some")
            })
            .count()
    }

    fn debug_xz_plane(&self) -> String {
        let visible_ids = ('A'..'Z').chain('a'..'z').chain('0'..'9').collect_vec();
        if self.bricks.is_empty() {
            return "[no bricks]".into();
        }
        let max_x = self
            .bricks
            .iter()
            .map(|(_, brick)| brick.1.x.max(brick.0.x))
            .max()
            .unwrap_or(0);
        let shape = GridShape {
            width: max_x as usize + 1,
            height: self.get_top_z().unwrap() as usize + 1,
        };
        shape.format_char_grid(shape.coord_iter().map(|coord| {
            let x = coord.x;
            // we want to render with 0 at the bottom
            let z = shape.height as isize - coord.y - 1;
            let brick_ids = self
                .bricks
                .iter()
                .filter(|(_, brick)| brick.x_range().contains(&x) && brick.z_range().contains(&z))
                .map(|(id, _)| id)
                .collect_vec();

            if brick_ids.len() > 1 {
                '?' // indicates multiple bricks
            } else if brick_ids.len() == 1 {
                let id_idx = *brick_ids[0];
                if id_idx >= visible_ids.len() {
                    '#' // indicates a brick with an index too large to render in a single char
                } else {
                    visible_ids[id_idx]
                }
            } else {
                '.'
            }
        }))
    }

    fn debug_yz_plane(&self) -> String {
        if self.bricks.is_empty() {
            return "[no bricks]".into();
        }
        let max_y = self
            .bricks
            .iter()
            .map(|(_, brick)| brick.1.y.max(brick.0.y))
            .max()
            .unwrap_or(0);
        let shape = GridShape {
            width: max_y as usize + 1,
            height: self.get_top_z().unwrap() as usize + 1,
        };
        shape.format_char_grid(shape.coord_iter().map(|coord| {
            // "x" on the 2D plane is the "y" coordinate in the 3D world
            let y = coord.x;
            // we want to render with 0 at the bottom
            let z = shape.height as isize - coord.y - 1;
            let brick_ids = self
                .bricks
                .iter()
                .filter(|(_, brick)| brick.y_range().contains(&y) && brick.z_range().contains(&z))
                .map(|(id, _)| id)
                .collect_vec();

            if brick_ids.len() > 1 {
                '?' // indicates multiple bricks
            } else if brick_ids.len() == 1 {
                friendlify_id(*brick_ids[0])
            } else {
                '.'
            }
        }))
    }

    fn debug(&self) -> String {
        let xz_plane = self.debug_xz_plane();
        let yz_plane = self.debug_yz_plane();
        let mut lines = xz_plane.lines().zip(yz_plane.lines());
        let mut result = "".to_string();
        while let Some((xz_line, yz_line)) = lines.next() {
            writeln!(result, "{} | {}", xz_line, yz_line).unwrap();
        }
        result
    }
}

fn friendlify_id(id: usize) -> char {
    let visible_ids = ('A'..'Z').chain('a'..'z').chain('0'..'9').collect_vec();
    visible_ids.get(id).copied().unwrap_or('#')
}

impl FromStr for World {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut world = World::new(
            s.lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<_>>>()?,
        );
        world.sort();
        Ok(world)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day22.part1().unwrap().unwrap(), "411".to_string(),);
    }

    fn sample_input() -> World {
        indoc! {"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        "}
        .parse()
        .unwrap()
    }

    #[test]
    fn test_drop() {
        let mut world = sample_input();
        assert_eq!(world.get_top_z(), Some(9));
        world.apply_gravity();
        assert_eq!(world.get_top_z(), Some(6));
    }

    #[test]
    fn test_support() {
        let mut world = sample_input();
        world.apply_gravity();
        assert_eq!(world.is_load_bearing(0), Some(true)); // A
        assert_eq!(world.is_load_bearing(1), Some(false)); // B
        assert_eq!(world.is_load_bearing(2), Some(false)); // C
        assert_eq!(world.is_load_bearing(3), Some(false)); // D
        assert_eq!(world.is_load_bearing(4), Some(false)); // E
        assert_eq!(world.is_load_bearing(5), Some(true)); // F
        assert_eq!(world.is_load_bearing(6), Some(false)); // G
        assert_eq!(world.find_non_load_bearing_bricks(), 5);
    }

    #[test]
    fn test_no_overlap() {
        let mut world = puzzle_input().unwrap();
        world.apply_gravity();
        let mut found = HashMap::<IntVector3, Vec<usize>>::new();
        for (id, brick) in world.bricks.iter() {
            let cubes = (brick.0.z..=brick.1.z)
                .flat_map(|z| {
                    (brick.0.x..=brick.1.x).flat_map(move |x| {
                        (brick.0.y..=brick.1.y).map(move |y| IntVector3 { x, y, z })
                    })
                })
                .collect_vec();
            for coord in cubes {
                found
                    .entry(IntVector3 {
                        x: coord.x,
                        y: coord.y,
                        z: coord.z,
                    })
                    .or_default()
                    .push(*id);
            }
        }
        let mut errors = Vec::new();
        for (coord, ids) in found {
            if ids.len() > 1 {
                errors.push(format!("multiple IDs at coord {:?}: {:?}", coord, ids));
            }
        }
        errors.sort();
        assert_eq!(errors.len(), 0, "errors: {:#?}", errors);
    }

    #[test]
    fn test_no_floating() {
        let mut world = puzzle_input().unwrap();
        world.apply_gravity();
        let mut all_cubes = HashMap::<IntVector3, usize>::new();
        for (id, brick) in world.bricks.iter() {
            let cubes = (brick.0.z..=brick.1.z)
                .flat_map(|z| {
                    (brick.0.x..=brick.1.x).flat_map(move |x| {
                        (brick.0.y..=brick.1.y).map(move |y| IntVector3 { x, y, z })
                    })
                })
                .collect_vec();
            for coord in cubes {
                all_cubes.insert(coord, *id);
            }
        }
        let mut floating_bricks: Vec<(usize, Brick)> = Vec::new();
        for (id, brick) in world.bricks.iter() {
            let cubes = (brick.0.z..=brick.1.z)
                .flat_map(|z| {
                    (brick.0.x..=brick.1.x).flat_map(move |x| {
                        (brick.0.y..=brick.1.y).map(move |y| IntVector3 { x, y, z })
                    })
                })
                .collect_vec();
            let brick_is_floating = cubes.iter().all(|coord| {
                let below = IntVector3 {
                    x: coord.x,
                    y: coord.y,
                    z: coord.z - 1,
                };
                below.z > 0 && !all_cubes.contains_key(&below)
            });
            if brick_is_floating {
                floating_bricks.push((*id, *brick));
            }
        }
        assert!(
            floating_bricks.is_empty(),
            "floating bricks: {:?}",
            floating_bricks
        );
    }
}
