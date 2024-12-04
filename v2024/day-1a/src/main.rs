use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let splitted: Vec<i32> = line
            .split("   ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        lefts.push(splitted[0]);
        rights.push(splitted[1]);
    }

    lefts.sort();
    rights.sort();

    let mut total_distance = 0;
    for (left, right) in lefts.iter().zip(rights.iter()) {
        total_distance += (right - left).abs();
    }

    println!("Total distance: {}", total_distance);

    Ok(())
}
