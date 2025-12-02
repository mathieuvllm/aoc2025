fn is_valid(id: u128) -> bool {
    let id = id.to_string();
    if !(id.len().is_multiple_of(2)) {
        return true;
    }

    let mid = id.len() / 2;
    let parts = (&id[..mid], &id[mid..]);

    parts.0 != parts.1
}

fn invalid_in_range(first: u128, last: u128) -> Option<Vec<u128>> {
    let mut invalid = vec![];

    for id in first..=last {
        if !(is_valid(id)) {
            invalid.push(id);
        }
    }

    match invalid.is_empty() {
        true => None,
        false => Some(invalid),
    }
}

fn parse(path: &str) -> std::io::Result<Vec<(u128, u128)>> {
    let file = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = file.lines().collect();
    let line = lines[0];

    Ok(line
        .split(',')
        .map(|pair| {
            let mut parts = pair.split('-');
            let a = parts.next().unwrap().parse::<u128>().unwrap();
            let b = parts.next().unwrap().parse::<u128>().unwrap();
            (a, b)
        })
        .collect())
}

fn main() {
    let ranges = parse("input.txt").unwrap();

    let mut total: u128 = 0;
    for &(first, last) in ranges.iter() {
        if let Some(invalid) = invalid_in_range(first, last) {
            let sum: u128 = invalid.iter().sum();
            total += sum;
        }
    }

    println!("{total}");
}
