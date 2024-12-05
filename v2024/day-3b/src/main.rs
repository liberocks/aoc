use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"(?P<do>do\(\))|(?P<dont>don't\(\))|(?P<mul>mul\((\d{1,3}),(\d{1,3})\))")
        .unwrap();

    let mut sum: u64 = 0;
    let mut enabled = true;

    let mut commands = Vec::new();

    for caps in re.captures_iter(&contents) {
        let m = caps.get(0).unwrap();
        let command_type = if caps.name("do").is_some() {
            "do"
        } else if caps.name("dont").is_some() {
            "dont"
        } else if caps.name("mul").is_some() {
            "mul"
        } else {
            continue;
        };
        commands.push((m.start(), command_type.to_string(), caps));
    }

    commands.sort_by_key(|k| k.0);

    for (_, command_type, caps) in commands {
        match command_type.as_str() {
            "do" => {
                enabled = true;
            }
            "dont" => {
                enabled = false;
            }
            "mul" => {
                if enabled {
                    let a = caps.get(4).unwrap().as_str().parse::<u64>().unwrap();
                    let b = caps.get(5).unwrap().as_str().parse::<u64>().unwrap();
                    sum += a * b;
                }
            }
            _ => {}
        }
    }

    println!("Sum of all correct patterns: {}", sum);

    Ok(())
}
