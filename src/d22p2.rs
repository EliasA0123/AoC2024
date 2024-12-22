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

fn next_secret(mut num: u64) -> u64 {
    num = (num ^ (num * 64)) % 16777216;
    num = (num ^ (num / 32)) % 16777216;
    num = (num ^ (num * 2048)) % 16777216;
    num
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut sum = 0;
    for num in input.lines() {
        let mut num = num.parse::<u64>().unwrap();
        for _ in 0..2000 {
            num = next_secret(num);
        }
        sum += num;
    }

    println!("{}\nTime: {:.2?}", sum, start.elapsed());
}