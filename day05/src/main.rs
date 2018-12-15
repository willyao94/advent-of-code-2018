use std::cmp;

fn opp_polarity(a:char, b:char) -> bool {
    //a!=b && a.to_ascii_lowercase() == b.to_ascii_lowercase() 
    a as u8 ^ b as u8 == 32     // Equivalent to above
}

fn react(data:&str) -> Vec<char> {
    let mut polymer: Vec<char> = Vec::new();
    for c in data.chars() {
        match polymer.last() {
            Some(&last) => if opp_polarity(c, last) {
                polymer.pop();
            } else {
                polymer.push(c);
            },
            None => polymer.push(c)
        }
    }
    polymer
}

fn main() {
    //let input = include_str!("../data/test.txt");
    let input = include_str!("../data/day05.txt");
    let part1 = react(input);
    println!("Part 1: {}", part1.len());

    let mut part2 = std::usize::MAX;
    for i in 'A' as u8..='Z' as u8 {
        let filtered = input.chars().filter(|c| c.to_ascii_uppercase() != i as char).collect::<String>();
        part2 = cmp::min(part2, react(&filtered).len());
    }
    println!("Part 2: {}", part2);
}
