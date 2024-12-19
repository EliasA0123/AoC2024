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

fn can_fill(pat: &str, towels: &Vec<&str>, filled_i: usize) -> bool {
    // println!("{}: {}", pat, filled_i);
    if filled_i == pat.len() {
        return true;
    }
    if filled_i > pat.len() {
        return false;
    }

    for &towel in towels {
        if filled_i + towel.len() <= pat.len() {
            if towel == &pat[filled_i..(filled_i + towel.len())] {
                // println!("worked");
                if can_fill(pat, towels, filled_i + towel.len()) {
                    return true;
                }
            }
        }
    }
    return false;
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut input = input.split("\n\n");
    let towels: Vec<&str> = input.next().unwrap().split(", ")
        // .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let patterns: Vec<&str> = input.next().unwrap().lines().collect();

    let mut count = 0;
    for pat in patterns {
        if can_fill(pat, &towels, 0) {
            count += 1;
        }
    }

    println!("{}\nTime: {:.2?}", count, start.elapsed());
}