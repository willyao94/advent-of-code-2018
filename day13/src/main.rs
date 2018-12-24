use std::collections::HashSet;

struct Cart {
    x: usize,
    y: usize,
    dir: char,
    intersec: usize,
}

impl Cart {
    fn new (x:usize, y:usize, dir:char) -> Self {
        Cart {
            x: x,
            y: y,
            dir: dir,
            intersec: 0,
        }
    }

    fn tick(&mut self, matrix:&Vec<Vec<char>>){
        // Move
        assert_ne!(matrix[self.y][self.x], ' ');
        match self.dir {
            '<' => self.x -= 1,
            '>' => self.x += 1,
            '^' => self.y -= 1,
            'v' => self.y += 1,
            _ => println!("Panic!")
        }

        // Check if dir needs to be updated
        match matrix[self.y][self.x] {
            '+' => {
                let mut turn = ' ';
                if self.dir == '<' {
                    match self.intersec % 3 {
                        0 => turn = 'v',
                        1 => turn = '<',
                        2 => turn = '^',
                        _ => println!("Panic!")
                    };
                } else if self.dir == '>' {
                    match self.intersec % 3 {
                        0 => turn = '^',
                        1 => turn = '>',
                        2 => turn = 'v',
                        _ => println!("Panic!")
                    };
                } else if self.dir == '^' {
                    match self.intersec % 3 {
                        0 => turn = '<',
                        1 => turn = '^',
                        2 => turn = '>',
                        _ => println!("Panic!")
                    };
                } else if self.dir == 'v' {
                    match self.intersec % 3 {
                        0 => turn = '>',
                        1 => turn = 'v',
                        2 => turn = '<',
                        _ => println!("Panic!")
                    };
                }
                self.dir = turn;
                self.intersec += 1;
            },
            '/' => {
                match self.dir {
                    '<' => self.dir = 'v',
                    '>' => self.dir = '^',
                    '^' => self.dir = '>',
                    'v' => self.dir = '<',
                    _ => println!("Panic!")
                };
            },
            '\\' => {
                match self.dir {
                    '<' => self.dir = '^',
                    '>' => self.dir = 'v',
                    '^' => self.dir = '<',
                    'v' => self.dir = '>',
                    _ => println!("Panic!")
                };
            },
            _ => ()
        }
    }
}

fn print_matrix(matrix:&Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{}", matrix[i][j]);
        }
        println!("");
    }
}

fn is_cart(c:char) -> bool {
    c == '^' || c == 'v' || c == '<' || c == '>'
}

fn main() {
    //let input = include_str!("../data/test.txt");
    //let input = include_str!("../data/testp2.txt");
    let input = include_str!("../data/day13.txt");

    let mut matrix:Vec<Vec<char>> = input.lines()
                            .map(|x| x.chars().collect())
                            .collect();

    let mut carts = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if is_cart(matrix[i][j]) {
                let cart = Cart::new(j,i, matrix[i][j]);
                carts.push(cart);

                match matrix[i][j] {
                    '<' => matrix[i][j] = '-',
                    '>' => matrix[i][j] = '-',
                    '^' => matrix[i][j] = '|',
                    'v' => matrix[i][j] = '|',
                    _ => println!("Panic!")
                }
            }
            
        }
    }

    loop {
        carts.sort_by_key(|cart| (cart.x, cart.y));
        let mut collision_indices = Vec::new();
        // Keep track of all current positions as well
        // to deal with --<>-- cases
        let mut positions = carts.iter()
                                .map(|cart| (cart.x, cart.y))
                                .collect::<HashSet<(usize, usize)>>();

        for i in 0..carts.len() {
            &carts[i].tick(&matrix);
            let coord = (carts[i].x, carts[i].y);
            if positions.contains(&coord) {
                // Find the index of other cart
                for j in 0..carts.len() {
                    if i==j { continue; }
                    if carts[j].x == carts[i].x  && carts[j].y == carts[i].y {
                        println!("{} {}", coord.0, coord.1);
                        collision_indices.push(i);
                        collision_indices.push(j);
                        break;
                    }
                }
            }
            positions.insert(coord);
        }
        if collision_indices.len() > 0 {
            collision_indices.sort_by(|a, b| b.cmp(a));
            for i in collision_indices {
                carts.remove(i);
            }
        }
        if carts.len() == 1 {
            println!("Part 2: {} {}", carts[0].x, carts[0].y);
            break;
        }
    }
}
