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

    let mut input = input.split("\n\n");
    let towels: Vec<&str> = input.next().unwrap().split(", ")
        .collect();

    let patterns: Vec<&str> = input.next().unwrap().lines().collect();

    let mut count: u64 = 0;
    for pat in patterns {
        let mut counts_to: Vec<u64> = vec![0; pat.len() + 1];
        counts_to[0] = 1;

        for i in 0..pat.len() {
            for &towel in &towels {
                if i + towel.len() <= pat.len() {
                    if towel == &pat[i..(i + towel.len())] {
                        counts_to[i + towel.len()] += counts_to[i];
                    }
                }
            }
        }

        count += counts_to[pat.len()];
    }

    println!("{}\nTime: {:.2?}", count, start.elapsed());
}