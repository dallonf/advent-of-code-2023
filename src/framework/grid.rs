use crate::framework::prelude::*;
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct IntVector {
    pub x: isize,
    pub y: isize,
}

impl IntVector {
    const fn new(x: isize, y: isize) -> Self {
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

    fn inverse(&self) -> IntVector {
        IntVector::new(-self.x, -self.y)
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GridShape {
    width: usize,
    height: usize,
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
}
