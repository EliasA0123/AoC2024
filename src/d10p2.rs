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

fn is_on_map(y: i32, x: i32, sizey: usize, sizex: usize) -> bool {
    y >= 0 && y < sizey as i32 && x >= 0 && x < sizex as i32
}

fn search(grid: &Vec<Vec<usize>>, y: i32, x: i32, height: usize) -> u64 {
    let mut paths = 0;
    for dir in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        if is_on_map(y + dir.0, x + dir.1, grid.len(), grid[0].len()) {
            if grid[(y + dir.0) as usize][(x + dir.1) as usize] == height + 1 {
                if height + 1 == 9 {
                    paths += 1
                }
                else {
                    paths += search(grid, y + dir.0, x + dir.1, height + 1);
                }
            }
        }
    }
    return paths;
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().map(|s| s.to_digit(10).unwrap() as usize).collect());
    }
    let sizey = grid.len();
    let sizex = grid[0].len();

    let mut rating = 0;
    for y in 0..sizey {
        for x in 0..sizex {
            if grid[y][x] == 0 {
                rating += search(&grid, y as i32, x as i32, 0);
            }
        }
    }

    println!("{}\nTime: {:.2?}", rating, start.elapsed());
}