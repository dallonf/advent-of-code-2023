// Day 1: Trebuchet?!

use crate::framework::Day;
use crate::prelude::*;

pub struct Day1;

lazy_static! {
    static ref PUZZLE_INPUT: String = include_str!("./day01_input.txt").trim().to_string();
}

impl Day for Day1 {
    fn day_number(&self) -> u8 {
        1
    }

    fn part1(&self) -> Option<Result<String>> {
        let result = sum_of_calibration_values(&PUZZLE_INPUT);
        Some(result.map(|it| it.to_string()))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

fn get_calibration_value(line: &str) -> Result<u32> {
    let chars = line.chars();
    let digits = chars.filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    let first = digits.first().ok_or(anyhow!("No digits found"))?;
    let last = digits.last().ok_or(anyhow!("No digits found"))?;
    Ok(first * 10 + last)
}

fn sum_of_calibration_values(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| get_calibration_value(line))
        .sum::<Result<u32>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("54634".to_string(), super::Day1.part1().unwrap().unwrap());
    }

    #[test]
    fn test_get_calibration_value() {
        assert_eq!(get_calibration_value("1abc2").unwrap(), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx").unwrap(), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f").unwrap(), 15);
        assert_eq!(get_calibration_value("treb7uchet").unwrap(), 77);
    }

    #[test]
    fn test_sum_of_calibration_values() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        assert_eq!(sum_of_calibration_values(input).unwrap(), 142);
    }
}
