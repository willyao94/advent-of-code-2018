use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

static SAFE_DISTANCE:i32 = 10000;

fn manhatten_distance(p0:(i32,i32), p1:(i32,i32)) -> i32 {
    (p0.0 - p1.0).abs() + (p0.1 - p1.1).abs()
}

fn main() {
    //let input = include_str!("../data/test.txt");
    let input = include_str!("../data/day06.txt");
    let mut points = Vec::new();
    let mut finites = HashSet::new();
    let (mut min_x, mut min_y, mut max_x, mut max_y)  = (0,0,0,0);
    for line in input.lines() {
        let tmp: Vec<i32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
        let point = (tmp[0], tmp[1]);
        points.push(point);
        finites.insert(point);
        min_x = cmp::min(min_x, tmp[0]);
        min_y = cmp::min(min_y, tmp[1]);
        max_x = cmp::max(max_x, tmp[0]);
        max_y = cmp::max(max_y, tmp[1]);
    }

    // Normalize every point so the smallest point is (0,0)
    points = points.iter().map(|x| (x.0-min_x, x.1-min_y)).collect();
    let size_x = max_x-min_x+1;
    let size_y = max_y-min_y+1;
    
    // Build matrix
    let mut matrix = Vec::new();
    for _ in 0..=size_x {
        matrix.push(vec![(0,0); size_y as usize]);
    }

    let mut areas = HashMap::new();
    let mut safe_count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let point = (i as i32,j as i32);
            let mut min_dist = std::i32::MAX;
            let mut total_dist = 0;
            let mut closest = Vec::new();
            for p in points.iter()  {
                let man_dist = manhatten_distance(point, *p);
                total_dist += man_dist;
                if man_dist == min_dist {
                    matrix[i][j] = (0,0);
                    closest.push(p);
                } else if man_dist < min_dist {
                    min_dist = man_dist;
                    matrix[i][j] = *p;
                    closest.clear();
                    closest.push(p);
                }
            }
            if closest.len() == 1 {
                *areas.entry(closest[0]).or_insert(0) += 1;
            }
            if total_dist < SAFE_DISTANCE {
                safe_count += 1;
            }
        }
    }
    // Remove anything on border as those expand infinitely
    // Rows
    for i in 0..matrix[0].len() {
        finites.remove(&matrix[0][i]);
        finites.remove(&matrix.last().unwrap()[i]);
    }
    // Cols
    for i in 0..matrix.len() {
        finites.remove(&matrix[i][0]);
        finites.remove(&matrix[i].last().unwrap());
    }

    let max_area = finites.iter().fold(0, |max, p| cmp::max(max, *areas.get(&*p).unwrap()));
    println!("Part 1: {}", max_area);
    println!("Part 2: {}", safe_count);
}
