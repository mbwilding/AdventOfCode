use crate::data::*;
use rayon::prelude::*;

pub fn part_1(lines: &[String]) -> u32 {
    let rows: Vec<Vec<char>> = lines
        .par_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    let mut processed = vec![vec![false; rows[0].len()]; rows.len()];

    for (i, row) in rows.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if !cell.is_ascii_digit() || processed[i][j] {
                continue;
            }

            let number = row
                .iter()
                .skip(j)
                .take_while(|c| c.is_ascii_digit())
                .enumerate()
                .map(|(offset, c)| {
                    processed[i][j + offset] = true;
                    c
                })
                .collect::<String>();

            let adjacent_to_symbol = (0..number.len()).any(|n| {
                DIRECTIONS.iter().any(|&(di, dj)| {
                    let (new_i, new_j) = (
                        (i as i16 + di) as usize,
                        (j as i16 + dj + n as i16) as usize,
                    );
                    new_i < rows.len() && new_j < row.len() && {
                        let adjacent_cell = rows[new_i][new_j];
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
