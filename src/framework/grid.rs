use crate::framework::prelude::*;
use std::ops::{Add, AddAssign, Mul};

/// Value can be used as a u8 bitmask
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North = 0b0001,
    South = 0b0010,
    East = 0b0100,
    West = 0b1000,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn as_vector(&self) -> IntVector {
        match self {
            Direction::North => NORTH,
            Direction::South => SOUTH,
            Direction::East => EAST,
            Direction::West => WEST,
        }
    }
}

impl From<Direction> for IntVector {
    fn from(direction: Direction) -> Self {
        direction.as_vector()
    }
}

impl TryFrom<IntVector> for Direction {
    type Error = anyhow::Error;

    fn try_from(vector: IntVector) -> Result<Self> {
        match vector {
            NORTH => Ok(Direction::North),
            SOUTH => Ok(Direction::South),
            EAST => Ok(Direction::East),
            WEST => Ok(Direction::West),
            _ => Err(anyhow!("invalid direction vector: {:?}", vector)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct IntVector {
    pub x: isize,
    pub y: isize,
}

pub const NORTH: IntVector = IntVector::new(0, -1);
pub const SOUTH: IntVector = IntVector::new(0, 1);
pub const EAST: IntVector = IntVector::new(1, 0);
pub const WEST: IntVector = IntVector::new(-1, 0);

impl IntVector {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn cardinal_neighbors(self) -> Vec<Self> {
        vec![
            IntVector::new(self.x - 1, self.y),
            IntVector::new(self.x + 1, self.y),
            IntVector::new(self.x, self.y - 1),
            IntVector::new(self.x, self.y + 1),
        ]
    }

    pub fn cardinal_neighbors_with_directions(self) -> Vec<(Self, Direction)> {
        vec![
            (self + WEST, Direction::West),
            (self + EAST, Direction::East),
            (self + NORTH, Direction::North),
            (self + SOUTH, Direction::South),
        ]
    }

    pub fn inverse(&self) -> IntVector {
        IntVector::new(-self.x, -self.y)
    }

    pub fn manhattan_distance(&self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
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

impl Mul<isize> for IntVector {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GridShape {
    pub width: usize,
    pub height: usize,
}

impl GridShape {
    /// Panics if coordinate is negative.
    pub fn arr_index(&self, coord: IntVector) -> usize {
        (coord.y as usize) * self.width + (coord.x as usize)
    }

    pub fn coordinate_for_index(&self, index: usize) -> IntVector {
        IntVector::new((index % self.width) as isize, (index / self.width) as isize)
    }

    pub fn parse_char_grid(input: &str) -> Result<(GridShape, Box<[char]>)> {
        let lines = input.lines().collect::<Vec<_>>();
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
        let height = chars.len() / width;
        Ok((GridShape { width, height }, chars.into_boxed_slice()))
    }

    pub fn format_char_grid(&self, chars: impl IntoIterator<Item = char>) -> String {
        let mut result = String::new();
        let mut chars = chars.into_iter().peekable();
        while chars.peek().is_some() {
            for _ in 0..self.width {
                if let Some(next_char) = chars.next() {
                    result.push(next_char);
                } else {
                    break;
                }
            }
            result.push('\n');
        }
        result
    }

    /// Returns an iterator over all coordinates in the grid, left to right, top to bottom.
    pub fn coord_iter(&self) -> impl Iterator<Item = IntVector> + '_ {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| IntVector::new(x as isize, y as isize)))
    }

    pub fn in_bounds(&self, coord: IntVector) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && coord.x < (self.width as isize)
            && coord.y < (self.height as isize)
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SignedGridShape {
    pub top_left: IntVector,
    pub bottom_right: IntVector,
}

#[allow(dead_code)]
impl SignedGridShape {
    pub fn new(top_left: IntVector, bottom_right: IntVector) -> Self {
        SignedGridShape {
            top_left,
            bottom_right,
        }
    }

    pub fn is_in_bounds(&self, coord: IntVector) -> bool {
        coord.x >= self.top_left.x
            && coord.y >= self.top_left.y
            && coord.x <= self.bottom_right.x
            && coord.y <= self.bottom_right.y
    }

    pub fn coord_iter(&self) -> impl Iterator<Item = IntVector> + '_ {
        (self.top_left.y..=self.bottom_right.y).flat_map(move |y| {
            (self.top_left.x..=self.bottom_right.x).map(move |x| IntVector::new(x, y))
        })
    }

    pub fn format_char_grid(&self, chars: impl IntoIterator<Item = char>) -> String {
        let mut result = String::new();
        let mut chars = chars.into_iter().peekable();
        while chars.peek().is_some() {
            for _ in self.top_left.x..=self.bottom_right.x {
                if let Some(next_char) = chars.next() {
                    result.push(next_char);
                } else {
                    break;
                }
            }
            result.push('\n');
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(
            IntVector::new(1, 6).manhattan_distance(IntVector::new(5, 11)),
            9
        );
        assert_eq!(
            IntVector::new(4, 0).manhattan_distance(IntVector::new(9, 10)),
            15
        );
    }
}
