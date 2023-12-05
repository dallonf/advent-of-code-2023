// Day 5: If You Give A Seed A Fertilizer

use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

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

#[derive(Debug, PartialEq, Eq)]
struct AlmanacMapList(Vec<AlmanacMap>);

#[derive(Debug, PartialEq, Eq)]
struct AlmanacMap {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: usize,
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let remaining_input = input;
        let (remaining_input, seeds) = {
            let captures = Regex::new("^seeds: ([ 0-9]+)\n\n")
                .unwrap()
                .captures(remaining_input)
                .ok_or(anyhow!("expected \"seeds:\", found: \"{remaining_input}\""))?;
            let seed_list = captures
                .get(1)
                .unwrap()
                .as_str()
                .split(" ")
                .map(|it| u32::from_str(it).map_err(anyhow::Error::from))
                .collect::<Result<Vec<u32>>>()?;
            (
                &remaining_input[captures.get(0).unwrap().len()..],
                seed_list,
            )
        };

        let header_regex = Regex::new("^\n?([a-z]+)-to-([a-z]+) map:\n").unwrap();
        let map_regex = Regex::new("^([0-9]+) ([0-9]+) ([0-9]+)\n").unwrap();
        let mut remaining_input = remaining_input;
        let mut map_lists = HashMap::new();
        while !remaining_input.trim().is_empty() {
            let captures = header_regex.captures(remaining_input).ok_or(anyhow!(
                "expected \"[category]-to-[category] map:\", found: \"{remaining_input}\""
            ))?;
            let category1 = Category::from_str(captures.get(1).unwrap().as_str())?;
            let category2 = Category::from_str(captures.get(2).unwrap().as_str())?;
            let key = (category1, category2);

            let mut maps = Vec::new();
            remaining_input = &remaining_input[captures.get(0).unwrap().len()..];
            fn not_newline(input: &str) -> bool {
                let next_char = input.chars().nth(0);
                next_char != Some('\n') && next_char != None
            }
            while not_newline(remaining_input) {
                let captures = map_regex.captures(remaining_input).ok_or(anyhow!(
                    "expected \"[destination] [source] [length]\", found: \"{remaining_input}\""
                ))?;
                let destination_range_start = u32::from_str(captures.get(1).unwrap().as_str())?;
                let source_range_start = u32::from_str(captures.get(2).unwrap().as_str())?;
                let range_length = usize::from_str(captures.get(3).unwrap().as_str())?;
                maps.push(AlmanacMap {
                    destination_range_start,
                    source_range_start,
                    range_length,
                });
                remaining_input = &remaining_input[captures.get(0).unwrap().len()..];
            }

            map_lists.insert(key, AlmanacMapList(maps));
        }

        Ok(Almanac {
            seeds,
            maps: map_lists,
        })
    }
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
        assert_eq!(result.seeds, vec![79, 14, 55, 13]);
        assert_eq!(result.maps.len(), 7);
        assert_eq!(
            result.maps.get(&(Category::Seed, Category::Soil)).unwrap(),
            &AlmanacMapList(vec![
                AlmanacMap {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2
                },
                AlmanacMap {
                    destination_range_start: 52,
                    source_range_start: 50,
                    range_length: 48
                }
            ])
        );
    }
}
