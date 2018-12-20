use std::cmp;
use std::collections::VecDeque;

static GRID_SIZE:i32 = 300;
static SQUARE_SIZE:i32 = 3;

fn cell_power(x:i32, y:i32, serial:i32) -> i32 {
    let rack_id = x + 10;
    let power_lvl = (rack_id * y + serial) * rack_id;
    if power_lvl<100 {
        -5
    } else {
        // Hundreds digit - 5
        power_lvl/100 % 10 - 5
    }
}

fn build_power(grid:&mut Vec<Vec<i32>>, serial:i32) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            grid[i][j] = cell_power(j as i32, i as i32, serial);
        }
    }
}

fn largest_power_square(grid:&Vec<Vec<i32>>, size:i32) -> (i32, i32, i32) {
    let mut largest_total_power = std::i32::MIN;
    let squared = (size * size) as usize;
    let (mut x, mut y) = (0, 0);
    for i in 0..grid.len()-size as usize {
        let mut sum = 0;
        let mut deque = VecDeque::new();
        for j in 0..grid[0].len()-size as usize {
            // Calculate moving sum of square using deque
            for k in 0..size as usize {
                if deque.len() >= squared {
                    sum -= deque.pop_back().unwrap();
                }
                deque.push_front(grid[i+k][j]);
                sum += grid[i+k][j];
            }
            if sum > largest_total_power {
                largest_total_power = sum;
                x = cmp::max(j as i32 - size + 1, 0);
                y = i;
            }
        }
    }
    (largest_total_power, x as i32, y as i32)
}

fn main() {
    assert_eq!(cell_power(3, 5, 8),4);
    assert_eq!(cell_power(122, 79, 57), -5);
    assert_eq!(cell_power(217, 196, 39), 0);
    assert_eq!(cell_power(101, 153, 71), 4);

    let serial = 7672;
    let mut fuel_cells = Vec::new();
    for _ in 0..GRID_SIZE {
        fuel_cells.push(vec![0; GRID_SIZE as usize]);
    }

    build_power(&mut fuel_cells, serial);
    let (_, x, y) = largest_power_square(&fuel_cells, SQUARE_SIZE);
    println!("Part 1: {},{}", x,y);

    let mut largest_size = 1;
    let mut largest_total = std::i32::MIN;
    let mut largest_x = 0;
    let mut largest_y = 0;
    for i in 0..GRID_SIZE {
        let (power, x, y) = largest_power_square(&fuel_cells, i);
        if power > largest_total {
            largest_total = power;
            largest_size = i;
            largest_x = x;
            largest_y = y;
        }
    }
    println!("Part 2: {},{}, {}", largest_x, largest_y, largest_size);
}
