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

fn max_index(bank: Vec<u8>) -> usize {
    bank.iter()
        .enumerate()
        .reduce(|(index1, value1), (index2, value2)| {
            if value2 > value1 {
                (index2, value2)
            } else {
                (index1, value1)
            }
        })
        .unwrap()
        .0
}

fn max_joltage(bank: Vec<u8>) -> u32 {
    let a = max_index(bank[..bank.len() - 1].to_vec());
    let b = max_index(bank[a + 1..].to_vec()) + a + 1;

    (bank[a] * 10 + bank[b]) as u32
}

fn main() {
    let input = parse("input.txt").ok().unwrap();
    let sum: u32 = input.iter().map(|v| max_joltage(v.clone())).sum();
    println!("{sum}");
}
