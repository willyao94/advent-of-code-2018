extern crate regex;

use regex::Regex;
use std::cmp;

fn print_matrix(matrix:&Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{}", matrix[i][j]);
        }
        println!("");
    }
}

fn main() {
    //let input = include_str!("../data/test.txt");
    let input = include_str!("../data/day10.txt");

    let re = Regex::new(r".*<(.?\d+),\s+(.?\d+)>.*<(.?\d+),\s+(.?\d+)>").unwrap();
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    input.lines().for_each(|line| {
        let vals:Vec<i32> = re.captures(line).unwrap()
                                .iter()
                                .filter_map(|cap| cap.unwrap().as_str().trim().parse().ok())
                                .collect();
        positions.push((vals[0], vals[1]));
        velocities.push((vals[2], vals[3]));
    });

    // Previous slice consists of a copy of points' pos, bounding_x, bounding_y
    let mut prev_slice:(Vec<(i32,i32)>, i32, i32) = (Vec::new(), 0, std::i32::MAX);
    let mut iterations = 0;
    
    loop {
        // Tick
        for i in 0..positions.len() {
            positions[i] = (positions[i].0 + velocities[i].0, positions[i].1 + velocities[i].1);
        }

        // Determine whether the message is formed
        // Message is formed when the delta of the 
        // highest and lowest point is at a minimum
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN);
        for p in positions.iter() {
            min_x = cmp::min(min_x, p.0);
            min_y = cmp::min(min_y, p.1);
            max_x = cmp::max(max_x, p.0);
            max_y = cmp::max(max_y, p.1);
        }

        let (bounding_x, bounding_y) = (max_x - min_x + 1, max_y - min_y + 1);
        // If the bounding box stopped growing smaller
        if prev_slice.2 < bounding_y {
            let mut matrix = Vec::new();
            for _ in 0..prev_slice.2 {
                matrix.push(vec!['.'; prev_slice.1 as usize]);
            }
            
            let (mut prev_min_x, mut prev_min_y) = (std::i32::MAX, std::i32::MAX);
            for p in prev_slice.0.iter() {
                prev_min_x = cmp::min(prev_min_x, p.0);
                prev_min_y = cmp::min(prev_min_y, p.1);
            }

            for p in prev_slice.0.iter() {
                let (normalized_x, normalized_y) = (p.0 - prev_min_x, p.1 - prev_min_y);
                matrix[normalized_y as usize][normalized_x as usize] = '#';
            }

            println!("Part 1:");
            print_matrix(&matrix);
            break;
        }

        prev_slice = (positions.clone(), bounding_x, bounding_y);
        iterations += 1;
    }
    println!("Part 2: {}", iterations);
}
