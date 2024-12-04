use std::fs::File;
use std::io::{self, Read};

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

        if levels.len() < 2 {
            continue;
        }

        let mut is_safe = true;
        let initial_delta = levels[1] - levels[0];
        let direction = initial_delta.signum();

        if initial_delta == 0 || initial_delta.abs() < 1 || initial_delta.abs() > 3 {
            continue;
        }

        for i in 1..levels.len() - 1 {
            let delta = levels[i + 1] - levels[i];

            if delta.signum() != direction || delta == 0 || delta.abs() < 1 || delta.abs() > 3 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe += 1;
        }
    }

    println!("Safe count: {}", safe);

    Ok(())
}