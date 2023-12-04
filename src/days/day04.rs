// Day 4: Scratchcards

use std::cell::Cell;
use std::collections::HashSet;
use std::str::FromStr;

use regex::Regex;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day4;

fn puzzle_input() -> Result<Vec<Card>> {
    include_str!("./day04_input.txt")
        .lines()
        .map(|line| Card::from_str(line))
        .collect()
}

impl Day for Day4 {
    fn day_number(&self) -> u8 {
        4
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let cards = puzzle_input()?;
            cards
                .iter()
                .map(|card| card.score())
                .sum::<u32>()
                .to_string()
                .pipe(Ok)
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let cards = puzzle_input()?;
            cards
                .iter()
                .map(|card| card.explode(&cards))
                .sum::<u32>()
                .to_string()
                .pipe(Ok)
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: u32,
    numbers: Box<[u8]>,
    winning_numbers: Box<[u8]>,
    explosion_result: Cell<Option<u32>>,
}

impl Card {
    fn matching_winning_numbers(&self) -> HashSet<u8> {
        let numbers_set = self.numbers.iter().copied().collect::<HashSet<u8>>();
        let winning_numbers_set = self
            .winning_numbers
            .iter()
            .copied()
            .collect::<HashSet<u8>>();
        numbers_set
            .intersection(&winning_numbers_set)
            .copied()
            .collect()
    }

    fn score(&self) -> u32 {
        let matching_winning_numbers = self.matching_winning_numbers().len() as u32;
        match matching_winning_numbers {
            0 => 0,
            1 => 1,
            more => 2_u32.pow(more - 1),
        }
    }

    fn get_cards_to_copy<'a>(&self, full_list: &'a [Card]) -> Vec<&'a Card> {
        let matching_numbers = self.matching_winning_numbers().len();
        if matching_numbers == 0 {
            return vec![];
        }
        let next_card = self.id as usize;
        let last_copied_card = next_card + matching_numbers - 1;
        let copied_cards = &full_list[next_card..=last_copied_card];
        copied_cards.iter().collect()
    }

    fn explode(&self, full_list: &[Card]) -> u32 {
        if let Some(result) = self.explosion_result.get() {
            return result;
        }
        let copied_cards = self.get_cards_to_copy(full_list);

        let result = 1 + copied_cards
            .iter()
            .map(|card| card.explode(full_list))
            .sum::<u32>();
        self.explosion_result.set(Some(result));
        result
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let framework = Regex::new(r"^Card +(\d+): ([0-9 ]+) \| ([0-9 ]+)$")
            .unwrap()
            .captures(s)
            .ok_or(anyhow!("Invalid card: {s}"))?;

        let id = framework.get(1).unwrap().as_str().parse()?;
        let numbers: Box<[u8]> = framework
            .get(2)
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>().map_err(|err| anyhow!(err)))
            .collect::<Result<Vec<u8>>>()?
            .into_boxed_slice();
        let winning_numbers: Box<[u8]> = framework
            .get(3)
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>().map_err(|err| anyhow!(err)))
            .collect::<Result<Vec<u8>>>()?
            .into_boxed_slice();

        Ok(Card {
            id,
            numbers,
            winning_numbers,
            explosion_result: Cell::new(None),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("26914".to_string(), super::Day4.part1().unwrap().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            "13080971".to_string(),
            super::Day4.part2().unwrap().unwrap()
        );
    }

    fn sample_input() -> Vec<Card> {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        input
            .lines()
            .map(|line| Card::from_str(line).unwrap())
            .collect()
    }

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected_card = Card {
            id: 1,
            numbers: vec![41, 48, 83, 86, 17].into_boxed_slice(),
            winning_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53].into_boxed_slice(),
            explosion_result: Cell::new(None),
        };
        assert_eq!(expected_card, Card::from_str(input).unwrap());
    }

    #[test]
    fn test_matching_winning_numbers() {
        let card = Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        let expected: HashSet<u8> = vec![48, 83, 17, 86].into_iter().collect();
        assert_eq!(expected, card.matching_winning_numbers());
    }

    #[test]
    fn test_score() {
        let scores = sample_input()
            .iter()
            .map(|card| (card.id, card.score()))
            .collect::<Vec<(u32, u32)>>();

        assert_eq!(scores, vec![(1, 8), (2, 2), (3, 2), (4, 1), (5, 0), (6, 0)]);
    }

    #[test]
    fn test_cards_to_copy() {
        let cards = sample_input();
        let card = &cards[0];
        let cards_to_copy = card.get_cards_to_copy(&cards);
        let expected = vec![2, 3, 4, 5];
        assert_eq!(
            cards_to_copy.iter().map(|it| it.id).collect::<Vec<u32>>(),
            expected
        );
    }

    #[test]
    fn test_explode_cards() {
        let cards = sample_input();
        let result = cards.iter().map(|card| card.explode(&cards)).sum::<u32>();
        assert_eq!(result, 30);
    }
}
