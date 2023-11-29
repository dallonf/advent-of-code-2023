use crate::framework::Day;

mod days {
    pub mod day00;
}
mod framework;

use framework::prelude;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![Box::new(days::day00::Day0)];

    for day in days.iter() {
        if let Some(output) = day.part1() {
            println!("Day {}, part 1: {}", day.day_number(), output);
        }
        if let Some(output) = day.part2() {
            println!("Day {}, part 2: {}", day.day_number(), output);
        }
    }
}
