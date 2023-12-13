// Day 13: Point of Incidence

use crate::framework::Day;
use crate::prelude::*;

pub struct Day13;

impl Day for Day13 {
    fn day_number(&self) -> u8 {
        13
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
            super::Day13.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }
}
