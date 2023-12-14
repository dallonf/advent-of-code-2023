// Day 14: Parabolic Reflector Dish

use crate::framework::Day;
use crate::prelude::*;

pub struct Day14;

impl Day for Day14 {
    fn day_number(&self) -> u8 {
        14
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
            super::Day14.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }
}
