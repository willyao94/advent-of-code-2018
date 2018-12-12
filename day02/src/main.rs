use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let f = match File::open("./data/day02.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file.");
            return;
        }
    };
    let file = BufReader::new(&f);
    let mut data: Vec<String> = Vec::new();
    for line in file.lines() {
        data.push(line.unwrap());
    }

    // Part 1:
    let mut twos = 0;
    let mut threes = 0;
    for i in 0..data.len() {
        let mut cache = HashMap::new();
        let s = &data[i];
        for c in s.chars() {
            /*let count = cache.entry(c).or_insert(0);
            *count += 1;*/
            *cache.entry(c).or_insert(0) += 1;
        }
        /*let mut two = false;
        let mut three = false;
        for (_, value) in &cache {
            if *value == 2 { two = true; }
            if *value == 3 { three = true; }
        }
        if two { twos += 1; }
        if three { threes += 1; }*/
        
        // Cleaner
        if cache.values().any(|&count| count == 2) { twos += 1; }
        if cache.values().any(|&count| count == 3) { threes += 1; }
    }
    println!("Part 1: {}", twos * threes);

    // Part 2:
    /*'outer: for i in 0..data.len() {
        for j in i+1..data.len()-1 {
            let s1 = &data[i].as_bytes();
            let s2 = &data[j].as_bytes();
            let mut diff_count: u32 = 0;
            let mut diff_ind = 0;
            for k in 0..s1.len() {
                if &s1[k] != &s2[k] {
                    if diff_count > 1 { break; }
                    diff_count += 1;
                    diff_ind = k;
                }
            }
            if diff_count == 1{
                let mut res = String::new();
                for (i, c) in data[i].chars().enumerate() {
                    if i == diff_ind { continue; }
                    let tmp = c.to_string();
                    res.push_str(&tmp);
                }
                println!("Part 2: {}", res);
                break 'outer;
            }
        }
    }*/

    // Cleaner part 2
    'outer: for i in 0..data.len() {
        for j in i+1..data.len()-1 {
            let s1 = data[i].chars();
            let s2 = data[j].chars();
            let mut res = String::new();
            let mut diff_count = 0;
            for (c1, c2) in s1.zip(s2) {
                if c1 == c2 { res.push(c1); }
                else { diff_count += 1; }

                if diff_count > 1 { break; }
            }
            if diff_count == 1 {
                println!("Part 2: {}", res);
                break 'outer;
            }
        }
    }
}