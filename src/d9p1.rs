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

    let input: Vec<usize> = input.chars()
        .map(|s| s.to_digit(10).unwrap())
        .map(|n| n as usize).collect();

    let mut files = Vec::new();
    let mut layout = Vec::new();

    for i in 0..input.len() {
        for _ in 0..input[i] {
            layout.push(i % 2 == 0);
            if i % 2 == 0 {
                files.push(i / 2);
            }
        }
    }

    let mut sum = 0;
    for i in 0..layout.len() {
        if layout[i] {
            sum += i * files.remove(0);
        }
        else {
            sum += i * files.remove(files.len() - 1);
        }
        if files.len() == 0 {
            break;
        }
    }

    println!("{}\nTime: {:.2?}", sum, start.elapsed());
}