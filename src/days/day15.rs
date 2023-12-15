// Day 15: Lens Library

use std::mem::MaybeUninit;
use std::str::FromStr;

use regex::Regex;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day15;

fn puzzle_input() -> Result<Vec<InitializationStep>> {
    include_str!("./day15_input.txt")
        .replace("\r", "")
        .replace("\n", "")
        .split(",")
        .map(|step| step.parse())
        .collect()
}

impl Day for Day15 {
    fn day_number(&self) -> u8 {
        15
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?
                .iter()
                .map(|s| {
                    let hash = s.holiday_hash as u64;
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct InitializationStep {
    label: String,
    operation: Operation,
    holiday_hash: u8,
}

lazy_static! {
    static ref HOLIDAY_HASH_REGEX: Regex = Regex::new("^([a-z]+)(-|=[0-9])$").unwrap();
}

impl FromStr for InitializationStep {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = HOLIDAY_HASH_REGEX
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid initialization step: {}", s))?;
        let label = captures.get(1).unwrap().as_str().to_string();
        let operation = match captures.get(2).unwrap().as_str() {
            "-" => Operation::Remove,
            s => Operation::Insert(s[1..].parse::<u8>()?),
        };
        let holiday_hash = s.holiday_hash();
        Ok(Self {
            label,
            operation,
            holiday_hash,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Remove,
    Insert(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    focal_length: u8,
    label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct LensBox(Vec<Lens>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Boxen([LensBox; u8::MAX as usize]);

impl Default for Boxen {
    fn default() -> Self {
        // see https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut array: [MaybeUninit<LensBox>; u8::MAX as usize] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..u8::MAX {
            array[i as usize] = MaybeUninit::new(LensBox::default());
        }
        unsafe { std::mem::transmute(array) }
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

    fn sample_input() -> Vec<InitializationStep> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        input.split(",").map(|step| step.parse().unwrap()).collect()
    }

    #[test]
    fn test_hash_sequence() {
        let hash_sum = sample_input()
            .iter()
            .map(|s| s.holiday_hash as u64)
            .sum::<u64>();
        assert_eq!(hash_sum, 1320);
    }
}
