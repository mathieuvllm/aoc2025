use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Inventory {
    ranges: Vec<(u64, u64)>,
    available: Vec<u64>,
}

impl Inventory {}

fn parse(path: &str) -> io::Result<Inventory> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut available: Vec<u64> = vec![];

    let mut is_range = true;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            is_range = false;
            continue;
        }

        if is_range {
            let mut bounds = line.split("-");
            ranges.push((
                bounds.next().unwrap().parse::<u64>().unwrap(),
                bounds.next().unwrap().parse::<u64>().unwrap(),
            ));
        } else {
            available.push(line.parse::<u64>().unwrap());
        }
    }

    Ok(Inventory { ranges, available })
}

fn main() {
    let input = parse("example.txt").ok().unwrap();
    println!("{:?}", input);
}
