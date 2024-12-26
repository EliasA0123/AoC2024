use std::collections::HashMap;
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

    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (cpu1, cpu2) = (&line[0..2], &line[3..5]);
        links.entry(cpu1).and_modify(|e| e.push(cpu2)).or_insert(vec![cpu2]);
        links.entry(cpu2).and_modify(|e| e.push(cpu1)).or_insert(vec![cpu1]);
    }

    let mut count = 0;
    for (&cpu1, linked) in links.iter() {
        for i2 in 0..linked.len() {
            let cpu2 = linked[i2];
            for i3 in i2+1..linked.len() {
                let cpu3 = linked[i3];
                if links.get(&cpu2).unwrap().contains(&cpu3) {
                    if &cpu1[0..1] == "t" || &cpu2[0..1] == "t" || &cpu3[0..1] == "t" {
                        count += 1;
                    }
                }
            }
        }
    }
    count /= 3;

    println!("{}\nTime: {:.2?}", count, start.elapsed());
}