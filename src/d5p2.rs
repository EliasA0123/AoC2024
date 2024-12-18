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



fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut input = input.split("\n\n");
    let rules = input.next().unwrap().lines();
    let updates = input.next().unwrap().lines()
        .map(|update| update.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    
    let mut count = 0;

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in rules.clone() {
        let mut rule = rule.split("|");
        let num1: u32 = rule.next().unwrap().parse().unwrap();
        let num2: u32 = rule.next().unwrap().parse().unwrap();
        rules_map.entry(num2)
            .and_modify(|v| v.push(num1))
            .or_insert(vec![num1]);
    }

    'update: for update in updates {
        for i in 0..update.len() {
            if rules_map.get(&update[i]).is_some() {
                for prereq in rules_map.get(&update[i]).unwrap() {
                    if !update[0..i].contains(prereq) && update.contains(prereq) {
                        // Fix time - super inefficient?
                        let mut fixed_update: Vec<u32> = Vec::new();
                        while fixed_update.len() < update.len() {
                            'fix: for num in &update {
                                if !fixed_update.contains(num) {
                                    if let Some(v) = rules_map.get(num) {
                                        for newnum in v {
                                            if update.contains(newnum) && !fixed_update.contains(newnum) {
                                                continue 'fix;
                                            }
                                        }
                                    fixed_update.push(*num);
                                    }
                                }
                            }
                        }
                        count += fixed_update[(fixed_update.len() + 1) / 2 - 1];
                        continue 'update;
                    }
                }
            }
        }
    }
    println!("{}\nTime: {:.2?}", count, start.elapsed());
}