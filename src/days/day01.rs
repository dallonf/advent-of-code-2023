// Day 1: Trebuchet?!

use regex::Regex;

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
        let result = sum_of_calibration_values_mk2(&PUZZLE_INPUT);
        Some(result.map(|it| it.to_string()))
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

fn get_calibration_value_mk2(line: &str) -> Result<u32> {
    let digit_regex_template = r"([0-9]|one|two|three|four|five|six|seven|eight|nine)";
    let first_digit_regex = Regex::new(digit_regex_template).unwrap();
    let last_digit_regex = Regex::new(&format!(".*+{}", digit_regex_template)).unwrap();
    let first_digit_capture = first_digit_regex
        .captures(line)
        .ok_or(anyhow!("No digits found"))?
        .get(1)
        .unwrap()
        .as_str();
    let last_digit_capture = last_digit_regex
        .captures(line)
        .ok_or(anyhow!("No digits found"))?
        .get(1)
        .unwrap()
        .as_str();
    println!("digits for {line}: {first_digit_capture} and {last_digit_capture}");
    let first = digit(first_digit_capture)?;
    let last = digit(last_digit_capture)?;
    Ok(first * 10 + last)
}

fn digit(input: &str) -> Result<u32> {
    match input {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        numeric => numeric.parse::<u32>().map_err(Into::into),
    }
}

fn sum_of_calibration_values_mk2(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| get_calibration_value_mk2(line))
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
    fn test_part2() {
        assert_eq!("53855".to_string(), super::Day1.part2().unwrap().unwrap());
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

    #[test]
    fn test_get_calibration_value_mk2() {
        assert_eq!(get_calibration_value_mk2("two1nine").unwrap(), 29);
        assert_eq!(get_calibration_value_mk2("eightwothree").unwrap(), 83);
        assert_eq!(get_calibration_value_mk2("abcone2threexyz").unwrap(), 13);
        assert_eq!(get_calibration_value_mk2("xtwone3four").unwrap(), 24);
        assert_eq!(get_calibration_value_mk2("4nineeightseven2").unwrap(), 42);
        assert_eq!(get_calibration_value_mk2("zoneight234").unwrap(), 14);
        assert_eq!(get_calibration_value_mk2("7pqrstsixteen").unwrap(), 76);
    }

    #[test]
    fn test_sum_of_calibration_values_mk2() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        assert_eq!(sum_of_calibration_values_mk2(input).unwrap(), 281);
    }
}
