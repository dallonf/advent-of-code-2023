// Day 17: Clumsy Crucible

use crate::framework::Day;
use crate::prelude::*;

pub struct Day17;

impl Day for Day17 {
    fn day_number(&self) -> u8 {
        17
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
            super::Day17.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }
}
