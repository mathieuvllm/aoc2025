use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse(path: &str) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut banks: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        banks.push(line.bytes().map(|b| b - b'0').collect());
    }

    Ok(banks)
}

fn main() {
    let input = parse("example.txt").ok().unwrap();
    println!("{input:?}");
}
