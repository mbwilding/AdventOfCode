use std::collections::HashSet;

pub fn part_1(lines: &[String]) -> u32 {
    let grid = get_rows_of_chars(lines);

    let mut sum = 0;

    for (current_row_index, current_row) in grid.iter().enumerate() {
        let mut current_col_index = 0;
        while current_col_index < current_row.len() {
            let current_cell = current_row[current_col_index];

            if !current_cell.is_ascii_digit() {
                current_col_index += 1;
                continue;
            }

            let number_string = extract_number(&grid, current_row_index, current_col_index);

            if is_adjacent_to_symbol(
                &grid,
                current_row_index,
                current_row,
                current_col_index,
                &number_string,
            ) {
                sum += number_string.parse::<u32>().unwrap();
            }

            current_col_index += number_string.len();
        }
    }

    sum
}

pub fn part_2(lines: &[String]) -> u32 {
    let grid = get_rows_of_chars(lines);
    let mut sum = 0;

    for (current_row_index, current_row) in grid.iter().enumerate() {
        for (current_col_index, &current_cell) in current_row.iter().enumerate() {
            if current_cell == '*' {
                if let Some(product) =
                    product_from_adjacent_cells(&grid, current_row_index, current_col_index)
                {
                    sum += product;
                }
            }
        }
    }

    sum
}

fn get_rows_of_chars(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn is_adjacent_to_symbol(
    grid: &Vec<Vec<char>>,
    current_row_index: usize,
    current_row: &Vec<char>,
    current_col_index: usize,
    number_as_string: &str,
) -> bool {
    (0..number_as_string.len()).any(|number_char_index| {
        DIRECTIONS.iter().any(|&(row_offset, col_offset)| {
            let (adjacent_row_index, adjacent_col_index) = (
                (current_row_index as i16 + row_offset) as usize,
                (current_col_index as i16 + col_offset + number_char_index as i16) as usize,
            );
            adjacent_row_index < grid.len() && adjacent_col_index < current_row.len() && {
                let adjacent_cell = grid[adjacent_row_index][adjacent_col_index];
                adjacent_cell != '.' && !adjacent_cell.is_ascii_digit()
            }
        })
    })
}

fn product_from_adjacent_cells(
    grid: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> Option<u32> {
    let mut adjacent_numbers: HashSet<u32> = HashSet::new();

    for &(row_offset, col_offset) in DIRECTIONS.iter() {
        let (adjacent_row_index, adjacent_col_index) = (
            (row_index as i16 + row_offset) as usize,
            (col_index as i16 + col_offset) as usize,
        );

        if adjacent_row_index < grid.len() && adjacent_col_index < grid[0].len() {
            let adjacent_cell = grid[adjacent_row_index][adjacent_col_index];
            if adjacent_cell.is_ascii_digit() {
                let number_string = extract_number(grid, adjacent_row_index, adjacent_col_index);
                if let Ok(number) = number_string.parse() {
                    adjacent_numbers.insert(number);
                }
            }
        }
    }

    if adjacent_numbers.len() == 2 {
        let mut number_iter = adjacent_numbers.into_iter();
        Some(number_iter.next().unwrap() * number_iter.next().unwrap())
    } else {
        None
    }
}

fn extract_number(rows: &[Vec<char>], row: usize, col: usize) -> String {
    let start = rows[row]
        .iter()
        .take(col)
        .rposition(|c| !c.is_ascii_digit())
        .map_or(0, |pos| pos + 1);

    rows[row][start..]
        .iter()
        .take_while(|c| c.is_ascii_digit())
        .collect()
}

pub const DIRECTIONS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
