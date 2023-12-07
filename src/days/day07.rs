// Day 7: Camel Cards

use crate::framework::Day;
use crate::prelude::*;

pub struct Day7;

impl Day for Day7 {
    fn day_number(&self) -> u8 {
        7
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
            super::Day7.part1().unwrap().unwrap()
        );
    }
}
