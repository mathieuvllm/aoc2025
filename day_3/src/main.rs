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

fn max_joltage_2(bank: Vec<u8>) -> u64 {
    let mut indexes = vec![];
    indexes.push(max_index(bank[..bank.len() - 11].to_vec()));

    for n in (0usize..=10).rev() {
        let last = *indexes.last().unwrap();
        indexes.push(max_index(bank[last + 1..bank.len() - n].to_vec()) + last + 1);
    }

    let mut mult: u64 = 1;
    let mut sum: u64 = 0;

    for i in (0usize..=11).rev() {
        sum += (bank[indexes[i]] as u64) * mult;
        mult *= 10;
    }

    sum
}

fn main() {
    let input = parse("input.txt").ok().unwrap();
    let sum: u64 = input.iter().map(|v| max_joltage_2(v.clone())).sum();
    println!("{sum}");
}
