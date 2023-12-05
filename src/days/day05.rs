// Day 5: If You Give A Seed A Fertilizer

use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

use regex::Regex;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day5;

fn puzzle_input() -> Result<Almanac> {
    Almanac::from_str(include_str!("./day05_input.txt"))
}

impl Day for Day5 {
    fn day_number(&self) -> u8 {
        5
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let almanac = puzzle_input()?;
            let lowest_location = almanac.lowest_location()?;
            Ok(lowest_location.to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let almanac = puzzle_input()?;
            let lowest_location = almanac.lowest_location_with_ranges()?;
            Ok(lowest_location.to_string())
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    seeds: Vec<u64>,
    maps: HashMap<(Category, Category), AlmanacMapList>,
}

#[derive(Debug, PartialEq, Eq)]
struct AlmanacMapList(Vec<AlmanacMap>);

impl AlmanacMapList {
    fn map(&self, input: u64) -> u64 {
        for rule in self.0.iter().rev() {
            if input >= rule.source_range_start {
                if input < rule.source_range_start + rule.range_length {
                    return rule.destination_range_start + (input - rule.source_range_start);
                } else {
                    // Since the list is sorted, if we reach this point, we know none of the rules apply
                    return input;
                }
            }
        }
        input
    }

    fn map_range(&self, input: &RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        let mut output = Vec::new();
        let mut current_range = input.clone();
        for rule in self.0.iter() {
            let rule_source_end_inclusive = rule.source_range_start + rule.range_length - 1;
            if *current_range.start() < rule.source_range_start {
                // input extends before the current rule, so insert a passthrough range
                let passthrough_end = (rule.source_range_start - 1).min(*current_range.end());
                let passthrough_range = *current_range.start()..=passthrough_end;
                output.push(passthrough_range);
                current_range = (passthrough_end + 1)..=*current_range.end();
                if current_range.is_empty() {
                    return output;
                }
            }
            // now that's taken care of, the current range either begins at, or in the middle of, or after, the current rule
            // if it's after, we can skip to the next rule
            if *current_range.start() > rule_source_end_inclusive {
                continue;
            }
            // now it's definitely either at or in the middle of the rule's source range
            let source_begin = *current_range.start();
            let source_end = (*current_range.end()).min(rule_source_end_inclusive);
            let source_offset = source_begin - rule.source_range_start;
            let length = source_end - source_begin;
            let destination_start = rule.destination_range_start + source_offset;
            let destination_end = destination_start + length;
            let range = destination_start..=destination_end;
            output.push(range);
            current_range = (source_end + 1)..=*current_range.end();
            if current_range.is_empty() {
                return output;
            }
        }
        // anything left over gets passed through
        output.push(current_range);
        output
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AlmanacMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Almanac {
    fn map_seed_to_location(&self, seed: u64) -> Result<u64> {
        let mut current_number = seed;
        for categories in [
            Category::Seed,
            Category::Soil,
            Category::Fertilizer,
            Category::Water,
            Category::Light,
            Category::Temperature,
            Category::Humidity,
            Category::Location,
        ]
        .windows(2)
        {
            let key = (categories[0], categories[1]);
            let map = self
                .maps
                .get(&key)
                .ok_or(anyhow!("Couldn't find map list for {:?}", &key))?;
            current_number = map.map(current_number);
        }
        Ok(current_number)
    }

    fn map_seed_range_to_location_ranges(
        &self,
        seed: &RangeInclusive<u64>,
    ) -> Result<Vec<RangeInclusive<u64>>> {
        let mut current_ranges = vec![seed.clone()];
        for categories in [
            Category::Seed,
            Category::Soil,
            Category::Fertilizer,
            Category::Water,
            Category::Light,
            Category::Temperature,
            Category::Humidity,
            Category::Location,
        ]
        .windows(2)
        {
            let key = (categories[0], categories[1]);
            let map_list = self
                .maps
                .get(&key)
                .ok_or(anyhow!("Couldn't find map list for {:?}", &key))?;
            current_ranges = current_ranges
                .iter()
                .flat_map(|range| map_list.map_range(range))
                .collect();
        }
        Ok(current_ranges)
    }

    fn lowest_location(&self) -> Result<u64> {
        let mut lowest_location = u64::MAX;
        for seed in self.seeds.iter() {
            let location = self.map_seed_to_location(*seed)?;
            if location < lowest_location {
                lowest_location = location;
            }
        }
        Ok(lowest_location)
    }

    fn seed_ranges(&self) -> Vec<RangeInclusive<u64>> {
        self.seeds
            .chunks_exact(2)
            .map(|range| {
                let start = range[0];
                let length = range[1];
                start..=(start + length)
            })
            .collect()
    }

    fn lowest_location_with_ranges(&self) -> Result<u64> {
        let seed_ranges = self.seed_ranges();
        let location_ranges: Vec<RangeInclusive<u64>> = seed_ranges
            .iter()
            .map(|seed_range| self.map_seed_range_to_location_ranges(&seed_range))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        let lowest_location = location_ranges
            .iter()
            .map(|it| it.start())
            .min()
            .ok_or(anyhow!("no location range found"))
            .copied();

        lowest_location
    }
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
                .map(|it| u64::from_str(it).map_err(anyhow::Error::from))
                .collect::<Result<Vec<u64>>>()?;
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
                let destination_range_start = u64::from_str(captures.get(1).unwrap().as_str())?;
                let source_range_start = u64::from_str(captures.get(2).unwrap().as_str())?;
                let range_length = u64::from_str(captures.get(3).unwrap().as_str())?;
                maps.push(AlmanacMap {
                    destination_range_start,
                    source_range_start,
                    range_length,
                });
                remaining_input = &remaining_input[captures.get(0).unwrap().len()..];
            }
            maps.sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));

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
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            "322500873".to_string(),
            super::Day5.part1().unwrap().unwrap()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            "108956227".to_string(),
            super::Day5.part2().unwrap().unwrap()
        );
    }

    fn example_input() -> Almanac {
        Almanac::from_str(include_str!("./day05_example_input.txt")).unwrap()
    }

    #[test]
    fn test_parsing() {
        let result = example_input();
        assert_eq!(result.seeds, vec![79, 14, 55, 13]);
        assert_eq!(result.maps.len(), 7);
        assert_eq!(
            result.maps.get(&(Category::Seed, Category::Soil)).unwrap(),
            &AlmanacMapList(vec![
                AlmanacMap {
                    destination_range_start: 52,
                    source_range_start: 50,
                    range_length: 48
                },
                AlmanacMap {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_length: 2
                },
            ])
        );
    }

    #[test]
    fn test_mapping() {
        let almanac = example_input();
        let seed_to_soil = almanac.maps.get(&(Category::Seed, Category::Soil)).unwrap();
        assert_eq!(seed_to_soil.map(79), 81);
        assert_eq!(seed_to_soil.map(14), 14);
        assert_eq!(seed_to_soil.map(55), 57);
        assert_eq!(seed_to_soil.map(13), 13);
    }

    #[test]
    fn test_problem_mappings() {
        let almanac = example_input();
        let fertilizer_to_water = almanac
            .maps
            .get(&(Category::Fertilizer, Category::Water))
            .unwrap();
        let water_to_light = almanac
            .maps
            .get(&(Category::Water, Category::Light))
            .unwrap();
        assert_eq!(fertilizer_to_water.map(53), 49);
        assert_eq!(water_to_light.map(81), 74);
    }

    #[test]
    fn test_seed_to_location_mapping() {
        let almanac = example_input();
        assert_eq!(almanac.map_seed_to_location(79).unwrap(), 82);
        assert_eq!(almanac.map_seed_to_location(14).unwrap(), 43);
        assert_eq!(almanac.map_seed_to_location(55).unwrap(), 86);
        assert_eq!(almanac.map_seed_to_location(13).unwrap(), 35);
    }

    #[test]
    fn test_map_ranges() {
        let almanac = example_input();
        let seed_to_soil = almanac.maps.get(&(Category::Seed, Category::Soil)).unwrap();
        assert_eq!(
            seed_to_soil.map_range(&(47..=105)),
            vec![47..=49, 52..=99, 50..=51, 100..=105]
        );
    }

    #[test]
    fn test_problem_ranges() {
        let almanac = example_input();
        let seed_to_soil = almanac.maps.get(&(Category::Seed, Category::Soil)).unwrap();
        let fertizilizer_to_water = almanac
            .maps
            .get(&(Category::Fertilizer, Category::Water))
            .unwrap();
        let temperature_to_humidity = almanac
            .maps
            .get(&(Category::Temperature, Category::Humidity))
            .unwrap();
        assert_eq!(seed_to_soil.map_range(&(79..=93)), vec![81..=95]);
        assert_eq!(fertizilizer_to_water.map_range(&(81..=95)), vec![81..=95]);
        assert_eq!(
            fertizilizer_to_water.map_range(&(57..=70)),
            vec![53..=56, 61..=70]
        );
        assert_eq!(temperature_to_humidity.map_range(&(93..=99)), vec![93..=99]);
        assert_eq!(
            temperature_to_humidity.map_range(&(68..=74)),
            vec![69..=69, 0..=0, 70..=74]
        );
    }

    #[test]
    fn test_ranges_equivalence() {
        let almanac = example_input();
        let mut current_ranges = almanac.seed_ranges();
        for categories in [
            Category::Seed,
            Category::Soil,
            Category::Fertilizer,
            Category::Water,
            Category::Light,
            Category::Temperature,
            Category::Humidity,
            Category::Location,
        ]
        .windows(2)
        {
            let key = (categories[0], categories[1]);
            let map_list = almanac.maps.get(&key).unwrap();
            let ranges = current_ranges.clone();
            let all_numbers: Vec<u64> = current_ranges.iter().flat_map(|it| it.clone()).collect();
            let mapped_numbers: HashSet<u64> =
                all_numbers.iter().map(|it| map_list.map(*it)).collect();
            current_ranges = current_ranges
                .iter()
                .flat_map(|range| map_list.map_range(range))
                .collect();
            let numbers_in_ranges: HashSet<u64> =
                current_ranges.iter().flat_map(|it| it.clone()).collect();
            assert_eq!(
                numbers_in_ranges, mapped_numbers,
                "{:?} mapping: {:?}",
                key, ranges
            )
        }
    }

    #[test]
    fn test_lowest_location_with_ranges() {
        let almanac = example_input();
        assert_eq!(almanac.lowest_location_with_ranges().unwrap(), 46);
    }
}
