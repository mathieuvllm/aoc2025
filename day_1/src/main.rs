use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn rotate(current: i32, distance: i32) -> (i32, u32) {
    let mut sum = current + distance;
    let mut zeroes: u32 = 0;
    while sum < 0 {
        sum += 100;
        if current != 0 || sum < 0 {
            zeroes += 1;
        }
    }
    while sum > 99 {
        sum -= 100;
        if sum != 0 {
            zeroes += 1;
        }
    }

    (sum, zeroes)
}

fn parse(path: &str) -> io::Result<Vec<i32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut distances = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut chars = line.chars();
        let first = chars.next().unwrap();
        let rest: String = chars.collect();
        let mut value: i32 = rest.parse().unwrap();

        if first == 'L' {
            value = -value;
        }

        distances.push(value);
    }

    Ok(distances)
}

fn main() {
    let input = parse("input.txt").ok().unwrap();
    let mut password: u32 = 0;

    let mut current = 50;
    for &distance in input.iter() {
        let ret = rotate(current, distance);
        current = ret.0;
        if current == 0 {
            password += 1;
        }
        password += ret.1;
    }
    println!("{password}");
}
