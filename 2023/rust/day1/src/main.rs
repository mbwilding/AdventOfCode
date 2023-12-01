use std::collections::HashMap;
use common_2023::read_lines;

fn main() {
    let lines = read_lines("2023/!data/day1/real.txt").expect("Failed to read lines from file");

    let numbers = process_lines_part_1(&lines);
    let mut sum = 0;
    for line in numbers {
        let first = line.first().unwrap_or(&0);
        let last = line.last().unwrap_or(&1);
        let concatenated_str = first.to_string() + &last.to_string();
        let result: u32 = concatenated_str.parse().unwrap();
        sum += result;
    }
    println!("Part One: {:?}", sum);

    let part_2 = process_lines_part_2(&lines);
    println!("Part Two: {:?}", part_2);
}

fn process_lines_part_1(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut container_numbers: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        if line.is_empty() {

        } else {
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
    }

    return container_numbers;
}

fn process_lines_part_2(lines: &Vec<String>) -> i32 {
    let mut sum = 0;

    for line in lines {
        let value = find_first_last_digit(line);
        sum += value;
    }

    sum
}

fn find_first_last_digit(line: &str) -> i32 {
    let number_map = create_number_map();
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut current_word = String::new();

    for c in line.chars() {
        if c.is_alphabetic() {
            current_word.push(c);
            for i in 0..current_word.len() {
                if let Some(&number) = number_map.get(current_word[i..].to_lowercase().as_str()) {
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
    concatenated_str.parse().unwrap()
}

fn create_number_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);
    map.insert("four".to_string(), 4);
    map.insert("five".to_string(), 5);
    map.insert("six".to_string(), 6);
    map.insert("seven".to_string(), 7);
    map.insert("eight".to_string(), 8);
    map.insert("nine".to_string(), 9);

    map
}
