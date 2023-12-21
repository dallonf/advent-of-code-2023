pub mod grid;
pub mod prelude;
pub mod try_block;

pub trait Day {
    fn day_number(&self) -> u8;
    /// Returns `None` if the day is not yet implemented.
    fn part1(&self) -> Option<anyhow::Result<String>>;
    /// Returns `None` if the day is not yet implemented.
    fn part2(&self) -> Option<anyhow::Result<String>>;

    /// Returns true if the script was run, false if the script was not found.
    fn run_script(&self, _name: &str) -> anyhow::Result<bool> {
        Ok(false)
    }
}
