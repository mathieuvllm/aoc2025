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

fn neighbors(rows: Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u8 {
    if rows[row_idx][col_idx] != '@' {
        return 9;
    }
    let mut count: u8 = 0;
    let row_length = rows[0].len();
    let col_length = rows.len();

    if row_idx != 0 {
        if rows[row_idx - 1][col_idx] == '@' {
            count += 1;
        }
        if col_idx != 0 && rows[row_idx - 1][col_idx - 1] == '@' {
            count += 1;
        }
        if col_idx != row_length - 1 && rows[row_idx - 1][col_idx + 1] == '@' {
            count += 1;
        }
    }
    if row_idx != col_length - 1 {
        if rows[row_idx + 1][col_idx] == '@' {
            count += 1;
        }
        if col_idx != 0 && rows[row_idx + 1][col_idx - 1] == '@' {
            count += 1;
        }
        if col_idx != row_length - 1 && rows[row_idx + 1][col_idx + 1] == '@' {
            count += 1;
        }
    }
    if col_idx != 0 && rows[row_idx][col_idx - 1] == '@' {
        count += 1;
    }
    if col_idx != row_length - 1 && rows[row_idx][col_idx + 1] == '@' {
        count += 1;
    }

    count
}

// used for debugging
fn print_grid(grid: Vec<Vec<char>>) {
    grid.iter().for_each(|v| {
        v.iter().for_each(|c| print!("{c}"));
        println!();
    });
}

fn main() {
    let input = parse("input.txt").ok().unwrap();
    let mut new: Vec<Vec<char>> = input.clone();
    let col_len = input.len();
    let row_len = input[0].len();
    let mut count = 0;

    for row_idx in 0..col_len {
        for col_idx in 0..row_len {
            if neighbors(input.clone(), row_idx, col_idx) < 4 {
                count += 1;
                new[row_idx][col_idx] = 'x';
            }
        }
    }

    println!("{count}");
}
