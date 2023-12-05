use rayon::prelude::*;

pub fn part_1(lines: &[String]) -> usize {
    let seeds = extract_seeds(lines);
    let sections = split_at_blank_lines(&lines[2..]);
    let maps = sections_to_maps(&sections);
    calculate_lowest_location_part_1(&seeds, &maps)
}

pub fn part_2(lines: &[String]) -> usize {
    let seed_ranges = extract_seed_ranges(lines);
    let seeds = generate_seed_numbers(seed_ranges);
    let sections = split_at_blank_lines(&lines[2..]);
    let maps = sections_to_maps(&sections);
    calculate_lowest_location_part_2(seeds, &maps)
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

fn calculate_lowest_location_part_1(seeds: &Vec<usize>, maps: &Vec<Vec<Map>>) -> usize {
    let mut lowest_location = usize::MAX;

    for &seed in seeds {
        let mut current_seed = seed;
        for map in maps {
            current_seed = map_value_through_map(current_seed, map);
        }
        lowest_location = lowest_location.min(current_seed);
    }

    lowest_location
}

fn map_value_through_map(seed: usize, map: &Vec<Map>) -> usize {
    for current in map {
        if seed >= current.src_range_start && seed < current.src_range_start + current.range {
            return current.dst_range_start + (seed - current.src_range_start);
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

fn calculate_lowest_location_part_2(seeds: Vec<usize>, maps: &Vec<Vec<Map>>) -> usize {
    seeds
        .par_iter()
        .map(|&seed| calculate_lowest_location_part_1(&vec![seed], maps))
        .min()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Map {
    dst_range_start: usize,
    src_range_start: usize,
    range: usize,
}

impl Map {
    fn from_line(line: &str) -> Self {
        let mut split = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

        Self {
            dst_range_start: split.next().unwrap(),
            src_range_start: split.next().unwrap(),
            range: split.next().unwrap(),
        }
    }
}
