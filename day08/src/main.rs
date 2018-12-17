fn sum_metadata(data:&mut Vec<i32>, sum:&mut i32) {
    let child_count = data.pop().unwrap();
    let metadata_count = data.pop().unwrap();
    for _ in 0..child_count {
        sum_metadata(data, sum);
    }

    *sum += (0..metadata_count).map(|_| data.pop().unwrap()).sum::<i32>();
}

fn sum_metadata_2(data:&mut Vec<i32>) -> i32 {
    let child_count = data.pop().unwrap();
    let metadata_count = data.pop().unwrap();
    // Recurse and collect children
    let children = (0..child_count).map(|_| sum_metadata_2(data)).collect::<Vec<i32>>();

    (0..metadata_count).map(|_| {
        let i = data.pop().unwrap();
        if child_count == 0 {   // Hopefully branch prediction
            i
        } else {
            // Children is not 0 indexed so -1
            // Might reference a child node that does not exist
            match children.get(i as usize - 1) {
                Some(&val) => val,
                None => 0
            }
        }
    }).sum::<i32>()
}

fn main() {
    //let input = include_str!("../data/test.txt");
    let input = include_str!("../data/day08.txt");
    let mut data: Vec<i32> = input.split_whitespace()
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|x| x.parse().unwrap())
                            .collect();
    data.reverse();
    let mut sum = 0;
    sum_metadata(&mut data.clone(), &mut sum);
    println!("Part 1: {}", sum);

    sum = sum_metadata_2(&mut data.clone());
    println!("Part 2: {}", sum);
}
