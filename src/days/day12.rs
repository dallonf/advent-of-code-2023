// Day 12: Hot Springs

use std::str::FromStr;

use regex::Regex;

use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<Vec<Record>> {
    let input = include_str!("./day12_input.txt");
    input
        .lines()
        .map(|line| Record::from_str(line))
        .collect::<Result<Vec<_>>>()
}

pub struct Day12;

impl Day for Day12 {
    fn day_number(&self) -> u8 {
        12
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?
                .into_par_iter()
                .map(|record| record.possible_arrangements())
                .sum::<usize>()
                .to_string()
                .pipe(Ok)
        }))
    }

    #[cfg(feature = "slow_solutions")]
    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?
                .into_par_iter()
                .map(|record| record.unfold().possible_arrangements())
                .sum::<usize>()
                .to_string()
                .pipe(Ok)
        }))
    }

    #[cfg(not(feature = "slow_solutions"))]
    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Record {
    damage_sequence: Box<[Option<bool>]>,
    continguous_damage_sequences: Box<[u32]>,
}

// fn damage_record_to_char(input: Option<bool>) -> char {
//     match input {
//         Some(true) => '#',
//         Some(false) => '.',
//         None => '?',
//     }
// }

impl Record {
    fn possible_arrangements(&self) -> usize {
        fn resolve_next(
            current_is_damaged: bool,
            current_damage_sequence_length: usize,
            remaining_damage_sequence: &[Option<bool>],
            remaining_contiguous_damage_sequences: &[u32],
        ) -> usize {
            // println!(
            //     "{}{}->{} {}",
            //     (0..current_damage_sequence_length)
            //         .map(|_| damage_record_to_char(Some(true)))
            //         .join(""),
            //     damage_record_to_char(Some(current_is_damaged)),
            //     remaining_damage_sequence
            //         .iter()
            //         .copied()
            //         .map(damage_record_to_char)
            //         .join(""),
            //     remaining_contiguous_damage_sequences.iter().join(",")
            // );
            let next_contiguous_damage_sequence = remaining_contiguous_damage_sequences.get(0);
            if current_is_damaged {
                if let Some(next_contiguous_damage_sequence) = next_contiguous_damage_sequence {
                    let current_damage_sequence_length = current_damage_sequence_length + 1;
                    if current_damage_sequence_length > *next_contiguous_damage_sequence as usize {
                        // this sequence is too long. invalid arrangement.
                        return 0;
                    } else {
                        return remaining_valid_arrangements(
                            current_damage_sequence_length,
                            remaining_damage_sequence,
                            remaining_contiguous_damage_sequences,
                        );
                    }
                } else {
                    // there's damage, but there can't be any more damage sequences. invalid arrangement.
                    return 0;
                }
            } else {
                if current_damage_sequence_length == 0 {
                    // just a contiguous undamaged sequence, keep moving
                    return remaining_valid_arrangements(
                        current_damage_sequence_length,
                        remaining_damage_sequence,
                        remaining_contiguous_damage_sequences,
                    );
                }
                let next_contiguous_damage_sequence = next_contiguous_damage_sequence.expect(
                    "pretty sure there has to be an expected next damage sequence at this point",
                );
                if current_damage_sequence_length == *next_contiguous_damage_sequence as usize {
                    // pop the next contiguous damage sequence
                    return remaining_valid_arrangements(
                        0,
                        remaining_damage_sequence,
                        &remaining_contiguous_damage_sequences[1..],
                    );
                } else {
                    // wrong sequence length. invalid arrangement
                    return 0;
                }
            }
        }

        fn remaining_valid_arrangements(
            current_damage_sequence_length: usize,
            remaining_damage_sequence: &[Option<bool>],
            remaining_contiguous_damage_sequences: &[u32],
        ) -> usize {
            if remaining_damage_sequence.is_empty() {
                if remaining_contiguous_damage_sequences.is_empty() {
                    return 1;
                } else {
                    // special case: the last sequence is resolved
                    if remaining_contiguous_damage_sequences.len() == 1
                        && remaining_contiguous_damage_sequences[0] as usize
                            == current_damage_sequence_length
                    {
                        return 1;
                    }
                    return 0;
                }
            }

            let current = remaining_damage_sequence[0];
            let remaining_damage_sequence = &remaining_damage_sequence[1..];

            if let Some(first) = current {
                // known
                return resolve_next(
                    first,
                    current_damage_sequence_length,
                    remaining_damage_sequence,
                    remaining_contiguous_damage_sequences,
                );
            } else {
                // unknown; try both
                return resolve_next(
                    true,
                    current_damage_sequence_length,
                    remaining_damage_sequence,
                    remaining_contiguous_damage_sequences,
                ) + resolve_next(
                    false,
                    current_damage_sequence_length,
                    remaining_damage_sequence,
                    remaining_contiguous_damage_sequences,
                );
            }
        }

        remaining_valid_arrangements(0, &self.damage_sequence, &self.continguous_damage_sequences)
    }

    fn unfold(&self) -> Self {
        let mut damage_sequence = Vec::with_capacity(self.damage_sequence.len() * 5 + 4);
        for i in 0..5 {
            if i > 0 {
                damage_sequence.push(None);
            }
            damage_sequence.extend_from_slice(&self.damage_sequence);
        }
        Self {
            damage_sequence: damage_sequence.into_boxed_slice(),
            continguous_damage_sequences: self
                .continguous_damage_sequences
                .repeat(5)
                .into_boxed_slice(),
        }
    }
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = Regex::new("^([.#?]+) ([0-9,]+)$")?;
        let captures = regex
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid record: {}", s))?;
        let damage_sequence = captures
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                '?' => None,
                _ => unreachable!(),
            })
            .collect();

        let contingous_damage_sequences = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(|s| s.parse::<u32>().map_err(|err| anyhow!(err)))
            .collect::<Result<Box<[_]>>>()?;

        Ok(Self {
            damage_sequence,
            continguous_damage_sequences: contingous_damage_sequences,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(super::Day12.part1().unwrap().unwrap(), "7771".to_string(),);
    }

    #[cfg(feature = "slow_solutions")]
    #[test]
    fn test_part2() {
        assert_eq!(super::Day12.part2().unwrap().unwrap(), "0".to_string(),);
    }


    #[test]
    fn test_parsing() {
        let result = Record::from_str("???.### 1,1,3").unwrap();
        assert_eq!(
            result,
            Record {
                damage_sequence: vec![
                    None,
                    None,
                    None,
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true)
                ]
                .into_boxed_slice(),
                continguous_damage_sequences: vec![1, 1, 3].into_boxed_slice(),
            }
        );
    }

    #[test]
    fn test_possible_arrangements() {
        assert_eq!(
            Record::from_str("???.### 1,1,3")
                .unwrap()
                .possible_arrangements(),
            1
        );
        assert_eq!(
            Record::from_str(".??..??...?##. 1,1,3")
                .unwrap()
                .possible_arrangements(),
            4
        );
        assert_eq!(
            Record::from_str("?#?#?#?#?#?#?#? 1,3,1,6")
                .unwrap()
                .possible_arrangements(),
            1
        );
        assert_eq!(
            Record::from_str("????.#...#... 4,1,1")
                .unwrap()
                .possible_arrangements(),
            1
        );
        assert_eq!(
            Record::from_str("????.######..#####. 1,6,5")
                .unwrap()
                .possible_arrangements(),
            4
        );
        assert_eq!(
            Record::from_str("?###???????? 3,2,1")
                .unwrap()
                .possible_arrangements(),
            10
        );
    }

    #[test]
    fn test_unfold() {
        assert_eq!(
            Record::from_str(".# 1").unwrap().unfold(),
            Record::from_str(".#?.#?.#?.#?.# 1,1,1,1,1").unwrap()
        );
        assert_eq!(
            Record::from_str("???.### 1,1,3").unwrap().unfold(),
            Record::from_str(
                "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_unfolded_arrangements() {
        assert_eq!(
            Record::from_str("???.### 1,1,3")
                .unwrap()
                .unfold()
                .possible_arrangements(),
            1
        );
        assert_eq!(
            Record::from_str(".??..??...?##. 1,1,3")
                .unwrap()
                .unfold()
                .possible_arrangements(),
            16384
        );
        assert_eq!(
            Record::from_str("?#?#?#?#?#?#?#? 1,3,1,6")
                .unwrap()
                .unfold()
                .possible_arrangements(),
            1
        );
        assert_eq!(
            Record::from_str("????.#...#... 4,1,1")
                .unwrap()
                .unfold()
                .possible_arrangements(),
            16
        );
        assert_eq!(
            Record::from_str("????.######..#####. 1,6,5")
                .unwrap()
                .unfold()
                .possible_arrangements(),
            2500
        );
        assert_eq!(
            Record::from_str("?###???????? 3,2,1")
                .unwrap()
                .unfold()
                .possible_arrangements(),
            506250
        );
    }
}
