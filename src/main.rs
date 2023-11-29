use crate::framework::Day;

mod days;
mod framework;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![Box::new(days::Day0)];
    for day in days.iter() {
        if let Some(output) = day.part1() {
            println!("Day {}, part 1: {}", day.day_number(), output);
        }
        if let Some(output) = day.part2() {
            println!("Day {}, part 2: {}", day.day_number(), output);
        }
    }
}
