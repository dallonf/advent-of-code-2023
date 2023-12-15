// Day 15: Lens Library

use crate::framework::Day;
use crate::prelude::*;

pub struct Day15;

fn puzzle_input() -> Vec<String> {
    include_str!("./day15_input.txt")
        .replace("\r", "")
        .replace("\n", "")
        .split(",")
        .map(|step| step.to_string())
        .collect()
}

impl Day for Day15 {
    fn day_number(&self) -> u8 {
        15
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()
                .iter()
                .map(|s| {
                    let hash = s.as_str().holiday_hash() as u64;
                    dbg!(s, hash);
                    hash
                })
                .sum::<u64>()
                .to_string()
                .pipe(Ok)
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

trait HolidayHash {
    fn holiday_hash(&self) -> u8;
}

impl<'a> HolidayHash for &'a str {
    fn holiday_hash(&self) -> u8 {
        self.chars().fold(0, |result, c| {
            let intermediate = (result as u32 + c as u32) * 17;
            (intermediate % 256) as u8
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day15.part1().unwrap().unwrap(), "506891".to_string(),);
    }

    #[test]
    fn test_holiday_hash() {
        assert_eq!("HASH".holiday_hash(), 52);
    }

    #[test]
    fn test_hash_sequence() {
        let sequence = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".split(",");
        let hash_sum = sequence.map(|s| s.holiday_hash() as u64).sum::<u64>();
        assert_eq!(hash_sum, 1320);
    }
}
