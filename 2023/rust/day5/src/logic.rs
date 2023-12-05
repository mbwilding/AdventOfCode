use rayon::prelude::*;

pub fn part_1(lines: &[String]) -> usize {
    let seeds = extract_seeds(lines);
    let sections = split_at_blank_lines(&lines[2..]);
    let maps = sections_to_maps(&sections);
    calculate_lowest_location(&seeds, &maps)
}

pub fn part_2(lines: &[String]) -> usize {
    let seed_ranges = extract_seed_ranges(lines);
    let seeds = generate_seed_numbers(seed_ranges);
    let sections = split_at_blank_lines(&lines[2..]);
    let maps = sections_to_maps(&sections);
    calculate_lowest_location(&seeds, &maps)
}

fn extract_seeds(lines: &[String]) -> Vec<usize> {
    lines[0]
        .split_once("seeds: ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn split_at_blank_lines(lines: &[String]) -> Vec<Vec<&str>> {
    let mut sections = Vec::new();
    let mut current_chunk = Vec::new();

    for line in lines {
        if line.is_empty() {
            if !current_chunk.is_empty() {
                sections.push(current_chunk);
                current_chunk = Vec::new();
            }
        } else {
            current_chunk.push(line.as_str());
        }
    }

    if !current_chunk.is_empty() {
        sections.push(current_chunk);
    }

    sections
}

fn sections_to_maps(sections: &[Vec<&str>]) -> Vec<Vec<Map>> {
    sections
        .par_iter()
        .map(|section| {
            section
                .iter()
                .skip(1)
                .map(|line| Map::from_line(line))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn calculate_lowest_location(seeds: &[usize], maps: &[Vec<Map>]) -> usize {
    seeds
        .par_iter()
        .map(|&seed| {
            maps.iter().fold(seed, |current_seed, map| {
                map_value_through_map(current_seed, map)
            })
        })
        .min()
        .unwrap_or(usize::MAX)
}

fn map_value_through_map(seed: usize, map: &Vec<Map>) -> usize {
    for current in map {
        if seed >= current.source && seed < current.source + current.range {
            return current.destination + (seed - current.source);
        }
    }

    seed
}

fn extract_seed_ranges(lines: &[String]) -> Vec<(usize, usize)> {
    lines[0]
        .split_once("seeds: ")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0].parse::<usize>().unwrap();
            let length = chunk[1].parse::<usize>().unwrap();
            (start, length)
        })
        .collect()
}

fn generate_seed_numbers(ranges: Vec<(usize, usize)>) -> Vec<usize> {
    let mut seeds = Vec::new();
    for (start, length) in ranges {
        seeds.extend(start..start + length);
    }
    seeds
}

struct Map {
    destination: usize,
    source: usize,
    range: usize,
}

impl Map {
    fn from_line(line: &str) -> Self {
        let mut split = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

        Self {
            destination: split.next().unwrap(),
            source: split.next().unwrap(),
            range: split.next().unwrap(),
        }
    }
}
