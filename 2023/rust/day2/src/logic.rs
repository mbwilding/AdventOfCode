use rayon::prelude::*;
use std::collections::HashMap;

pub fn part_1(lines: &[String]) -> i32 {
    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    lines
        .par_iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let game_id: i32 = parts[0].split_whitespace().last().unwrap().parse().unwrap();
            let mut sets = parts[1].split("; ");

            if sets.all(|set| {
                set.split(", ")
                    .map(|c| c.split_whitespace().collect::<Vec<&str>>())
                    .all(|vec| {
                        let count: i32 = vec[0].parse().unwrap();
                        let color = vec[1];
                        count <= *max_cubes.get(color).unwrap()
                    })
            }) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_2(lines: &[String]) -> u32 {
    lines
        .par_iter()
        .map(|game| {
            let sets = game.split(": ").nth(1).unwrap().split("; ");
            let mut min_cubes: HashMap<&str, u32> = HashMap::new();

            for set in sets {
                set.split(", ").for_each(|cube| {
                    let parts: Vec<&str> = cube.split_whitespace().collect();
                    let count = parts[0].parse::<u32>().unwrap();
                    let color = parts[1];
                    min_cubes
                        .entry(color)
                        .and_modify(|e| *e = u32::max(*e, count))
                        .or_insert(count);
                });
            }

            min_cubes.values().product::<u32>()
        })
        .sum()
}
