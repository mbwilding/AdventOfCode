use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>> where P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
