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

fn next_dir(dir: (i32, i32)) -> (i32, i32) {
    (dir.1, -dir.0)
}

fn is_on_map(y: i32, x: i32, sizey: usize, sizex: usize) -> bool {
    y >= 0 && y < sizey as i32 && x >= 0 && x < sizex as i32
}

fn creates_loop(mut grid: Vec<Vec<char>>, mut prev_poses: HashSet<((i32, i32), (i32, i32))>, add_y: i32, add_x: i32, mut dir: (i32, i32)) -> bool {
    grid[add_y as usize][add_x as usize] = '#';
    let mut y = add_y - dir.0;
    let mut x = add_x - dir.1;
    let sizey = grid.len();
    let sizex = grid[0].len();
    dir = next_dir(dir);
    loop {
        if !is_on_map(y + dir.0, x + dir.1, sizey, sizex) {
            return false;
        }
        if grid[(y + dir.0) as usize][(x + dir.1) as usize] == '#' {
            dir = next_dir(dir);
        }
        else {
            y += dir.0;
            x += dir.1;
            if prev_poses.contains(&((y, x), dir)) {
                return true;
            }
            prev_poses.insert(((y, x), dir));
        }
    }

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
    
    let mut dir = (-1, 0);

    let mut visited = HashSet::new();
    let mut prev_poses = HashSet::new();

    let mut count = 0;
    loop {
        if is_on_map(y + dir.0, x + dir.1, sizey, sizex) {
            if grid[(y + dir.0) as usize][(x + dir.1) as usize] == '#' {
                dir = next_dir(dir);
            }
            else {
                if !visited.contains(&(y + dir.0, x + dir.1)) {
                    if creates_loop(grid.clone(), prev_poses.clone(), y + dir.0, x + dir.1, dir) {
                        count += 1;
                    }
                }
                y += dir.0;
                x += dir.1;
                visited.insert((y, x));
                prev_poses.insert(((y, x), dir));
            }
        }
        else {
            break;
        }
    }
    println!("{}\nTime: {:.2?}", count, start.elapsed());
}