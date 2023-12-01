use rayon::prelude::*;
use std::collections::HashMap;

pub fn part_1(lines: &[String]) -> u32 {
    lines
        .par_iter()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum()
}

pub fn part_2(lines: &[String]) -> u32 {
    let number_lut = word_to_number_lut();

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
                        if let Some(&number) = number_lut.get(&current_word[i..]) {
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

fn word_to_number_lut() -> HashMap<&'static str, u32> {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    map
}
