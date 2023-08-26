fn main() {
    let reader = common::read_lines("../!data/day1/real.txt").expect("Invalid path");

    for line in reader {
        match line {
            Ok(l) => {
                println!("{l}")
            },
            Err(e) => eprintln!("Error reading line: {e}"),
        }
    }
}
