// Day 2: Cube Conundrum

use crate::framework::Day;
use crate::prelude::*;

pub struct Day2;

impl Day for Day2 {
    fn day_number(&self) -> u8 {
        2
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(Ok("Hello, world!".to_string()))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            "Hello, world!".to_string(),
            super::Day2.part1().unwrap().unwrap()
        );
    }
}
