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

    let mut safe_count = 0;
    'lines: for line in input {
        let mut levels = line.split(" ")
            .map(|s| s.parse::<i32>().unwrap());

        let first_num: i32 = levels.next().unwrap();
        let mut prev_num: i32 = levels.next().unwrap();

        if (first_num - prev_num).abs() > 3 || first_num == prev_num {
            continue 'lines;
        }

        let mut prev_comp = prev_num > first_num;

        for num in levels {
            if (num - prev_num).abs() > 3
                || num == prev_num
                || (num > prev_num) != prev_comp
            {
                continue 'lines;
            }
            prev_comp = num > prev_num;
            prev_num = num;
        }
        safe_count += 1;
    }
    println!("{}", safe_count);
}