use crate::data::*;
use rayon::prelude::*;

pub fn part_1(lines: &[String]) -> u32 {
    let rows: Vec<Vec<char>> = lines
        .par_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    let mut processed = vec![vec![false; rows[0].len()]; rows.len()];

    for (row_i, row) in rows.iter().enumerate() {
        for (cell_i, cell) in row.iter().enumerate() {
            if !cell.is_ascii_digit() || processed[row_i][cell_i] {
                continue;
            }

            let number = row
                .iter()
                .skip(cell_i)
                .take_while(|c| c.is_ascii_digit())
                .enumerate()
                .map(|(offset, c)| {
                    processed[row_i][cell_i + offset] = true;
                    c
                })
                .collect::<String>();

            let adjacent_to_symbol = (0..number.len()).any(|n| {
                DIRECTIONS.iter().any(|&(di, dj)| {
                    let (d_row_i, d_cell_i) = (
                        (row_i as i16 + di) as usize,
                        (cell_i as i16 + dj + n as i16) as usize,
                    );
                    d_row_i < rows.len() && d_cell_i < row.len() && {
                        let adjacent_cell = rows[d_row_i][d_cell_i];
                        adjacent_cell != '.' && !adjacent_cell.is_ascii_digit()
                    }
                })
            });

            if adjacent_to_symbol {
                sum += number.parse::<u32>().unwrap();
            }
        }
    }

    sum
}

pub fn part_2(_lines: &[String]) -> u32 {
    0
}
