// Day 7: Camel Cards

use std::fmt::Display;
use std::str::FromStr;

use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<Game> {
    let input = include_str!("./day07_input.txt");
    Game::from_str(input)
}

pub struct Day7;

impl Day for Day7 {
    fn day_number(&self) -> u8 {
        7
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            Ok(puzzle_input()?.total_winnings().to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

type Number = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(input: char) -> Result<Self> {
        match input {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(anyhow!("Invalid card: {}", input)),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Jack => 'J',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let matching_cards = self.cards.iter().counts();
        let counts = matching_cards
            .values()
            .sorted()
            .rev()
            .copied()
            .collect_vec();
        if counts.get(0) == Some(&5) {
            HandType::FiveOfAKind
        } else if counts.get(0) == Some(&4) {
            HandType::FourOfAKind
        } else if counts.get(0) == Some(&3) {
            if counts.get(1) == Some(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if counts.get(0) == Some(&2) {
            if counts.get(1) == Some(&2) {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type != other_type {
            return self_type.cmp(&other_type);
        }
        let self_cards = self.cards.iter().copied().collect_vec();
        let other_cards = other.cards.iter().copied().collect_vec();
        for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
            if self_card != other_card {
                return self_card.cmp(&other_card);
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{}", card.to_char())?;
        }
        Ok(())
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let chars = s.chars();
        let cards_vec = chars.map(Card::from_char).collect::<Result<Vec<_>>>()?;
        let cards = cards_vec
            .try_into()
            .map_err(|_| anyhow!("Invalid hand: {}", s))?;
        Ok(Hand { cards })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HandWithBid {
    hand: Hand,
    bid: Number,
}

impl FromStr for HandWithBid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (hand_str, bid_str) = s.split_once(" ").ok_or(anyhow!("Invalid input: {s}"))?;
        let hand = Hand::from_str(hand_str)?;
        let bid = bid_str.parse::<Number>()?;
        Ok(HandWithBid { hand, bid })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    hands: Vec<HandWithBid>,
}

impl Game {
    fn total_winnings(&self) -> Number {
        let ranked_hands = self
            .hands
            .iter()
            .sorted_by_key(|hand| hand.hand)
            .enumerate();
        ranked_hands
            .map(|(rank, hand)| hand.bid * (rank as Number + 1))
            .sum()
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let hands = s
            .lines()
            .map(|line| line.parse::<HandWithBid>())
            .collect::<Result<Vec<_>>>()?;
        Ok(Game { hands })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            "249726565".to_string(),
            super::Day7.part1().unwrap().unwrap()
        );
    }

    fn sample_input() -> Game {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        input.parse::<Game>().unwrap()
    }

    #[test]
    fn test_parsing() {
        let game = sample_input();
        assert_eq!(5, game.hands.len());
        assert_eq!(
            HandWithBid {
                hand: Hand {
                    cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]
                },
                bid: 765
            },
            game.hands[0]
        );
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(
            Hand::from_str("AAAAA").unwrap().get_type(),
            HandType::FiveOfAKind
        );
        assert_eq!(
            Hand::from_str("AA8AA").unwrap().get_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand::from_str("23332").unwrap().get_type(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand::from_str("TTT98").unwrap().get_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand::from_str("23432").unwrap().get_type(),
            HandType::TwoPair
        );
        assert_eq!(
            Hand::from_str("A23A4").unwrap().get_type(),
            HandType::OnePair
        );
        assert_eq!(
            Hand::from_str("23456").unwrap().get_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn test_comparison() {
        assert!(Hand::from_str("AA8AA").unwrap() > Hand::from_str("23456").unwrap());
        assert!(Hand::from_str("33332").unwrap() > Hand::from_str("2AAAA").unwrap());
        assert!(Hand::from_str("77888").unwrap() > Hand::from_str("77788").unwrap());
    }

    #[test]
    fn test_total_winnings() {
        let game = sample_input();
        assert_eq!(game.total_winnings(), 6440);
    }
}
