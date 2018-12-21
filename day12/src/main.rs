use std::collections::HashMap;
use std::collections::LinkedList;

static PATTERN_LENGTH:usize = 5;
static TIME:isize = 50000000000;
static PART_1_GENS:usize = 20;

struct Pot {
    id: i32,
    val: char,
}

impl Pot {
    fn new(id:i32, val:char) -> Self {
        Pot {
            id: id,
            val: val,
        }
    }
}

fn grow_plants(pots:&Vec<Pot>, patterns:&HashMap<String, char>) -> Vec<Pot> {
    let mut deque_str = String::from(".....");
    // Take growing left into account
    let mut pot_id = pots.first().unwrap().id - 3;
    let mut next_gen:Vec<Pot> = Vec::new();
    let mut iter = pots.iter();
    for _ in 0..pots.len() + PATTERN_LENGTH - 1 {
        let next = patterns.get(&deque_str).ok_or('.').unwrap();
        next_gen.push(Pot::new(pot_id, *next));

        // Length should be at most PATTERN_LENGTH
        deque_str.remove(0);
        match iter.next() {
            Some(p) => deque_str.push(p.val),
            None => deque_str.push('.')
        }
        pot_id+=1;
    }
    next_gen
}

fn main() {
    //let input = include_str!("../data/test.txt");
    let input = include_str!("../data/day12.txt");

    let mut patterns:HashMap<String, char> = HashMap::new();
    let mut iter = input.lines();
    let mut i = -1;
    let mut pots:Vec<Pot> = iter.next()
                            .unwrap()
                            .split_whitespace()
                            .collect::<Vec<&str>>()[2]
                            .chars()
                            .map(|x| {
                                i+=1;
                                Pot::new(i,x)
                            })
                            .collect();
    // Empty line
    iter.next();
    for line in iter {
        let split:Vec<&str> = line.split_whitespace().collect();
        patterns.insert(String::from(split[0]), split[2].chars().next().unwrap());
    }
    
    for _ in 0..PART_1_GENS {
        pots = grow_plants(&pots, &patterns);
    }
    println!("Part 1: {}", pots.iter().filter(|x| x.val == '#').map(|x| x.id).sum::<i32>());

    let mut prev_sum = 0;
    let mut prev_deltas = LinkedList::new();
    let mut iterations = PART_1_GENS;   // Continue where left off
    let stable = 100;
    loop {
        pots = grow_plants(&pots, &patterns);
        let pot_sum = pots.iter().filter(|x| x.val == '#').map(|x| x.id).sum();
        let delta = pot_sum - prev_sum;
        // Asssume it will continue to grow at a steady rate
        // if past n generations grew at the same rate
        if prev_deltas.len() >= stable && prev_deltas.iter().all(|&x| x == delta) {
            println!("Part 2: {}", pot_sum as isize + (TIME - iterations as isize - 1) * delta as isize);
            break;
        }
        if prev_deltas.len() >= stable {
            prev_deltas.pop_front();
        }
        prev_deltas.push_back(delta);
        iterations += 1;
        prev_sum = pot_sum;
    }
}