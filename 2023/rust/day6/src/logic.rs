pub fn part_1(lines: &[String]) -> usize {
    let times = extract_numbers(&lines[0]);
    let distances = extract_numbers(&lines[1]);

    calculate_ways_to_win_and_multiply(&times, &distances)
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
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn extract_number(line: &str) -> usize {
    line.split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<usize>()
        .unwrap()
}

fn calculate_ways_to_win_and_multiply(times: &[usize], distances: &[usize]) -> usize {
    times
        .iter()
        .zip(distances)
        .map(|(&time, &distance)| calculate_ways_to_win(time, distance))
        .collect::<Vec<usize>>()
        .iter()
        .product()
}

fn calculate_ways_to_win(time: usize, distance: usize) -> usize {
    (0..time)
        .filter(|hold| hold * (time - hold) > distance)
        .count()
}
