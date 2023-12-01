use anyhow::Result;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run<F1, F2, T1, T2>(day: i32, file: &str, part_1: F1, part_2: F2) -> Result<()>
where
    F1: Fn(&[String]) -> T1,
    F2: Fn(&[String]) -> T2,
    T1: Display,
    T2: Display,
{
    let path = format!("2023/!data/day{}/{}.txt", day, file);
    let lines = read_file_lines(&path)?;

    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));

    Ok(())
}

pub fn test<F, T>(day: i32, file: &str, part: F, expected: T) -> Result<()>
    where
        F: Fn(&[String]) -> T, T: PartialEq + Debug
{
    let path = format!("../../!data/day{}/{}.txt", day, file);
    let lines = read_file_lines(&path)?;
    let actual = part(&lines);
    assert_eq!(expected, actual);

    Ok(())
}

fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Result<Vec<_>, io::Error>>()?;
    Ok(lines)
}
