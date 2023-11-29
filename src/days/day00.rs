// Day 0 - Template

use crate::framework::Day;
use crate::prelude::*;

pub struct Day0;

impl Day for Day0 {
    fn day_number(&self) -> u8 {
        0
    }

    fn part1(&self) -> Option<String> {
        Some("Hello, world!".to_string())
    }

    fn part2(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Some("Hello, world!".to_string()), super::Day0.part1());
    }
}
