use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse(path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        rows.push(line.chars().collect());
    }

    Ok(rows)
}

fn main() {
    let input = parse("example.txt").ok().unwrap();
    println!("{input:?}");
}
