// Day 9: Mirage Maintenance

use crate::framework::Day;
use crate::prelude::*;

pub struct Day9;

fn puzzle_input() -> Result<Vec<Vec<i32>>> {
    include_str!("./day09_input.txt")
        .lines()
        .map(parse_sequence)
        .collect()
}

impl Day for Day9 {
    fn day_number(&self) -> u8 {
        9
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let input = puzzle_input()?;
            let results = input.par_iter().map(|it| extrapolate(it));
            let sum: i32 = results.sum();
            Ok(sum.to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let input = puzzle_input()?;
            let results = input.par_iter().map(|it| extrapolate_backwards(it));
            let sum: i32 = results.sum();
            Ok(sum.to_string())
        }))
    }
}

fn parse_sequence(input: &str) -> Result<Vec<i32>> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().map_err(|err| err.into()))
        .collect()
}

fn get_difference_sequence(input: &[i32]) -> Vec<i32> {
    input.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn extrapolate(input: &[i32]) -> i32 {
    if input.iter().all(|&it| it == 0) {
        return 0;
    }
    let next_diff = extrapolate(&get_difference_sequence(input));
    input.last().copied().unwrap_or(0) + next_diff
}

fn extrapolate_backwards(input: &[i32]) -> i32 {
    if input.iter().all(|&it| it == 0) {
        return 0;
    }
    let next_diff = extrapolate_backwards(&get_difference_sequence(input));
    input.first().copied().unwrap_or(0) - next_diff
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            super::Day9.part1().unwrap().unwrap(),
            "1637452029".to_string(),
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::Day9.part2().unwrap().unwrap(), "0".to_string());
    }

    fn sample_input() -> Vec<Vec<i32>> {
        indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "}
        .lines()
        .map(parse_sequence)
        .collect::<Result<Vec<Vec<i32>>>>()
        .unwrap()
    }

    #[test]
    fn test_parsing() {
        let result = parse_sequence("0 3 6 9 12 15").unwrap();
        assert_eq!(result, vec![0, 3, 6, 9, 12, 15])
    }

    #[test]
    fn test_get_difference_sequence() {
        let input = parse_sequence("0 3 6 9 12 15").unwrap();
        let expected = vec![3, 3, 3, 3, 3];
        let result = get_difference_sequence(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extrapolate() {
        let input = sample_input();
        let results = input.iter().map(|it| extrapolate(it)).collect_vec();
        let expected = vec![18, 28, 68];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_extrapolate_backwards() {
        let input = sample_input();
        let results = input
            .iter()
            .map(|it| extrapolate_backwards(it))
            .collect_vec();
        let expected = vec![-3, 0, 5];
        assert_eq!(results, expected);
    }
}
