use rayon::prelude::*;
use std::str::SplitWhitespace;

pub fn part_1(lines: &[String]) -> usize {
    let seeds = extract_seeds(lines);

    execute(&lines, &seeds)
}

pub fn part_2(lines: &[String]) -> usize {
    let seeds = generate_seeds(lines);

    execute(&lines, &seeds)
}

fn execute(lines: &&[String], seeds: &[usize]) -> usize {
    let sections = split_at_blank_lines(&lines[2..]);
    let maps = sections_to_maps(&sections);

    calculate_lowest_location(seeds, &maps)
}

fn extract_seeds(lines: &[String]) -> Vec<usize> {
    split_seeds_whitespace(lines)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
}

fn generate_seeds(lines: &[String]) -> Vec<usize> {
    split_seeds_whitespace(lines)
        .collect::<Vec<_>>()
        .chunks(2)
        .fold(Vec::new(), |mut seeds, chunk| {
            let start = chunk[0].parse::<usize>().unwrap();
            let length = chunk[1].parse::<usize>().unwrap();
            seeds.extend(start..start + length);

            seeds
        })
}

fn split_seeds_whitespace(lines: &[String]) -> SplitWhitespace {
    lines[0].split_once(':').unwrap().1.split_whitespace()
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
                .collect()
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
        .unwrap()
}

fn map_value_through_map(seed: usize, map: &Vec<Map>) -> usize {
    for current in map {
        if seed >= current.source && seed < current.source + current.range {
            return current.destination + (seed - current.source);
        }
    }

    seed
}

struct Map {
    destination: usize,
    source: usize,
    range: usize,
}

impl Map {
    fn from_line(line: &str) -> Self {
        let mut split = line.split_whitespace().map(|s| s.parse().unwrap());

        Self {
            destination: split.next().unwrap(),
            source: split.next().unwrap(),
            range: split.next().unwrap(),
        }
    }
}
