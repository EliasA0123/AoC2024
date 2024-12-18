use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::HashSet;

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

fn is_on_map(y: i32, x: i32, sizey: usize, sizex: usize) -> bool {
    y >= 0 && y < sizey as i32 && x >= 0 && x < sizex as i32
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let sizey = grid.len();
    let sizex = grid[0].len();

    let mut nodes = HashSet::new();
    for y1 in 0..sizey {
        for x1 in 0..sizex {
            if grid[y1][x1] != '.' {
                let freq = grid[y1][x1];
                for y2 in 0..sizey {
                    for x2 in 0..sizex {
                        if grid[y2][x2] == freq && (y1 != y2 || x1 != x2) {
                            let y1 = y1 as i32;
                            let x1 = x1 as i32;
                            let y2 = y2 as i32;
                            let x2 = x2 as i32;
                            if is_on_map(y1 - (y2 - y1), x1 - (x2 - x1), sizey, sizex) {
                                nodes.insert((y1 - (y2 - y1), x1 - (x2 - x1)));
                            }
                            if is_on_map(y2 + (y2 - y1), x2 + (x2 - x1), sizey, sizex) {
                                nodes.insert((y2 + (y2 - y1), x2 + (x2 - x1)));
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}\nTime: {:.2?}", nodes.len(), start.elapsed());
}