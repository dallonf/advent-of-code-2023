// Day 2: Cube Conundrum

use std::ops::Add;
use std::str::FromStr;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day2;

fn puzzle_input() -> Result<Vec<Game>> {
    include_str!("./day02_input.txt")
        .lines()
        .par_bridge()
        .map(Game::from_str)
        .collect()
}

impl Day for Day2 {
    fn day_number(&self) -> u8 {
        2
    }

    fn part1(&self) -> Option<Result<String>> {
        let games = puzzle_input();
        let inventory = Inventory {
            red: 12,
            green: 13,
            blue: 14,
        };
        let possible_game_ids = games.map(|it| inventory.possible_game_ids(&it));
        let sum = possible_game_ids.map(|it| it.iter().sum::<u32>());
        Some(sum.map(|it| it.to_string()))
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
        self.pulls.iter().all(|pull| pull.fits_in(inventory))
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let error = || anyhow!("Invalid game string: {}", s);
        let game_match = regex::Regex::new(r"Game (\d+): (.*)")
            .unwrap()
            .captures(s)
            .ok_or_else(error)?;
        let id = game_match.get(1).unwrap().as_str().parse::<u32>()?;
        let pulls = {
            let pulls_strings = game_match
                .get(2)
                .unwrap()
                .as_str()
                .split(";")
                .map(str::trim);
            pulls_strings
                .map(|it| {
                    let qty_strings = it.split(",").map(str::trim);
                    let mut inventory = Inventory::default();
                    for qty_string in qty_strings {
                        let qty_match = regex::Regex::new(r"(\d+) (red|green|blue)")
                            .unwrap()
                            .captures(qty_string)
                            .ok_or_else(error)?;
                        let qty = qty_match.get(1).unwrap().as_str().parse::<u32>()?;
                        let color = match qty_match.get(2).unwrap().as_str() {
                            "red" => Color::RED,
                            "green" => Color::GREEN,
                            "blue" => Color::BLUE,
                            _ => return Err(error()),
                        };
                        match color {
                            Color::RED => inventory.red += qty,
                            Color::GREEN => inventory.green += qty,
                            Color::BLUE => inventory.blue += qty,
                        }
                    }
                    Ok(inventory)
                })
                .collect::<Result<Vec<_>>>()
        }?;
        Ok(Game { id, pulls })
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

    fn possible_game_ids<'a>(&self, games: impl IntoIterator<Item = &'a Game>) -> Vec<u32> {
        games
            .into_iter()
            .filter(|game| game.is_possible_with_inventory(self))
            .map(|game| game.id)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::ops::Not;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            "2505".to_string(),
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
        let result = Game::from_str(input_string).unwrap();
        assert_eq!(expected_game, result);
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

    #[test]
    fn possible_game_ids() {
        let test_input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let games = test_input
            .lines()
            .map(Game::from_str)
            .collect::<Result<Vec<_>>>()
            .unwrap();
        let inventory = Inventory {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert_eq!(inventory.possible_game_ids(&games), vec![1, 2, 5]);
    }
}
