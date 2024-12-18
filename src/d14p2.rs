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

fn max_line_len(counts: &HashMap<(i32, i32), i32>) -> i32 {
    let mut maxxest_run = 0;
    for x in 0..101 {
        let mut in_run = false;
        let mut max_run = 0;
        let mut y_start = 0;
        for y in 0..103 {
            if !in_run {
                if *counts.get(&(x, y)).unwrap() != 0 {
                    y_start = y;
                    in_run = true;
                }
            } else {
                if *counts.get(&(x, y)).unwrap() == 0 {
                    if max_run < y - y_start {
                        max_run = y - y_start;
                    }
                    in_run = false;
                }
            }
        }
        if maxxest_run < max_run {
            maxxest_run = max_run;
        }
    }
    maxxest_run
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let input = input.lines();

    let size = vec![101, 103];
    let mut positions = Vec::new();
    let mut velocities = Vec::new();

    let mut counts = HashMap::new();
    for x in 0..size[0] {
        for y in 0..size[1] {
            counts.insert((x, y), 0);
        }
    }

    for robot in input {
        let p_and_v: Vec<&str> = robot.split(" v=").collect();
        let p = p_and_v[0].split("=").collect::<Vec<&str>>()[1];
        let p: Vec<i32> = p.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        let v: Vec<i32> = p_and_v[1].split(",").map(|s| s.parse::<i32>().unwrap()).collect();

        positions.push(p);
        velocities.push(v);
    }

    for p in &positions {
        counts.entry((p[0], p[1])).and_modify(|e| *e += 1);
    }

    // Windows prevented me from trying this god-awful plan :)
    // let mut write_file = match File::open("output.txt") {
    //     Err(why) => panic!("couldn't open file: {}", why),
    //     Ok(file) => file,
    // };

    // let mut print_str = String::new();

    let mut runs = 0;
    let mut order = Vec::new();
    while runs < 10000 { // increased until it worked
        runs += 1;
        for r_i in 0..positions.len() { 
            counts.entry((positions[r_i][0], positions[r_i][1])).and_modify(|e| *e -= 1);
            for i in 0..2 {
                positions[r_i][i] = (positions[r_i][i] + velocities[r_i][i]) % size[i];
                if positions[r_i][i] < 0 {
                    positions[r_i][i] += size[i];
                }
            }
            counts.entry((positions[r_i][0], positions[r_i][1])).and_modify(|e| *e += 1);
        }
        order.push(max_line_len(&counts));

        if runs == 10000 {
            let mut print_str = format!("{}\n", runs);
            for y in 0..size[1] {
                for x in 0..size[0] {
                    let n = *counts.get(&(x, y)).unwrap();
                    if n == 0 {
                        print_str += ".";
                    } else {
                        print_str += "X";
                    }
                }
                print_str += "\n";
            }
            println!("{}", print_str);
        }
    }

    // match write_file.write_all(print_str.as_bytes()) {
    //     Ok(_) => (),
    //     Err(e) => panic!("{}", e)

    let mut max_n = 0;
    let mut max_i = 0;
    for i in 0..order.len() {
        if order[i] > max_n {
            max_n = order[i];
            max_i = i;
        }
    }

    for _ in 0..max_i {
        for r_i in 0..positions.len() { 
            counts.entry((positions[r_i][0], positions[r_i][1])).and_modify(|e| *e -= 1);
            for i in 0..2 {
                positions[r_i][i] = (positions[r_i][i] + velocities[r_i][i]) % size[i];
                if positions[r_i][i] < 0 {
                    positions[r_i][i] += size[i];
                }
            }
            counts.entry((positions[r_i][0], positions[r_i][1])).and_modify(|e| *e += 1);
        }
    }

    println!("{}: {}", max_i + 1, max_n);
    println!("Time: {:.2?}", start.elapsed());
}