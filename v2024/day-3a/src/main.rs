use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum: u64 = 0;

    for line in contents.lines() {
        for caps in re.captures_iter(line) {
            let a = caps[1].parse::<u64>().unwrap();
            let b = caps[2].parse::<u64>().unwrap();
            sum += a * b;
        }
    }

    println!("Sum of all correct patterns: {}", sum);

    Ok(())
}