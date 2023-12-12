// Day 12: Hot Springs

use crate::framework::Day;
use crate::prelude::*;

pub struct Day12;

impl Day for Day12 {
    fn day_number(&self) -> u8 {
        12
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || Ok("Hello, world!".to_string())))
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
            super::Day12.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }
}
