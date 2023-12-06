pub fn part_1(lines: &[String]) -> usize {
    let times = extract_numbers(&lines[0]);
    let distances = extract_numbers(&lines[1]);

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| calculate_ways_to_win(time, distance))
        .product()
}

pub fn part_2(lines: &[String]) -> usize {
    let time = extract_number(&lines[0]);
    let distance = extract_number(&lines[1]);

    calculate_ways_to_win(time, distance)
}

fn extract_numbers(line: &str) -> Vec<usize> {
    line.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn extract_number(line: &str) -> usize {
    line.split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap()
}

fn calculate_ways_to_win(time: usize, distance: usize) -> usize {
    (0..time)
        .filter(|hold| hold * (time - hold) > distance)
        .count()
}
