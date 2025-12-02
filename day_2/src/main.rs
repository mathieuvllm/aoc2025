fn is_valid(id: u64) -> bool {
    let id = id.to_string();
    if !(id.len().is_multiple_of(2)) {
        return true;
    }

    let mid = id.len() / 2;
    let parts = (&id[..mid], &id[mid..]);

    parts.0 != parts.1
}

fn invalid_in_range(first: u64, last: u64) -> Option<Vec<u64>> {
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

fn main() {
    let ret = invalid_in_range(11, 22);
    let mut total: u64 = 0;
    if let Some(invalid) = ret {
        let sum: u64 = invalid.iter().sum();
        total += sum;
    }

    println!("{total}");
}
