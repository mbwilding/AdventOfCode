use crate::logic::*;
use common_2023::*;

mod data;
mod logic;
mod tests;

pub const DAY: i32 = 3;

fn main() {
    run(DAY, "real", part_1, part_2);
}
