use crate::logic::*;
use anyhow::Result;
use common_2023::read_lines;

mod logic;
mod tests;

fn main() -> Result<()> {
    let lines = read_lines("2023/!data/day1/real.txt")?;

    let part_1 = process_lines_part_1(&lines);
    println!("Part 1: {}", part_1);

    let part_2 = process_lines_part_2(&lines);
    println!("Part 2: {}", part_2);

    Ok(())
}
