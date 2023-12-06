// Day 6: Wait For It

use crate::framework::Day;
use crate::prelude::*;

pub struct Day6;

impl Day for Day6 {
    fn day_number(&self) -> u8 {
        6
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
            super::Day6.part1().unwrap().unwrap()
        );
    }
}
