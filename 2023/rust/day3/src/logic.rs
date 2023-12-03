use crate::data::*;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn part_1(lines: &[String]) -> u32 {
    let rows: Vec<Vec<char>> = lines
        .par_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    let mut processed = vec![vec![false; rows[0].len()]; rows.len()];

    for (row_i, row_chars) in rows.iter().enumerate() {
        for (col_i, &cell_char) in row_chars.iter().enumerate() {
            if !cell_char.is_ascii_digit() || processed[row_i][col_i] {
                continue;
            }

            let number_str = extract_number(&rows, row_i, col_i);

            for offset in 0..number_str.len() {
                processed[row_i][col_i + offset] = true;
            }

            let adjacent_to_symbol = (0..number_str.len()).any(|n| {
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
            });

            if adjacent_to_symbol {
                sum += number_str.parse::<u32>().unwrap();
            }
        }
    }

    sum
}

pub fn part_2(lines: &[String]) -> u64 {
    let rows: Vec<Vec<char>> = lines
        .par_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;

    for (row_i, row) in rows.iter().enumerate() {
        for (col_i, &cell_char) in row.iter().enumerate() {
            if cell_char == '*' {
                let mut part_numbers: HashSet<u64> = HashSet::new();

                for &(di, dj) in DIRECTIONS.iter() {
                    let (adj_row, adj_col) =
                        ((row_i as i16 + di) as usize, (col_i as i16 + dj) as usize);

                    if adj_row < rows.len() && adj_col < rows[0].len() {
                        let adj_cell = rows[adj_row][adj_col];
                        if adj_cell.is_ascii_digit() {
                            let number_str = extract_number(&rows, adj_row, adj_col);
                            if let Ok(num) = number_str.parse::<u64>() {
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

fn extract_number(rows: &[Vec<char>], row: usize, col: usize) -> String {
    let mut number_str = String::new();
    let mut c = col;

    while c > 0 && rows[row][c - 1].is_ascii_digit() {
        c -= 1;
    }

    while c < rows[row].len() && rows[row][c].is_ascii_digit() {
        number_str.push(rows[row][c]);
        c += 1;
    }

    number_str
}
