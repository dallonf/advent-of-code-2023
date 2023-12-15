// Day 15: Lens Library

use std::fmt::Display;
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
        Some(try_block(move || {
            let mut boxen = Boxen::default();
            for step in puzzle_input()? {
                boxen.follow_instruction(step)?;
            }
            boxen.total_focusing_power().to_string().pipe(Ok)
        }))
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

impl HolidayHash for String {
    fn holiday_hash(&self) -> u8 {
        self.as_str().holiday_hash()
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

impl Lens {
    fn focusing_power(&self, box_number: u8, index_in_box: usize) -> u64 {
        (box_number as u64 + 1) * (index_in_box as u64 + 1) * self.focal_length as u64
    }
}

impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct LensBox(Vec<Lens>);

impl LensBox {
    fn total_focusing_power(&self, box_number: u8) -> u64 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, lens)| lens.focusing_power(box_number, i))
            .sum()
    }
}

const BYTE_ARRAY_LENGTH: usize = u8::MAX as usize + 1;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Boxen([LensBox; BYTE_ARRAY_LENGTH]);

impl Boxen {
    fn follow_instruction(&mut self, step: InitializationStep) -> Result<()> {
        let box_num = step.label.holiday_hash();
        let current_box = &mut self.0[box_num as usize];
        match step.operation {
            Operation::Remove => {
                let existing_lens_index = current_box
                    .0
                    .iter()
                    .position(|lens| lens.label == step.label);
                if let Some(existing_lens_index) = existing_lens_index {
                    current_box.0.remove(existing_lens_index);
                }
            }
            Operation::Insert(focal_length) => {
                let existing_lens_index = current_box
                    .0
                    .iter()
                    .position(|lens| lens.label == step.label);
                if let Some(existing_lens_index) = existing_lens_index {
                    current_box.0[existing_lens_index].focal_length = focal_length;
                } else {
                    current_box.0.push(Lens {
                        focal_length,
                        label: step.label,
                    });
                }
            }
        }
        Ok(())
    }

    fn total_focusing_power(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, lens_box)| lens_box.total_focusing_power(i as u8))
            .sum()
    }
}

impl Default for Boxen {
    fn default() -> Self {
        // see https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut array: [MaybeUninit<LensBox>; BYTE_ARRAY_LENGTH] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..BYTE_ARRAY_LENGTH {
            array[i] = MaybeUninit::new(LensBox::default());
        }
        unsafe { std::mem::transmute(array) }
    }
}

impl Display for Boxen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let boxen_with_lenses = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, lens_box)| !lens_box.0.is_empty())
            .collect_vec();
        for (i, (box_num, lens_box)) in boxen_with_lenses.iter().enumerate() {
            write!(
                f,
                "Box {}: {}{}",
                box_num,
                lens_box.0.iter().map(|lens| lens.to_string()).join(" "),
                if i == boxen_with_lenses.len() - 1 {
                    ""
                } else {
                    "\n"
                }
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day15.part1().unwrap().unwrap(), "506891".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::Day15.part2().unwrap().unwrap(), "230462".to_string());
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

    #[test]
    fn test_initialization() {
        let mut boxen = Boxen::default();
        let mut instruction_iter = sample_input().into_iter();
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // rn=1
        assert_eq!(boxen.to_string(), "Box 0: [rn 1]");
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // cm-
        assert_eq!(boxen.to_string(), "Box 0: [rn 1]");
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // qp=3
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1]
              Box 1: [qp 3]
            "}
            .trim()
        );
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // cm=2
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1] [cm 2]
              Box 1: [qp 3]
            "}
            .trim()
        );
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // qp-
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1] [cm 2]
            "}
            .trim()
        );
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // pc=4
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1] [cm 2]
              Box 3: [pc 4]
            "}
            .trim()
        );
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // ot=9
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1] [cm 2]
              Box 3: [pc 4] [ot 9]
            "}
            .trim()
        );
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // ab=5
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1] [cm 2]
              Box 3: [pc 4] [ot 9] [ab 5]
            "}
            .trim()
        );
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // pc-
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1] [cm 2]
              Box 3: [ot 9] [ab 5]
            "}
            .trim()
        );
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // pc=6
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1] [cm 2]
              Box 3: [ot 9] [ab 5] [pc 6]
            "}
            .trim()
        );
        boxen
            .follow_instruction(instruction_iter.next().unwrap())
            .unwrap(); // ot=7
        assert_eq!(
            boxen.to_string(),
            indoc! {"
              Box 0: [rn 1] [cm 2]
              Box 3: [ot 7] [ab 5] [pc 6]
            "}
            .trim()
        );
    }

    #[test]
    fn test_focusing_power() {
        let mut boxen = Boxen::default();
        for step in sample_input() {
            boxen.follow_instruction(step).unwrap();
        }
        assert_eq!(boxen.total_focusing_power(), 145);
    }
}
