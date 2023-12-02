use crate::data::*;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn part_1(lines: &[String]) -> u32 {
    let max_cubes = max_cubes_lut();

    lines
        .par_iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let game_id: u32 = parts[0].split_whitespace().last().unwrap().parse().unwrap();

            parts[1]
                .split("; ")
                .all(|set| {
                    set.split(", ").all(|c| {
                        let vec: Vec<&str> = c.split_whitespace().collect();
                        let count: u32 = vec[0].parse().unwrap();
                        let color = vec[1];
                        count <= *max_cubes.get(color).unwrap()
                    })
                })
                .then_some(game_id)
        })
        .sum()
}

pub fn part_2(lines: &[String]) -> u32 {
    lines
        .par_iter()
        .map(|game| {
            game.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .fold(HashMap::new(), |mut min_cubes, set| {
                    set.split(", ").for_each(|cube| {
                        let parts: Vec<&str> = cube.split_whitespace().collect();
                        let count = parts[0].parse::<u32>().unwrap();
                        let color = parts[1];
                        min_cubes
                            .entry(color)
                            .and_modify(|e| *e = u32::max(*e, count))
                            .or_insert(count);
                    });
                    min_cubes
                })
                .values()
                .product::<u32>()
        })
        .sum()
}
