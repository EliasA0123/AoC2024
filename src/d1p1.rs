use std::fs::File;
use std::io::prelude::*;

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

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in input {
        let mut parts = line.split("   ");
        list1.push(parts.next().unwrap().parse().unwrap());
        list2.push(parts.next().unwrap().parse().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut diffs_sum: i32 = 0;
    for i in 0..list1.len() {
        diffs_sum += (list1[i] - list2[i]).abs();
    }
    println!("{}", diffs_sum);
}