use common_2022::read_lines;
use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let lines: Vec<String> = read_lines("../!data/day3/real.txt")
        .expect("Failed to read lines from file")
        .collect();

    println!("Part One: {}", part_one(&lines));
}

// fn part_one(lines: &Vec<String>) -> u32 {
//     let mut count: u32 = 0;
//
//     for line in lines {
//         let split = split_string_in_half(line);
//         let char = intersecting_character(&split.0, &split.1).expect("No intersecting characters");
//         let priority = char_to_priority(char).expect("Not a letter");
//         count += u32::from(priority);
//     }
//
//     count
// }

fn part_one(lines: &Vec<String>) -> u32 {
    lines
        .par_iter()
        .map(split_string_in_half)
        .map(|(first, second)| {
            intersecting_character(&first, &second).expect("No intersecting characters")
        })
        .map(|char| char_to_priority(char).expect("Not a letter"))
        .map(u32::from)
        .sum()
}

fn split_string_in_half(s: &str) -> (String, String) {
    let char_count = s.chars().count();
    let mid = char_count / 2;

    let first_half: String = s.chars().take(mid).collect();
    let second_half: String = s.chars().skip(mid).collect();

    (first_half, second_half)
}

fn intersecting_character(s1: &str, s2: &str) -> Option<char> {
    let set1: HashSet<_> = s1.chars().collect();
    let set2: HashSet<_> = s2.chars().collect();

    set1.intersection(&set2).cloned().next()
}

fn char_to_priority(char: char) -> Option<u8> {
    match char {
        'a'..='z' => Some((char as u8 - 'a' as u8) + 1),
        'A'..='Z' => Some((char as u8 - 'A' as u8) + 27),
        _ => None,
    }
}
