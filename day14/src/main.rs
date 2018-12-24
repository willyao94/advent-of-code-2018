fn next_10_recipes_scores(input:usize) -> String {
    let mut recipes:Vec<usize> = Vec::new();
    recipes.push(3);
    recipes.push(7);
    let (mut fst, mut sec) = (0, 1);
    loop {
        let sum = recipes[fst] + recipes[sec];
        if sum >= 10 {
            recipes.push(sum / 10);
        }
        recipes.push(sum % 10);
        fst = (fst + recipes[fst] + 1) % recipes.len();
        sec = (sec + recipes[sec] + 1) % recipes.len();
        
        if recipes.len() > input + 10 {
            break;
        }
    }
    let mut res = String::new();
    (input..input+10).for_each(|n| res.push_str(&recipes[n].to_string()));
    res
}

fn reverse_match(src:&Vec<usize>, target:&Vec<usize>) -> bool {
    let mut count = 0;
    for i in 0..target.len() {
        if target[i] != src[src.len() - i - 1] {
            break;
        }
        count += 1;
    }
    count == target.len()
}

fn recipes_before_sequence(input:&str) -> usize {
    let mut recipes:Vec<usize> = Vec::new();
    recipes.push(3);
    recipes.push(7);
    let (mut fst, mut sec) = (0, 1);
    let mut search:Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    search.reverse();
    loop {
        let sum = recipes[fst] + recipes[sec];
        if sum >= 10 {
            recipes.push(sum / 10);
            if recipes.len() >= search.len() && recipes[recipes.len()-1] == search[0]{
                if reverse_match(&recipes, &search) {
                    return recipes.len() - search.len();
                }
            }
        }
        recipes.push(sum % 10);
        if recipes.len() >= search.len() && recipes[recipes.len()-1] == search[0]{
            if reverse_match(&recipes, &search) {
                return recipes.len() - search.len();
            }
        }

        fst = (fst + recipes[fst] + 1) % recipes.len();
        sec = (sec + recipes[sec] + 1) % recipes.len();
    }
}

fn main() {
    assert_eq!(next_10_recipes_scores(9), "5158916779");
    assert_eq!(next_10_recipes_scores(5), "0124515891");
    assert_eq!(next_10_recipes_scores(18), "9251071085");
    assert_eq!(next_10_recipes_scores(2018), "5941429882");
    
    assert_eq!(recipes_before_sequence("51589"), 9);
    assert_eq!(recipes_before_sequence("01245"), 5);
    assert_eq!(recipes_before_sequence("92510"), 18);
    assert_eq!(recipes_before_sequence("59414"), 2018);

    let input = 846601;
    println!("Part 1: {}", next_10_recipes_scores(input));
    println!("Part 2: {}", recipes_before_sequence(&input.to_string()));
}