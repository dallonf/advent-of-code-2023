// Day 18: Lavaduct Lagoon

use crate::framework::Day;
use crate::prelude::*;

pub struct Day18;

impl Day for Day18 {
    fn day_number(&self) -> u8 {
        18
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
            super::Day18.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }
}
