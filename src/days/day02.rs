// Day 2: Cube Conundrum

use std::ops::Add;

use crate::framework::Day;
use crate::prelude::*;
use chumsky::prelude::*;
use chumsky::text::{keyword, whitespace};

pub struct Day2;

impl Day for Day2 {
    fn day_number(&self) -> u8 {
        2
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(Ok("Hello, world!".to_string()))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: u32,
    pulls: Vec<Inventory>,
}

impl Game {
    fn is_possible_with_inventory(&self, inventory: &Inventory) -> bool {
        let mut actual_minimum_inventory = Inventory::default();
        for pull in self.pulls.iter() {
            actual_minimum_inventory = actual_minimum_inventory + pull.clone().into();
        }
        actual_minimum_inventory.fits_in(inventory)
    }

    fn parser<'a>() -> impl Parser<&'a str, Game, Error = Simple<char>> {
        let int = text::int::<char, Simple<char>>(10).map(|it| it.parse::<u32>().unwrap());

        let color = choice((
            keyword("red").to(Color::RED),
            keyword("green").to(Color::GREEN),
            keyword("blue").to(Color::BLUE),
        ));

        let quantity = int.padded().then(color);

        let pull = quantity.separated_by(just(',').padded()).map(|it| {
            let mut inventory = Inventory::default();
            for (quantity, color) in it {
                match color {
                    Color::RED => inventory.red += quantity,
                    Color::GREEN => inventory.green += quantity,
                    Color::BLUE => inventory.blue += quantity,
                }
            }
            inventory
        });

        let all_pulls = pull.separated_by(just(';').padded()).collect();

        keyword("Game")
            .padded()
            .ignore_then(int)
            .padded()
            .then_ignore(just(':'))
            .then(all_pulls)
            .map(|(id, pulls)| Game { id, pulls })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Inventory {
    red: u32,
    green: u32,
    blue: u32,
}

impl Add for Inventory {
    type Output = Inventory;

    fn add(self, other: Inventory) -> Inventory {
        Inventory {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Inventory {
    fn fits_in(&self, other: &Inventory) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[cfg(test)]
mod test {
    use std::ops::Not;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            "Hello, world!".to_string(),
            super::Day2.part1().unwrap().unwrap()
        );
    }

    #[test]
    fn test_parser() {
        let input_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_game = Game {
            id: 1,
            pulls: vec![
                Inventory {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Inventory {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Inventory {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };
        let result = Game::parser().parse(input_string);
        // assert_eq!(expected_game
    }

    #[test]
    fn test_inventory_fits_in() {
        assert!(Inventory {
            red: 1,
            green: 1,
            blue: 1,
        }
        .fits_in(&Inventory {
            red: 1,
            green: 2,
            blue: 1,
        }));
        assert!((Inventory {
            red: 1,
            green: 1,
            blue: 4,
        }
        .fits_in(&Inventory {
            red: 5,
            green: 1,
            blue: 1,
        }))
        .not());
    }
}
