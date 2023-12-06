// Day 6: Wait For It

use std::str::FromStr;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day6;

impl Day for Day6 {
    fn day_number(&self) -> u8 {
        6
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let input = Leaderboard::from_str(include_str!("./day06_input.txt"))?;
            input.get_part1_solution().to_string().pipe(anyhow::Ok)
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let input = Race::from_str_with_bad_kerning(include_str!("./day06_input.txt"))?;
            input.ways_to_win().to_string().pipe(anyhow::Ok)
        }))
    }
}

/// Gets the distance in millimeters travelled by a boat in a race `time` milliseconds long,
/// when you hold the boat down for `button_hold` milliseconds.
/// `button_hold` must be less than `time`.
fn simulate_race(time: u64, button_hold: u64) -> u64 {
    let remaining_time = time - button_hold;
    let velocity = button_hold;
    let distance = velocity * remaining_time;
    distance
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    /// The best distance ever recorded in this race
    distance: u64,
}

impl Race {
    fn from_str_with_bad_kerning(input: &str) -> Result<Self> {
        let lines = input.lines().collect::<Vec<_>>();
        let time_line = lines[0].trim_start_matches("Time: ");
        let time = time_line.replace(" ", "").parse::<u64>()?;
        let distance_line = lines[1].trim_start_matches("Distance: ");
        let distance = distance_line.replace(" ", "").parse::<u64>()?;
        Ok(Race { time, distance })
    }

    fn ways_to_win(&self) -> u64 {
        // let odd_midpoint = if self.time % 2 == 0 {
        //     None
        // } else {
        //     Some(self.time / 2)
        // };
        // let halfway = self.time / 2;
        let possible_button_holds = 1..self.time;
        let possible_wins = possible_button_holds
            .par_bridge()
            .filter(|hold| simulate_race(self.time, *hold) > self.distance);
        possible_wins.count() as u64
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Leaderboard {
    races: Vec<Race>,
}

impl Leaderboard {
    fn get_part1_solution(&self) -> u64 {
        let ways_to_win_each_race = self.races.iter().map(|race| race.ways_to_win());
        ways_to_win_each_race.product()
    }
}

impl FromStr for Leaderboard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect::<Vec<_>>();
        let time_line = lines[0].trim_start_matches("Time: ");
        let times = time_line
            .split_whitespace()
            .map(|s| s.parse::<u64>().map_err(anyhow::Error::from))
            .collect::<Result<Vec<u64>>>()?;
        let distance_line = lines[1].trim_start_matches("Distance: ");
        let distances = distance_line
            .split_whitespace()
            .map(|s| s.parse::<u64>().map_err(anyhow::Error::from))
            .collect::<Result<Vec<u64>>>()?;

        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| Race {
                time: *time,
                distance: *distance,
            })
            .collect::<Vec<_>>();
        Ok(Leaderboard { races })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("1159152".to_string(), super::Day6.part1().unwrap().unwrap());
    }

    fn sample_input() -> Leaderboard {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        Leaderboard::from_str(input).unwrap()
    }

    fn sample_input_pt_2() -> Race {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        Race::from_str_with_bad_kerning(input).unwrap()
    }

    #[test]
    fn test_simulate_race() {
        assert_eq!(simulate_race(7, 0), 0);
        assert_eq!(simulate_race(7, 1), 6);
        assert_eq!(simulate_race(7, 2), 10);
        assert_eq!(simulate_race(7, 3), 12);
        assert_eq!(simulate_race(7, 4), 12);
        assert_eq!(simulate_race(7, 5), 10);
        assert_eq!(simulate_race(7, 6), 6);
        assert_eq!(simulate_race(7, 7), 0);
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            sample_input(),
            Leaderboard {
                races: vec![
                    Race {
                        time: 7,
                        distance: 9,
                    },
                    Race {
                        time: 15,
                        distance: 40,
                    },
                    Race {
                        time: 30,
                        distance: 200,
                    }
                ]
            }
        )
    }

    #[test]
    fn test_ways_to_win() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.ways_to_win(), 4);
    }

    #[test]
    fn test_part1_solution() {
        let leaderboard = sample_input();
        assert_eq!(leaderboard.get_part1_solution(), 288);
    }

    #[test]
    fn test_parse_with_bad_kerning() {
        let race = sample_input_pt_2();
        assert_eq!(
            race,
            Race {
                time: 71530,
                distance: 940200,
            }
        )
    }

    #[test]
    fn test_ways_to_win_big_race() {
        let race = sample_input_pt_2();
        assert_eq!(race.ways_to_win(), 71503);
    }
}
