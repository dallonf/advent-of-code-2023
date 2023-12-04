pub mod prelude;
pub mod try_block;

pub trait Day {
    fn day_number(&self) -> u8;
    /// Returns `None` if the day is not yet implemented.
    fn part1(&self) -> Option<anyhow::Result<String>>;
    /// Returns `None` if the day is not yet implemented.
    fn part2(&self) -> Option<anyhow::Result<String>>;
}
