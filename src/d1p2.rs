use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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
    let input = input.lines();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: HashMap<i32, i32> = HashMap::new();
    for line in input {
        let mut parts = line.split("   ");
        left_list.push(parts.next().unwrap().parse().unwrap());
        right_list.entry(parts.next().unwrap().parse().unwrap())
            .and_modify(|e| {*e += 1})
            .or_insert(1);
    }
    let mut similarity_score = 0;
    for n in &left_list {
        match right_list.get(n) {
            Some(count) => similarity_score += n * count,
            None => ()
        };
    }
    println!("{}", similarity_score);
}