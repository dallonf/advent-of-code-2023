// Day 21: Step Counter

use crate::framework::Day;
use crate::prelude::*;

pub struct Day21;

impl Day for Day21 {
    fn day_number(&self) -> u8 {
        21
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || "Hello, world!".to_string().pipe(Ok)))
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
            super::Day21.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }
}
