use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn rotate(current: i32, distance: i32) -> i32 {
    let mut sum = current + distance;
    while sum < 0 {
        sum += 100;
    }
    while sum > 99 {
        sum -= 100;
    }
    sum
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
        current = rotate(current, distance);
        if current == 0 {
            password += 1;
        }
    }
    println!("{password}");
}
