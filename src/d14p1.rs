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

fn quad(p: Vec<i32>, size: Vec<i32>) -> Option<usize> {
    let mut quad = 0;
    for i in 0..2 {
        if size[i] % 2 == 0 {
            if p[i] < size[i] / 2 {
                quad += i + 1;
            }
        } else {
            if p[i] == size[i] / 2 {
                return None;
            } else if p[i] < size[i] / 2 {
                quad += i + 1;
            }
        }
    }
    return Some(quad);
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    // let mut grid: Vec<Vec<char>> = Vec::new();
    // for line in input.lines() {
    //     grid.push((0..11).map(|_| '.'));
    // }
    // let sizey = grid.len();
    // let sizex = grid[0].len();

    let input = input.lines();
    let size = vec![101, 103];

    let mut quad_counts = vec![0, 0, 0, 0];
    for robot in input {
        let p_and_v: Vec<&str> = robot.split(" v=").collect();
        let p = p_and_v[0].split("=").collect::<Vec<&str>>()[1];
        let p: Vec<i32> = p.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        let v: Vec<i32> = p_and_v[1].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        // println!("{}, {}, {}, {}", p[0], p[1], v[0], v[1]);

        let mut final_p = Vec::new();
        for i in 0..p.len() {
            let mut final_coord = (p[i] + 100 * v[i]) % size[i];
            if final_coord < 0 {
                final_coord += size[i];
            }
            final_p.push(final_coord);
        }
        // print!("{:?} ", final_p);

        match quad(final_p, size.clone()) {
            Some(i) => {
                quad_counts[i] += 1;
                // println!("in quad {}", i);
            },
            None => ()
        }
    }

    let product: i32 = quad_counts.into_iter().product();
    println!("{}\nTime: {:.2?}", product, start.elapsed());
}