use rayon::prelude::*;

pub fn part_1(lines: &[String]) -> u32 {
    lines.par_iter().map(|_line| 0).sum()
}

pub fn part_2(lines: &[String]) -> u32 {
    lines.par_iter().map(|_line| 0).sum()
}
