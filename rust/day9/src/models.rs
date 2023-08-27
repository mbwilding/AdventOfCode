#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Unknown direction character: {}", c),
        }
    }
}

#[derive(Debug)]
pub struct Move {
    direction: Direction,
    moves: u8,
}

impl Move {
    pub fn new(line: String) -> Self {
        Self {
            direction: Direction::from_char(line.chars().nth(0).unwrap()),
            moves: line[2..].trim().parse().unwrap(),
        }
    }
}
