use anyhow::Result;
use std::collections::HashMap;
use common_2023::read_lines;

fn main() -> Result<()> {
    let lines = read_lines("2023/!data/day1/real.txt")?;

    let part_1 = process_lines_part_1(&lines);
    println!("Part 1: {}", part_1);

    let part_2 = process_lines_part_2(&lines);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn process_lines_part_1(lines: &Vec<String>) -> u32 {
    let mut container_numbers: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let mut characters: Vec<char> = Vec::new();
        let mut numbers: Vec<u32> = Vec::new();

        for character in line.chars() {
            characters.push(character);
            if let Some(value) = character.to_digit(10) {
                numbers.push(value)
            }
        }

        container_numbers.push(numbers)
    }

    let mut sum = 0;
    for line in container_numbers {
        let first = line.first().unwrap();
        let last = line.last().unwrap();
        let concatenated_str = first.to_string() + &last.to_string();
        let result: u32 = concatenated_str.parse().unwrap();
        sum += result;
    }

    return sum;
}

fn process_lines_part_2(lines: &Vec<String>) -> i32 {
    let number_map = word_to_number_lut();
    let mut sum = 0;

    for line in lines {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut current_word = String::new();

        for c in line.chars() {
            if c.is_alphabetic() {
                current_word.push(c);
                for i in 0..current_word.len() {
                    if let Some(&number) = number_map.get(&current_word[i..]) {
                        if first_digit == 0 {
                            first_digit = number;
                        }
                        last_digit = number;
                        break;
                    }
                }
            } else if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as i32;
                if first_digit == 0 {
                    first_digit = digit;
                }
                last_digit = digit;
                current_word.clear();
            } else {
                current_word.clear();
            }
        }

        let concatenated_str = first_digit.to_string() + &last_digit.to_string();
        let value = &concatenated_str.parse().unwrap();
        sum += value;
    }

    sum
}

fn word_to_number_lut() -> HashMap<&'static str, i32> {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() -> Result<()> {
        let lines = read_lines("../../!data/day1/mock1.txt")?;
        let result = process_lines_part_1(&lines);
        assert_eq!(result, 142);

        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let lines = read_lines("../../!data/day1/mock2.txt")?;
        let result = process_lines_part_2(&lines);
        assert_eq!(result, 281);

        Ok(())
    }
}
