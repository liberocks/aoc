use std::fs::File;
use std::io::{self, Read};

fn is_safe(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let initial_delta = levels[1] - levels[0];
    if initial_delta == 0 || initial_delta.abs() < 1 || initial_delta.abs() > 3 {
        return false;
    }

    let direction = initial_delta.signum();

    for i in 1..levels.len() - 1 {
        let delta = levels[i + 1] - levels[i];
        if delta.signum() != direction || delta == 0 || delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut safe = 0;
    for line in contents.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if is_safe(&levels) {
            safe += 1;
            continue;
        }

        let mut found = false;
        for i in 0..levels.len() {
            let mut modified_levels = levels.clone();
            modified_levels.remove(i);

            if is_safe(&modified_levels) {
                safe += 1;
                found = true;
                break;
            }
        }

        if found {
            continue;
        }
    }

    println!("Safe count: {}", safe);

    Ok(())
}