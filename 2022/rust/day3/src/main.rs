use common_2022::read_lines;
use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let lines: Vec<String> = read_lines("2022/!data/day3/real.txt")
        .expect("Failed to read lines from file")
        .collect();

    println!("Part 1: {}", part_one(&lines));
}

fn part_one(lines: &Vec<String>) -> u32 {
    lines
        .par_iter()
        .map(|line| split_string_in_half(line))
        .map(|(first, second)| {
            intersecting_character(first, second).expect("No intersecting characters")
        })
        .map(|char| char_to_priority(char).expect("Not a letter"))
        .map(u32::from)
        .sum()
}

fn split_string_in_half(s: &str) -> (&str, &str) {
    let char_count = s.chars().count();
    let mid = char_count / 2;

    let mid_byte_index = s
        .char_indices()
        .nth(mid)
        .map_or(s.len(), |(index, _)| index);

    let first_half = &s[..mid_byte_index];
    let second_half = &s[mid_byte_index..];

    (first_half, second_half)
}

fn intersecting_character(s1: &str, s2: &str) -> Option<char> {
    let set1: HashSet<_> = s1.chars().collect();
    let set2: HashSet<_> = s2.chars().collect();

    set1.intersection(&set2).cloned().next()
}

fn char_to_priority(char: char) -> Option<u8> {
    match char {
        'a'..='z' => Some((char as u8 - b'a') + 1),
        'A'..='Z' => Some((char as u8 - b'A') + 27),
        _ => None,
    }
}
