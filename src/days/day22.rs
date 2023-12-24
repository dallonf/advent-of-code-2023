// Day 22: Sand Slabs

use std::collections::HashMap;
use std::fmt::Write;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::framework::grid::{GridShape, IntVector};
use crate::framework::Day;
use crate::prelude::*;

pub struct Day22;

impl Day for Day22 {
    fn day_number(&self) -> u8 {
        22
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || "Hello, world!".to_string().pipe(Ok)))
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
    bricks: Vec<Brick>,
}

impl World {
    fn sort(&mut self) {
        self.bricks.sort_by_key(|brick| brick.lowest_z());
    }

    fn apply_gravity(&mut self) {
        let mut highest_grounded_point = HashMap::<IntVector, isize>::new();
        for brick in self.bricks.iter_mut() {
            let brick_lowest_z = brick.lowest_z();
            let horizontal_coords = (brick.0.x..=brick.1.x)
                .flat_map(|x| (brick.0.y..=brick.1.y).map(move |y| IntVector::new(x, y)))
                .collect_vec();
            let stop_z = horizontal_coords
                .iter()
                .map(|vec2d| highest_grounded_point.get(vec2d).copied().unwrap_or(0) + 1)
                .max()
                .expect("Brick must exist on the XY plane");
            let z_diff = brick_lowest_z - stop_z;
            brick.0.z -= z_diff;
            brick.1.z -= z_diff;
            for coord in horizontal_coords {
                highest_grounded_point.insert(coord, stop_z);
            }
        }

        self.sort();
    }

    fn get_top_z(&self) -> Option<isize> {
        self.bricks.iter().map(|brick| brick.highest_z()).max()
    }

    fn debug_xz_plane(&self) -> String {
        if self.bricks.is_empty() {
            return "[no bricks]".into();
        }
        let max_x = self
            .bricks
            .iter()
            .map(|brick| brick.1.x.max(brick.0.x))
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
            let brick_indices = self
                .bricks
                .iter()
                .enumerate()
                .filter(|(_, brick)| brick.x_range().contains(&x) && brick.z_range().contains(&z))
                .map(|(i, _)| i)
                .collect_vec();

            if brick_indices.len() > 1 {
                '?' // indicates multiple bricks
            } else if brick_indices.len() == 1 {
                if brick_indices[0] > 9 {
                    '#' // indicates a brick with an index too large to render in a single char
                } else {
                    brick_indices[0].to_string().chars().next().unwrap()
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
            .map(|brick| brick.1.y.max(brick.0.y))
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
            let brick_indices = self
                .bricks
                .iter()
                .enumerate()
                .filter(|(_, brick)| brick.y_range().contains(&y) && brick.z_range().contains(&z))
                .map(|(i, _)| i)
                .collect_vec();
            if brick_indices.len() > 1 {
                '?' // indicates multiple bricks
            } else if brick_indices.len() == 1 {
                if brick_indices[0] > 9 {
                    '#' // indicates a brick with an index too large to render in a single char
                } else {
                    brick_indices[0].to_string().chars().next().unwrap()
                }
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

impl FromStr for World {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut world = World {
            bricks: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<_>>>()?,
        };
        world.sort();
        Ok(world)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            super::Day22.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
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
}
