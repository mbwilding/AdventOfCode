use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn part_1(lines: &[String]) -> u32 {
    lines
        .iter()
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
        .iter()
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

lazy_static! {
    pub static ref NUMBER_LUT: HashMap<&'static str, u32> = {
        HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ])
    };
}
