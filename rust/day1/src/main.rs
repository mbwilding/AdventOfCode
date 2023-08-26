use common::read_lines;

fn main() {
    let lines = read_lines("../!data/day1/real.txt").expect("Failed to read lines from file");

    let sorted_calories = process_lines(lines);
    println!("Part One: {}", sorted_calories[0]);

    let top_3_sum: usize = sorted_calories.iter().take(3).sum();
    println!("Part Two: {}", top_3_sum);
}

fn process_lines<I: Iterator<Item = String>>(lines: I) -> Vec<usize> {
    let mut calories_per_group: Vec<usize> = Vec::new();
    let mut current_group_calories = 0;

    for line in lines {
        if line.is_empty() {
            calories_per_group.push(current_group_calories);
            current_group_calories = 0;
        } else if let Ok(calories) = line.parse::<usize>() {
            current_group_calories += calories;
        }
    }

    calories_per_group.sort_unstable_by(|a, b| b.cmp(a));

    calories_per_group
}
