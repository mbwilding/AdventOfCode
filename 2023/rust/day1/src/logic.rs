use crate::data::*;
use rayon::prelude::*;

pub fn part_1(lines: &[String]) -> u32 {
    lines
        .par_iter()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();

            let first = digits[0];
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum()
}

pub fn part_2(lines: &[String]) -> u32 {
    lines
        .par_iter()
        .map(|line| {
            let mut first = 0u32;
            let mut last = 0u32;
            let mut current_word = String::new();

            for c in line.chars() {
                if c.is_alphabetic() {
                    current_word.push(c);
                    for i in 0..current_word.len() {
                        if let Some(&number) = NUMBER_LUT.get(&current_word[i..]) {
                            if first == 0 {
                                first = number;
                            }
                            last = number;
                            break;
                        }
                    }
                } else if c.is_ascii_digit() {
                    let digit = c.to_digit(10).unwrap();
                    if first == 0 {
                        first = digit;
                    }
                    last = digit;
                    current_word.clear();
                }
            }

            first * 10 + last
        })
        .sum()
}
