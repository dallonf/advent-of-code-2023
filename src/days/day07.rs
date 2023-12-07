// Day 7: Camel Cards

use std::fmt::Display;
use std::str::FromStr;

use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input<HandType: Hand>() -> Result<Game<HandType>> {
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
            Ok(puzzle_input::<HandMk1>()?
                .total_winnings()
                .to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

type Number = u64;

trait Card: Ord + Copy {
    fn to_char(&self) -> char;
    fn from_char(input: char) -> Result<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CardMk1 {
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

impl Card for CardMk1 {
    fn to_char(&self) -> char {
        match self {
            CardMk1::Two => '2',
            CardMk1::Three => '3',
            CardMk1::Four => '4',
            CardMk1::Five => '5',
            CardMk1::Six => '6',
            CardMk1::Seven => '7',
            CardMk1::Eight => '8',
            CardMk1::Nine => '9',
            CardMk1::Ten => 'T',
            CardMk1::Jack => 'J',
            CardMk1::Queen => 'Q',
            CardMk1::King => 'K',
            CardMk1::Ace => 'A',
        }
    }

    fn from_char(input: char) -> Result<Self> {
        match input {
            '2' => Ok(CardMk1::Two),
            '3' => Ok(CardMk1::Three),
            '4' => Ok(CardMk1::Four),
            '5' => Ok(CardMk1::Five),
            '6' => Ok(CardMk1::Six),
            '7' => Ok(CardMk1::Seven),
            '8' => Ok(CardMk1::Eight),
            '9' => Ok(CardMk1::Nine),
            'T' => Ok(CardMk1::Ten),
            'J' => Ok(CardMk1::Jack),
            'Q' => Ok(CardMk1::Queen),
            'K' => Ok(CardMk1::King),
            'A' => Ok(CardMk1::Ace),
            _ => Err(anyhow!("Invalid card: {}", input)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CardMk2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card for CardMk2 {
    fn to_char(&self) -> char {
        match self {
            CardMk2::Joker => 'J',
            CardMk2::Two => '2',
            CardMk2::Three => '3',
            CardMk2::Four => '4',
            CardMk2::Five => '5',
            CardMk2::Six => '6',
            CardMk2::Seven => '7',
            CardMk2::Eight => '8',
            CardMk2::Nine => '9',
            CardMk2::Ten => 'T',
            CardMk2::Queen => 'Q',
            CardMk2::King => 'K',
            CardMk2::Ace => 'A',
        }
    }

    fn from_char(input: char) -> Result<Self> {
        let card_mk1 = CardMk1::from_char(input)?;
        Ok(card_mk1.into())
    }
}

impl From<CardMk1> for CardMk2 {
    fn from(card: CardMk1) -> Self {
        match card {
            CardMk1::Two => CardMk2::Two,
            CardMk1::Three => CardMk2::Three,
            CardMk1::Four => CardMk2::Four,
            CardMk1::Five => CardMk2::Five,
            CardMk1::Six => CardMk2::Six,
            CardMk1::Seven => CardMk2::Seven,
            CardMk1::Eight => CardMk2::Eight,
            CardMk1::Nine => CardMk2::Nine,
            CardMk1::Ten => CardMk2::Ten,
            CardMk1::Jack => CardMk2::Joker,
            CardMk1::Queen => CardMk2::Queen,
            CardMk1::King => CardMk2::King,
            CardMk1::Ace => CardMk2::Ace,
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

trait Hand: FromStr<Err = Error> + Ord + Copy {
    fn get_type(&self) -> HandType;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ArrayHand<CardType: Card> {
    cards: [CardType; 5],
}

type HandMk1 = ArrayHand<CardMk1>;
type HandMk2 = ArrayHand<CardMk2>;

impl Hand for ArrayHand<CardMk1> {
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

impl Hand for HandMk2 {
    fn get_type(&self) -> HandType {
        todo!()
    }
}

impl<CardType: Card> Ord for ArrayHand<CardType>
where
    Self: Hand,
{
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

impl<CardType: Card> PartialOrd for ArrayHand<CardType>
where
    Self: Hand,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<CardType: Card> Display for ArrayHand<CardType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{}", card.to_char())?;
        }
        Ok(())
    }
}

impl<CardType: Card> FromStr for ArrayHand<CardType> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let chars = s.chars();
        let cards_vec = chars.map(CardType::from_char).collect::<Result<Vec<_>>>()?;
        let cards = cards_vec
            .try_into()
            .map_err(|_| anyhow!("Invalid hand: {}", s))?;
        Ok(ArrayHand { cards })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HandWithBid<HandType: Hand> {
    hand: HandType,
    bid: Number,
}

impl<HandType: Hand> FromStr for HandWithBid<HandType> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (hand_str, bid_str) = s.split_once(" ").ok_or(anyhow!("Invalid input: {s}"))?;
        let hand = HandType::from_str(hand_str)?;
        let bid = bid_str.parse::<Number>()?;
        Ok(HandWithBid { hand, bid })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game<HandType: Hand> {
    hands: Vec<HandWithBid<HandType>>,
}

impl<HandType: Hand> Game<HandType> {
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

impl<HandType: Hand> FromStr for Game<HandType> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let hands = s
            .lines()
            .map(|line| line.parse::<HandWithBid<HandType>>())
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

    fn sample_input<HandType: Hand>() -> Game<HandType> {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        input.parse::<Game<HandType>>().unwrap()
    }

    #[test]
    fn test_parsing() {
        let game = sample_input();
        assert_eq!(5, game.hands.len());
        assert_eq!(
            HandWithBid {
                hand: HandMk1 {
                    cards: [
                        CardMk1::Three,
                        CardMk1::Two,
                        CardMk1::Ten,
                        CardMk1::Three,
                        CardMk1::King
                    ]
                },
                bid: 765
            },
            game.hands[0]
        );
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(
            HandMk1::from_str("AAAAA").unwrap().get_type(),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandMk1::from_str("AA8AA").unwrap().get_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandMk1::from_str("23332").unwrap().get_type(),
            HandType::FullHouse
        );
        assert_eq!(
            HandMk1::from_str("TTT98").unwrap().get_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandMk1::from_str("23432").unwrap().get_type(),
            HandType::TwoPair
        );
        assert_eq!(
            HandMk1::from_str("A23A4").unwrap().get_type(),
            HandType::OnePair
        );
        assert_eq!(
            HandMk1::from_str("23456").unwrap().get_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn test_comparison() {
        assert!(HandMk1::from_str("AA8AA").unwrap() > HandMk1::from_str("23456").unwrap());
        assert!(HandMk1::from_str("33332").unwrap() > HandMk1::from_str("2AAAA").unwrap());
        assert!(HandMk1::from_str("77888").unwrap() > HandMk1::from_str("77788").unwrap());
    }

    #[test]
    fn test_total_winnings() {
        let game: Game<HandMk1> = sample_input();
        assert_eq!(game.total_winnings(), 6440);
    }
}
