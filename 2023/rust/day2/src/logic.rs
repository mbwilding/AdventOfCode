use crate::data::*;
use std::collections::HashMap;

pub fn part_1(lines: &[String]) -> u32 {
    lines
        .iter()
        .filter_map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let game_id: u32 = parts[0].split_whitespace().last().unwrap().parse().unwrap();

            parts[1]
                .split("; ")
                .all(|set| {
                    set.split(", ").all(|c| {
                        let vec = c.split_whitespace().collect::<Vec<_>>();
                        let count = vec[0].parse::<u32>().unwrap();
                        let color = vec[1];
                        count <= *MAX_CUBES_LUT.get(color).unwrap()
                    })
                })
                .then_some(game_id)
        })
        .sum()
}

pub fn part_2(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|line| {
            line.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .fold(HashMap::new(), |mut min_cubes, set| {
                    set.split(", ").for_each(|cube| {
                        let parts = cube.split_whitespace().collect::<Vec<_>>();
                        let count = parts[0].parse::<u32>().unwrap();
                        let color = parts[1];
                        min_cubes
                            .entry(color)
                            .and_modify(|current| *current = u32::max(*current, count))
                            .or_insert(count);
                    });
                    min_cubes
                })
                .values()
                .product::<u32>()
        })
        .sum()
}
