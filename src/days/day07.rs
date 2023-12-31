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
            Ok(puzzle_input::<HandMk1>()?.total_winnings().to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            Ok(puzzle_input::<HandMk2>()?.total_winnings().to_string())
        }))
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

impl From<CardMk2> for CardMk1 {
    fn from(value: CardMk2) -> Self {
        match value {
            CardMk2::Joker => CardMk1::Jack,
            CardMk2::Two => CardMk1::Two,
            CardMk2::Three => CardMk1::Three,
            CardMk2::Four => CardMk1::Four,
            CardMk2::Five => CardMk1::Five,
            CardMk2::Six => CardMk1::Six,
            CardMk2::Seven => CardMk1::Seven,
            CardMk2::Eight => CardMk1::Eight,
            CardMk2::Nine => CardMk1::Nine,
            CardMk2::Ten => CardMk1::Ten,
            CardMk2::Queen => CardMk1::Queen,
            CardMk2::King => CardMk1::King,
            CardMk2::Ace => CardMk1::Ace,
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
        let jokers = self
            .cards
            .iter()
            .filter(|card| **card == CardMk2::Joker)
            .count();
        if jokers == 5 {
            return HandType::FiveOfAKind;
        }
        let matching_remaining_cards = self
            .cards
            .iter()
            .filter(|card| **card != CardMk2::Joker)
            .counts();
        let counts = matching_remaining_cards
            .values()
            .sorted()
            .rev()
            .copied()
            .collect_vec();
        if counts.get(0).map(|it| it + jokers) == Some(5) {
            HandType::FiveOfAKind
        } else if counts.get(0).map(|it| it + jokers) == Some(4) {
            HandType::FourOfAKind
        } else if counts.get(0).map(|it| it + jokers) == Some(3) {
            // note: there's no circumstance where a joker would be applied to a full house
            // it would become a four-of-a-kind instead
            if counts.get(1) == Some(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if counts.get(0).map(|it| it + jokers) == Some(2) {
            // note: there's no circumstance where a joker would be applied to a two-pair
            // it would become a three-of-a-kind instead
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

    #[test]
    fn test_part2() {
        assert_eq!("251135960".to_string(), super::Day7.part2().unwrap().unwrap());
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

    #[test]
    fn test_jokers() {
        assert_eq!(
            HandMk2::from_str("QJJQ2").unwrap().get_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandMk2::from_str("T55J5").unwrap().get_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandMk2::from_str("KTJJT").unwrap().get_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandMk2::from_str("QQQJA").unwrap().get_type(),
            HandType::FourOfAKind
        );
    }

    #[test]
    fn test_joker_full_house() {
        assert_eq!(
            HandMk2::from_str("QQJTT").unwrap().get_type(),
            HandType::FullHouse
        );
    }

    #[test]
    fn test_joker_ordering() {
        assert!(HandMk2::from_str("QQQQ2").unwrap() > HandMk2::from_str("JKKK2").unwrap());
    }

    #[test]
    fn test_random_real_inputs() {
        fn get_hand_type(input: &str) -> HandType {
            HandMk2::from_str(input).unwrap().get_type()
        }
        assert_eq!(get_hand_type("528Q8"), HandType::OnePair);
        assert_eq!(get_hand_type("72776"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("TTJJT"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("K68JJ"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("68868"), HandType::FullHouse);
        assert_eq!(get_hand_type("4A527"), HandType::HighCard);
        assert_eq!(get_hand_type("8T843"), HandType::OnePair);
        assert_eq!(get_hand_type("AQ347"), HandType::HighCard);
        assert_eq!(get_hand_type("737AJ"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("9Q93Q"), HandType::TwoPair);
        assert_eq!(get_hand_type("47J47"), HandType::FullHouse);
        assert_eq!(get_hand_type("5K26T"), HandType::HighCard);
        assert_eq!(get_hand_type("6AK6A"), HandType::TwoPair);
        assert_eq!(get_hand_type("T33JJ"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("5A2J6"), HandType::OnePair);
        assert_eq!(get_hand_type("6JQ4K"), HandType::OnePair);
        assert_eq!(get_hand_type("QQQ6Q"), HandType::FourOfAKind);
    }

    #[test]
    fn test_problematic_hands() {
        assert_eq!(
            HandMk2::from_str("JJJJJ").unwrap().get_type(),
            HandType::FiveOfAKind
        );
    }

    // #[test]
    // fn test_brute_force_equivalence() {
    //     let game: Game<HandMk2> = puzzle_input().unwrap();
    //     for hand_with_bid in game.hands.iter() {
    //         let mk2_type = hand_with_bid.hand.get_type();
    //         let possible_mk1_cards = hand_with_bid
    //             .hand
    //             .cards
    //             .iter()
    //             .map(|card| match card {
    //                 CardMk2::Joker => vec![
    //                     CardMk1::Two,
    //                     CardMk1::Three,
    //                     CardMk1::Four,
    //                     CardMk1::Five,
    //                     CardMk1::Six,
    //                     CardMk1::Seven,
    //                     CardMk1::Eight,
    //                     CardMk1::Nine,
    //                     CardMk1::Ten,
    //                     CardMk1::Queen,
    //                     CardMk1::King,
    //                     CardMk1::Ace,
    //                 ],
    //                 _ => vec![(*card).conv::<CardMk1>()],
    //             })
    //             .collect_vec();
    //         let mut possible_mk1_hands = vec![vec![]];
    //         for possible_cards in possible_mk1_cards.iter() {
    //             let mut new_possible_mk1_hands = vec![];
    //             for possible_hand in possible_mk1_hands.iter() {
    //                 for possible_card in possible_cards.iter() {
    //                     let mut new_possible_hand = possible_hand.clone();
    //                     new_possible_hand.push(*possible_card);
    //                     new_possible_mk1_hands.push(new_possible_hand);
    //                 }
    //             }
    //             possible_mk1_hands = new_possible_mk1_hands;
    //         }
    //         let best_mk1_hand = possible_mk1_hands
    //             .iter()
    //             .map(|cards| HandMk1 {
    //                 cards: cards.clone().try_into().unwrap(),
    //             })
    //             .max()
    //             .unwrap();

    //         let mk1_type = best_mk1_hand.get_type();
    //         assert_eq!(
    //             mk2_type, mk1_type,
    //             "{} interpreted as {}, {:?}",
    //             hand_with_bid.hand, best_mk1_hand, mk1_type
    //         );
    //     }
    // }

    #[test]
    fn test_winnings_mk2() {
        let game: Game<HandMk2> = sample_input();
        assert_eq!(game.total_winnings(), 5905);
    }
}
