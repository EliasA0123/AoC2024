use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn get_input() -> String {
    let mut file = match File::open("input.txt") {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read file: {}", why),
        Ok(_) => {},
    }
    input
}

fn concat(a: u64, b: u64) -> u64 {
    // (a.to_string() + &b.to_string()).parse().unwrap() <-- boring way
    let mut len_finder = 1;
    while len_finder <= b { len_finder *= 10 }
    a * len_finder + b
}

fn explore(goal: u64, mut nums: Vec<u64>, current_val: u64) -> bool {
    if current_val > goal {
        return false;
    }
    if nums.len() == 0 {
        return goal == current_val;
    }
    let next = nums.remove(0);

    if explore(goal, nums.clone(), current_val * next) { return true; }
    if explore(goal, nums.clone(), current_val + next) { return true; }
    if explore(goal, nums.clone(), concat(current_val, next)) { return true; }
    return false;
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let lines = input.lines();

    let mut count = 0;
    for line in lines {
        let mut nums: Vec<&str> = line.split(" ").collect();

        let goal = nums.remove(0);
        let goal = goal.replace(":", "");
        let goal: u64 = goal.parse().unwrap();
        
        let mut nums: Vec<u64> = nums.into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
        let first = nums.remove(0);

        if explore(goal, nums.clone(), first) {
            count += goal;
        }
    }
    println!("{}\nTime: {:.2?}", count, start.elapsed());
}