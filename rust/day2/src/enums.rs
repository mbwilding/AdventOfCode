pub enum Mode {
    PartOne,
    PartTwo,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Move {
    Rock,     // X or A
    Paper,    // Y or B
    Scissors, // Z or C
}

impl Move {
    pub fn part_one(ch: char) -> Option<Move> {
        match ch {
            'X' | 'A' => Some(Move::Rock),
            'Y' | 'B' => Some(Move::Paper),
            'Z' | 'C' => Some(Move::Scissors),
            _ => None,
        }
    }

    pub fn move_score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn part_two(ch: char, opponent: &Move) -> Option<Move> {
        match ch {
            'X' => match opponent {
                Move::Rock => Some(Move::Scissors),
                Move::Paper => Some(Move::Rock),
                Move::Scissors => Some(Move::Paper),
            },
            'Y' => Some(*opponent),
            'Z' => match opponent {
                Move::Rock => Some(Move::Paper),
                Move::Paper => Some(Move::Scissors),
                Move::Scissors => Some(Move::Rock),
            },
            _ => None,
        }
    }
}
