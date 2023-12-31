use std::time::Instant;

use clap::Parser;
use framework::prelude;
use framework::{format_duration, Day};
use prelude::*;

mod framework;
mod days {
    pub mod day00;
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
}

lazy_static! {
    static ref DAYS: Vec<Box<dyn Day + Sync>> = vec![
        Box::new(days::day01::Day1),
        Box::new(days::day02::Day2),
        Box::new(days::day03::Day3),
        Box::new(days::day04::Day4),
        Box::new(days::day05::Day5),
        Box::new(days::day06::Day6),
        Box::new(days::day07::Day7),
        Box::new(days::day08::Day8),
        Box::new(days::day09::Day9),
        Box::new(days::day10::Day10),
        Box::new(days::day11::Day11),
        Box::new(days::day12::Day12),
        Box::new(days::day13::Day13),
        Box::new(days::day14::Day14),
        Box::new(days::day15::Day15),
        Box::new(days::day16::Day16),
        Box::new(days::day17::Day17),
        Box::new(days::day18::Day18),
        Box::new(days::day19::Day19),
        Box::new(days::day20::Day20),
        Box::new(days::day21::Day21),
        Box::new(days::day22::Day22),
    ];
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long)]
    part: Option<u8>,
    /// Run a custom script instead of the day's code. `day` is required if this option is used.
    #[arg(short, long)]
    script: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(script) = args.script {
        let day_number = args.day.expect("Must specify a day when using --script");

        let result = DAYS
            .iter()
            .find(|day| day.day_number() == day_number)
            .expect("Day not found")
            .run_script(&script)
            .unwrap();

        if !result {
            println!("\"{}\" script not found.", script);
        }
        return;
    }

    for day in DAYS.iter() {
        if day.day_number() == 0 {
            println!("WARNING: Did you forget to change the day_number() for one of the days?");
        }

        if args.day.is_some() && args.day != Some(day.day_number()) {
            continue;
        }

        if args.part.is_none() || args.part == Some(1) {
            let day1_start = Instant::now();
            if let Some(output) = day.part1() {
                let day1_elapsed = day1_start.elapsed();
                println!(
                    "Day {}, part 1 ({}): {}",
                    day.day_number(),
                    format_duration(&day1_elapsed),
                    output.unwrap_or_else(|err| err.to_string()),
                );
            }
        }
        if args.part.is_none() || args.part == Some(2) {
            let day2_start = Instant::now();
            if let Some(output) = day.part2() {
                let day2_elapsed = day2_start.elapsed();
                println!(
                    "Day {}, part 2 ({}): {}",
                    day.day_number(),
                    format_duration(&day2_elapsed),
                    output.unwrap_or_else(|err| err.to_string()),
                );
            }
        }
    }
}
