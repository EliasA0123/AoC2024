use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
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

fn sort_and_tuplize(a: u32, b: u32) -> (u32, u32) {
    if a < b { (a, b) } else { (b, a) }
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut input = input.split("\n\n");
    let rules = input.next().unwrap().lines()
        .map(|rule| rule.split("|").map(|s| s.parse().unwrap()).collect::<Vec<u32>>());
    let updates = input.next().unwrap().lines()
        .map(|update| update.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    
    let mut safe_count = 0;

    let mut rules_map: HashMap<(u32, u32), u32> = HashMap::new();
    for rule in rules {
        rules_map.insert(sort_and_tuplize(rule[0], rule[1]), rule[0]);
    }

    'update: for update in updates {
        for i in 0..update.len() {
            for j in i+1..update.len() {
                if rules_map.get(&sort_and_tuplize(update[i], update[j])).unwrap() != &update[i] {
                    continue 'update;
                }
            }
        }
        safe_count += update[(update.len() + 1) / 2 - 1];
    }
    println!("{}\nTime: {:.2?}", safe_count, start.elapsed());
}