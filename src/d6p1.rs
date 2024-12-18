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
    let mut x: i32 = -1;
    let mut y: i32 = -1;
    for (i, line) in input.lines().enumerate() {
        if let Some(j) = line.find("^") {
            y = i as i32;
            x = j as i32;
        }
        grid.push(line.chars().collect());
    }
    let sizey = grid.len();
    let sizex = grid[0].len();
    
    let mut dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter().cycle();
    let mut dir = dirs.next().unwrap();

    let mut visited = HashSet::new();
    loop {
        if is_on_map(y + dir.0, x + dir.1, sizey, sizex) {
            if grid[(y + dir.0) as usize][(x + dir.1) as usize] == '#' {
                dir = dirs.next().unwrap();
            }
            else {
                y += dir.0;
                x += dir.1;
                visited.insert((y, x));
            }
        }
        else {
            break;
        }
    }
    println!("{}\nTime: {:.2?}", visited.len(), start.elapsed());
}