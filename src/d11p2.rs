use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
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

fn split(n: u64) -> Option<(u64, u64)> {
    let mut len = 0;
    let mut splitter = 1;
    while splitter <= n {
        len += 1;
        splitter *= 10;
    }
    if len % 2 != 0 {
        return None;
    }
    splitter = (splitter as f64).sqrt() as u64;
    return Some((n / splitter, n % splitter));
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in input.split(" ").map(|s| s.parse::<u64>().unwrap()) {
        stones.entry(stone)
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }

    for _ in 0..75 {
        let mut next_stones = HashMap::new();
        for (&stone, &count) in stones.iter() {
            if stone == 0 {
                next_stones.entry(1)
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
            } else {
                match split(stone) {
                    Some((front, back)) => {
                        next_stones.entry(front)
                            .and_modify(|cnt| *cnt += count)
                            .or_insert(count);
                        next_stones.entry(back)
                            .and_modify(|cnt| *cnt += count)
                            .or_insert(count);
                    },
                    None => {
                        next_stones.entry(stone * 2024)
                            .and_modify(|cnt| *cnt += count)
                            .or_insert(count);
                    }
                }
            }
        }
        stones = next_stones;
    }

    let count: u64 = stones.into_values().sum();

    println!("{}\nTime: {:.2?}", count, start.elapsed());
}