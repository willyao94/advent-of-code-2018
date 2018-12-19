use std::collections::VecDeque;

static CW_DIST:i32 = 2;
static CCW_DIST:i32 = 7;
static SCORE_MOD:i32 = 23;

fn marble_mania(player_count:usize, marble_count:u32) -> u32 {
    let mut circle = VecDeque::with_capacity(marble_count as usize);
    let mut score = vec![0u32; player_count];
    circle.push_back(0);
    for marble in 1..=marble_count {
        if marble as i32 % SCORE_MOD == 0 {
            (0..CCW_DIST).for_each(|_| {
                let tmp = circle.pop_back().unwrap();
                circle.push_front(tmp);
            });
            score[marble as usize % player_count] += marble + circle.pop_front().unwrap();
        } else {
            (0..CW_DIST).for_each(|_| {
                let tmp = circle.pop_front().unwrap();
                circle.push_back(tmp);
            });
            circle.push_front(marble);
        }
    }

    *score.iter().max().unwrap()
}

fn main() {
    // Tests
    let tests = include_str!("../data/test.txt");
    for line in tests.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let vars = split.iter().filter_map(|x| x.parse().ok()).collect::<Vec<u32>>();
        assert_eq!(marble_mania(vars[0] as usize, vars[1]), vars[2]);
    }

    let input = include_str!("../data/day09.txt");
    let split = input.split_whitespace().collect::<Vec<&str>>();
    let (player_count, marble_count) = (split[0].parse().unwrap(), split[6].parse().unwrap());
    println!("Part 1: {}", marble_mania(player_count, marble_count));
    println!("Part 2: {}", marble_mania(player_count, marble_count*100));
}
