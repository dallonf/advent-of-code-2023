// Day 5: If You Give A Seed A Fertilizer

use crate::framework::Day;
use crate::prelude::*;

pub struct Day5;

impl Day for Day5 {
    fn day_number(&self) -> u8 {
        5
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
            "Hello, world!".to_string(),
            super::Day5.part1().unwrap().unwrap()
        );
    }
}
