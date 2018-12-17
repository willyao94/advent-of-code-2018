use std::collections::HashMap;

static IDLE:char = ' ';
static STEP:i32 = 60;

fn lexicographical_topological_sort(adj_list:&Vec<(char, char)>) -> String {
    let mut graph = HashMap::new();
    let mut indegree = HashMap::new();
    for (src, dst) in adj_list.iter() {
        graph.entry(src).or_insert(Vec::new()).push(dst);
        graph.entry(dst).or_insert(Vec::new());
        indegree.entry(src).or_insert(0);
        *indegree.entry(dst).or_insert(0) += 1;
    }

    let mut stack = indegree.keys()
                        .cloned()   // to allow immutable borrow
                        .filter(|&x| indegree.get(x).unwrap() == &0)
                        .collect::<Vec<_>>();
    // Reverse sort to process alphabetically
    stack.sort_by(|a,b| b.cmp(a));
    let mut order = Vec::new();
    
    while stack.len() > 0 {
        let node = stack.pop().unwrap();
        order.push(node);
        let neighbours = graph.get_mut(node).unwrap();
        for nei in neighbours.iter() {
            // Satisfy a requirement
            if let Some(x) = indegree.get_mut(nei) {
                *x -= 1;
            }
            // All requirements satisfied
            if indegree.get(nei).unwrap() == &0 {
                stack.push(nei);
            }
        }   
        stack.sort_by(|a,b| b.cmp(a));
    }

    order.into_iter().collect()
}

struct Worker {
    work_id: char,
    time: i32,
}

impl Worker {
    fn new (work_id: char, time: i32) -> Self {
        Worker {
            work_id: work_id,
            time: time,
        }
    }
}

impl Worker {
    fn is_working(&self) -> bool {
        self.time > 0
    }
}

fn parallel_topological_sort(adj_list:&Vec<(char, char)>, num_workers: u32) -> u32 {
    let mut graph = HashMap::new();
    let mut indegree = HashMap::new();
    for (src, dst) in adj_list.iter() {
        graph.entry(src).or_insert(Vec::new()).push(dst);
        graph.entry(dst).or_insert(Vec::new());
        indegree.entry(src).or_insert(0);
        *indegree.entry(dst).or_insert(0) += 1;
    }

    let mut stack = indegree.keys()
                        .cloned()   // to allow immutable borrow
                        .filter(|&x| indegree.get(x).unwrap() == &0)
                        .collect::<Vec<_>>();
    // Reverse sort to process alphabetically
    stack.sort_by(|a,b| b.cmp(a));
    let mut order: Vec<char> = Vec::new();
    let mut workers: Vec<Worker> = (0..num_workers).map(|_| Worker::new(IDLE, 0)).collect();
    let mut elapsed_time = 0;

    while stack.len() > 0  || workers.iter().any(|x| x.is_working()) {
        for i in 0..workers.len() {
            workers[i].time -= 1;
            if !workers[i].is_working() {
                let curr = workers[i].work_id;
                if curr != IDLE {
                    order.push(curr);
                    let neighbours = graph.get(&curr).unwrap();
                    for nei in neighbours.iter() {
                        // Satisfy a requirement
                        if let Some(x) = indegree.get_mut(nei) {
                            *x -= 1;
                        }
                        // All requirements satisfied
                        if indegree.get(nei).unwrap() == &0 {
                            stack.push(nei);
                        }
                    }
                }
                
                // Assign new work if available
                match stack.pop() {
                    Some(&next_id) => {
                        let len = STEP + next_id as i32 - 'A' as i32 + 1;
                        workers[i] = Worker::new(next_id, len);
                    },
                    None => if workers[i].work_id != IDLE {
                        workers[i] = Worker::new(IDLE, 0)
                    }
                }
            }
            stack.sort_by(|a,b| b.cmp(a));
        }
        elapsed_time += 1;
    }
    //println!("{}", order.into_iter().collect::<String>());
    elapsed_time-1
}

fn main() {
    //let input = include_str!("../data/test.txt");
    let input = include_str!("../data/day07.txt");

    let mut adj_list = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let edge = (split[1].chars().next().unwrap(), split[7].chars().next().unwrap());
        adj_list.push(edge);
    }

    println!("Part 1: {}", lexicographical_topological_sort(&adj_list));
    let num_workers = 5;
    println!("Part 2: {}", parallel_topological_sort(&adj_list, num_workers));
}
