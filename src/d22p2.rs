use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::{HashMap, VecDeque};

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

    let mut bananas_map: HashMap<(i64, i64, i64, i64), Vec<(usize, u64)>> = HashMap::new();
    for (id, num) in input.lines().enumerate() {
        let mut dc = VecDeque::new();
        let mut prev_num = num.parse::<u64>().unwrap();
        for _ in 0..2000 {
            let num = next_secret(prev_num);
            dc.push_back((num % 10) as i64 - (prev_num % 10) as i64);
            if dc.len() > 4 {
                dc.pop_front();
            }

            if dc.len() == 4 {
                bananas_map.entry((dc[0], dc[1], dc[2], dc[3])).and_modify(|e| {
                    if !e.iter().any(|t| t.0 == id) {
                        e.push((id, num % 10));
                    }
                }).or_insert(vec![(id, num % 10)]);
            }

            prev_num = num;
        }
    }

    let max: u64 = bananas_map.into_values()
        .map(|v| v.iter().map(|t| t.1).sum::<u64>()).max().unwrap();

    // 4.57s
    println!("{}\nTime: {:.2?}", max, start.elapsed());
}