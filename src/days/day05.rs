// Day 5: If You Give A Seed A Fertilizer

use std::collections::HashMap;
use std::str::FromStr;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day5;

impl Day for Day5 {
    fn day_number(&self) -> u8 {
        5
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || Ok("Hello, world!".to_string())))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(anyhow!("Invalid category: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: HashMap<(Category, Category), AlmanacMapList>,
}

#[derive(Debug)]
struct AlmanacMapList(Vec<AlmanacMap>);

#[derive(Debug)]
struct AlmanacMap {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("./day05_example_input.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            "Hello, world!".to_string(),
            super::Day5.part1().unwrap().unwrap()
        );
    }

    #[test]
    fn test_parsing() {
        let result = Almanac::from_str(example_input()).unwrap();
        print!("{:#?}", result);
        assert!(false);
    }
}
