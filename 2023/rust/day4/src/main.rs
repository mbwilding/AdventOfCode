use crate::logic::*;
use common_2023::*;

mod logic;
mod tests;

pub const DAY: u8 = 4;

fn main() {
    run(DAY, "real", part_1, part_2);
}
