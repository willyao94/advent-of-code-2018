use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

extern crate regex;

fn main() {
    let f = match File::open("./data/day03.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file.");
            return;
        }
    };

    let file = BufReader::new(&f);
    //let mut claims = [[0u8; 11]; 9];
    let mut unlayered_ids = HashSet::new();
    let mut claims = HashMap::new();
    let mut key_mapping = HashMap::new();

    let re = Regex::new(r"#(\d+).*\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    for line in file.lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            let parse: Vec<i32> = vec![&cap[1], &cap[2], &cap[3], &cap[4], &cap[5]]
                                    .into_iter()
                                    .map(|x| x.trim().parse().unwrap()) // Parse int
                                    .collect();

            let id = parse[0];
            let x = parse[1];
            let y = parse[2];
            let dx = parse[3];
            let dy = parse[4];

            unlayered_ids.insert(id);
            for j in y..y+dy {
                for i in x..x+dx {
                    let key = (j,i);
                    // Part 1:
                    *claims.entry(key).or_insert(0) += 1;

                    // Part 2:
                    if key_mapping.contains_key(&key) {
                        let other_id = key_mapping.get(&key).unwrap();
                        unlayered_ids.remove(&id);
                        unlayered_ids.remove(&other_id);
                    } else {
                        key_mapping.insert(key, id);
                    }
                }
            }
        }
    }

    // Using fold aka reduce
    /*let count = claims.values().fold(0, |count, &x| {
        if x > 1 {
            count + 1
        } else {
            count
        }
    });*/

    // More idiomatic
    let count = claims.values().filter(|x| **x > 1).count();
    println!("Part 1: {}", count);

    for i in unlayered_ids.iter() {
        println!("Part 2: {}", i);
    }
}