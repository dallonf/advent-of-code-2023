pub mod prelude;

pub trait Day {
    fn day_number(&self) -> u8;
    fn part1(&self) -> Option<String>;
    fn part2(&self) -> Option<String>;
}
