// Day 20: Pulse Propagation

use crate::framework::Day;
use crate::prelude::*;

pub struct Day20;

impl Day for Day20 {
    fn day_number(&self) -> u8 {
        20
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
            super::Day20.part1().unwrap().unwrap(),
            "Hello, world!".to_string(),
        );
    }
}
