fn is_valid(id: u128) -> bool {
    let id = id.to_string();
    if !(id.len().is_multiple_of(2)) {
        return true;
    }

    let mid = id.len() / 2;
    let parts = (&id[..mid], &id[mid..]);

    parts.0 != parts.1
}

fn is_valid_2(id: u128) -> bool {
    let id = id.to_string();
    let length = id.len();

    for n in 2..=length {
        if id.len().is_multiple_of(n) {
            let slice_size = length / n;
            if id
                .as_bytes()
                .chunks(slice_size)
                .all(|chunk| chunk == &id.as_bytes()[..slice_size])
            {
                return false;
            }
        }
    }

    true
}

fn invalid_in_range(first: u128, last: u128, part: u8) -> Option<Vec<u128>> {
    let mut invalid = vec![];

    for id in first..=last {
        match part {
            1 => {
                if !(is_valid(id)) {
                    invalid.push(id);
                }
            }
            2 => {
                if !(is_valid_2(id)) {
                    invalid.push(id);
                }
            }
            _ => return None,
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
        if let Some(invalid) = invalid_in_range(first, last, 2) {
            let sum: u128 = invalid.iter().sum();
            total += sum;
        }
    }

    println!("{total}");
}
