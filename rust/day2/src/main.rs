mod enums;

use crate::enums::{Part, Move};
use common::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("../!data/day2/real.txt")
        .expect("Failed to read lines from file")
        .collect();

    println!("Part One: {}", get_total_score(&lines, Part::PartOne));
    println!("Part Two: {}", get_total_score(&lines, Part::PartTwo));
}

fn get_total_score(lines: &[String], part: Part) -> i32 {
    lines
        .iter()
        .filter_map(|line| {
            let chars: Vec<char> = line.chars().collect();
            if chars.len() == 3 {
                let opponent_move = Move::part_one(chars[0])?;
                let my_move = match part {
                    Part::PartOne => Move::part_one(chars[2])?,
                    Part::PartTwo => Move::part_two(chars[2], &opponent_move)?,
                };
                Some((opponent_move, my_move))
            } else {
                None
            }
        })
        .map(|(opponent, my_move)| get_score(&opponent, &my_move))
        .sum()
}

fn get_score(opponent: &Move, my_move: &Move) -> i32 {
    let outcome = if opponent == my_move {
        3 // Draw
    } else {
        match (opponent, my_move) {
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => 6, // Win
            _ => 0, // Lose
        }
    };

    outcome + my_move.move_score()
}
