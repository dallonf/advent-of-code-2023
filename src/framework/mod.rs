use std::time::Duration;

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

pub fn format_duration(input: &Duration) -> String {
    let total_seconds = input.as_secs();
    let millis = input.as_millis() % 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let mut output = "".to_string();
    if minutes > 0 {
        output += &format!("{}m ", minutes);
    }
    if seconds > 0 {
        output += &format!("{}s ", seconds);
    }
    output += &format!("{}ms", millis);
    output
}
