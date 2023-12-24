// Day 22: Sand Slabs

use crate::framework::Day;
use crate::prelude::*;

pub struct Day22;

impl Day for Day22 {
    fn day_number(&self) -> u8 {
        22
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
            super::Day22.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }
}
