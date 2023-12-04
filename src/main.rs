use crate::framework::Day;

mod days {
    pub mod day00;
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
}
mod framework;

use framework::prelude;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(days::day01::Day1),
        Box::new(days::day02::Day2),
        Box::new(days::day03::Day3),
        Box::new(days::day04::Day4),
    ];

    for day in days.iter() {
        if day.day_number() == 0 {
            println!("WARNING: Did you forget to change the day_number() for one of the days?");
        }
        if let Some(output) = day.part1() {
            println!(
                "Day {}, part 1: {}",
                day.day_number(),
                output.unwrap_or_else(|err| err.to_string())
            );
        }
        if let Some(output) = day.part2() {
            println!(
                "Day {}, part 2: {}",
                day.day_number(),
                output.unwrap_or_else(|err| err.to_string())
            );
        }
    }
}
