use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let f = match File::open("./data/day01.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file.");
            return;
        }
    };
    let file = BufReader::new(&f);
    let mut v: Vec<i32> = Vec::new();
    for line in file.lines() {
        let l = line.unwrap();
        let n: i32 = l.trim().parse().unwrap();
        v.push(n);
    }

    // Part 1:
    let sum: i32 = v.iter().sum();
    println!("Part 1: {}", sum);

    // Part 2:
    /*let mut seen = HashSet::new();
    let mut i: i32 = 0;
    let mut n = 0;
    loop {
        n += v[i as usize % v.len()];
        if seen.contains(&n) {
            println!("Part 2: {}", n);
            break;
        } else {
            seen.insert(n);
        }
        i += 1;
    }*/

    // Cleaner part 2 solution
    {
        let mut seen = HashSet::new();
        let mut sum = 0;
        let res =  v.iter()
                    .cycle()    // repeats an iterator endlessly
                    .find_map(|n| { // maps func and return first non-none result
                        sum += n;
                        seen.replace(sum)   // use this to break cycle; replaces existing val and returns it
                    })
                    .unwrap();
        println!("Part 2: {}", res);
    }
}