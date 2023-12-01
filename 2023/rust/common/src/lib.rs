use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(path: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Result<Vec<_>, io::Error>>()?;
    Ok(lines)
}
