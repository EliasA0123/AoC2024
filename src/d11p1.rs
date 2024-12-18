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

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut stones: Vec<u64> = input.split(" ")
        .map(|s| s.parse::<u64>().unwrap()).collect();

    for _ in 0..25 {
        let mut next_stones = Vec::new();
        for i in 0..stones.len() {
            if stones[i] == 0 {
                next_stones.push(1);
            } else {
                let str_stone = stones[i].to_string();
                if str_stone.len() % 2 == 0 {
                    next_stones.push(str_stone[..str_stone.len() / 2].parse::<u64>().unwrap());
                    next_stones.push(str_stone[str_stone.len() / 2..].parse::<u64>().unwrap());
                } else {
                    next_stones.push(stones[i] * 2024);
                }
            }
        }
        stones = next_stones;
    }

    println!("{}\nTime: {:.2?}", stones.len(), start.elapsed());
}