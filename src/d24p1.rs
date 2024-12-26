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

    let mut input = input.split("\n\n");
    let config = input.next().unwrap();
    let mut values = HashMap::new();
    for ln in config.lines() {
        values.insert(&ln[0..3], if &ln[5..6] == "1" {true} else {false});
    }

    let ops = input.next().unwrap();
    let mut operations: HashMap<(&str, &str), (&str, &str)> = HashMap::new();
    for ln in ops.lines() {
        let ln: Vec<&str> = ln.split(" ").collect();
        operations.insert((ln[1], ln[4]), (ln[0], ln[2]));
    }

    while operations.len() > 0 {
        let mut to_remove = Vec::new();
        for ((op, res), (val1, val2))  in operations.iter() {
            if values.contains_key(val1) && values.contains_key(val2) {
                let v1 = *values.get(val1).unwrap();
                let v2 = *values.get(val2).unwrap();
                values.insert(*res, match *op {
                    "AND" => v1 && v2,
                    "OR" => v1 || v2,
                    "XOR" => v1 != v2,
                    _ => panic!("something went real wrong")
                });
                to_remove.push((*op, *res));
            }
        }
        for k in &to_remove {
            operations.remove(k);
        }
    }

    let mut num: u64 = 0;
    let mut z_i = 0;
    loop {
        if let Some(b) = values.get(&format!("z{:02}", z_i).as_str()) {
            if *b {
                num += 1 << z_i;
            }
        } else {
            break;
        }
        z_i += 1;
    }

    println!("{}\nTime: {:.2?}", num, start.elapsed());
}