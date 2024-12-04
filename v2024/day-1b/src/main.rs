use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: HashMap<i32, i32> = HashMap::new();
    for line in contents.lines() {
        let splitted: Vec<i32> = line
            .split("   ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        lefts.push(splitted[0]);

        let right = splitted[1];
        if rights.contains_key(&right) {
            let count = rights.get_mut(&right).unwrap();
            *count += 1;
        } else {
            rights.insert(right, 1);
        }
    }

    let mut similarity: i32 = 0;
    for left in lefts.iter() {
        if rights.contains_key(left) {
            similarity += left * rights.get(left).unwrap();
        }
    }

    println!("Similarity score: {:?}", similarity);

    Ok(())
}
