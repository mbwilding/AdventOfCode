use crate::logic::*;
use anyhow::Result;
use common_2023::*;

mod logic;
mod tests;

fn main() -> Result<()> {
    run(1, "real", part_1, part_2)?;

    Ok(())
}
