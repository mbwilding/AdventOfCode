use crate::data::*;
use std::collections::HashSet;

pub fn part_1(lines: &[String]) -> u32 {
    let rows = get_rows_of_chars(lines);

    let mut sum = 0;

    for (row_i, row_chars) in rows.iter().enumerate() {
        let mut col_i = 0;
        while col_i < row_chars.len() {
            let cell_char = row_chars[col_i];

            if !cell_char.is_ascii_digit() {
                col_i += 1;
                continue;
            }

            let number_str = extract_number(&rows, row_i, col_i);

            if adjacent_to_symbol(&rows, row_i, row_chars, col_i, &number_str) {
                sum += number_str.parse::<u32>().unwrap();
            }

            col_i += number_str.len();
        }
    }

    sum
}

pub fn part_2(lines: &[String]) -> u32 {
    let rows = get_rows_of_chars(lines);

    let mut sum = 0;

    for (row_i, row) in rows.iter().enumerate() {
        for (col_i, &cell_char) in row.iter().enumerate() {
            if cell_char == '*' {
                let mut part_numbers: HashSet<u32> = HashSet::new();

                for &(di, dj) in DIRECTIONS.iter() {
                    let (adj_row, adj_col) =
                        ((row_i as i16 + di) as usize, (col_i as i16 + dj) as usize);

                    if adj_row < rows.len() && adj_col < rows[0].len() {
                        let adj_cell = rows[adj_row][adj_col];
                        if adj_cell.is_ascii_digit() {
                            let number_str = extract_number(&rows, adj_row, adj_col);
                            if let Ok(num) = number_str.parse::<u32>() {
                                part_numbers.insert(num);
                            }
                        }
                    }
                }

                if part_numbers.len() == 2 {
                    let mut iter = part_numbers.into_iter();
                    sum += iter.next().unwrap() * iter.next().unwrap();
                }
            }
        }
    }

    sum
}

fn get_rows_of_chars(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn adjacent_to_symbol(
    rows: &Vec<Vec<char>>,
    row_i: usize,
    row_chars: &Vec<char>,
    col_i: usize,
    number_str: &String,
) -> bool {
    (0..number_str.len()).any(|n| {
        DIRECTIONS.iter().any(|&(di, dj)| {
            let (d_row_i, d_cell_i) = (
                (row_i as i16 + di) as usize,
                (col_i as i16 + dj + n as i16) as usize,
            );
            d_row_i < rows.len() && d_cell_i < row_chars.len() && {
                let adjacent_cell = rows[d_row_i][d_cell_i];
                adjacent_cell != '.' && !adjacent_cell.is_ascii_digit()
            }
        })
    })
}

fn extract_number(rows: &[Vec<char>], row: usize, col: usize) -> String {
    let start = rows[row]
        .iter()
        .take(col)
        .rposition(|&c| !c.is_ascii_digit())
        .map_or(0, |pos| pos + 1);

    rows[row][start..]
        .iter()
        .take_while(|&&c| c.is_ascii_digit())
        .collect()
}
