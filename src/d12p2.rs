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

fn get_region(grid: &Vec<Vec<char>>, region: &mut Vec<(i32, i32)>, y: i32, x: i32) {
    region.push((y, x));
    for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        if is_on_map(y + dir.0, x + dir.1, grid.len(), grid[0].len()) {
            if grid[(y + dir.0) as usize][(x + dir.1) as usize] == grid[y as usize][x as usize] 
                && !region.contains(&((y + dir.0), (x + dir.1))) {
                get_region(grid, region, y + dir.0, x + dir.1);
            }
        }
    }
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

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut regions: Vec<Vec<(i32, i32)>> = Vec::new();
    for y in 0..sizey {
        for x in 0..sizex {
            let y = y as i32;
            let x = x as i32;
            if !seen.contains(&(y, x)) {
                let mut region = Vec::new();
                get_region(&grid, &mut region, y, x);
                for &pos in &region {
                    seen.insert(pos);
                }
                regions.push(region);
            }
        }
    }

    let mut cost = 0;
    for region in &regions {
        let mut walls = Vec::new();
        let mut wall_count = 0;

        for &(y, x) in region {
            for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                if is_on_map(y + dir.0, x + dir.1, sizey, sizex) {
                    if !region.contains(&(y + dir.0, x + dir.1)) {
                        walls.push(((y, x), dir));
                    }
                } else {
                    walls.push(((y, x), dir));
                }
            }
        }

        let mut checked = Vec::new();
        for wall in &walls {
            if !checked.contains(wall) {
                wall_count += 1;
                let &((y, x), dir) = wall;
                if dir.0 == 0 {
                    let mut test_y = y + 1;
                    while is_on_map(test_y, x, sizey, sizex) {
                        if walls.contains(&((test_y, x), dir)) {
                            checked.push(((test_y, x), dir));
                        } else {
                            break;
                        }
                        test_y += 1;
                    }
                    test_y = y - 1;
                    while is_on_map(test_y, x, sizey, sizex) {
                        if walls.contains(&((test_y, x), dir)) {
                            checked.push(((test_y, x), dir));
                        } else {
                            break;
                        }
                        test_y -= 1;
                    }
                } else {
                    let mut test_x = x + 1;
                    while is_on_map(y, test_x, sizey, sizex) {
                        if walls.contains(&((y, test_x), dir)) {
                            checked.push(((y, test_x), dir));
                        } else {
                            break;
                        }
                        test_x += 1;
                    }
                    test_x = x - 1;
                    while is_on_map(y, test_x, sizey, sizex) {
                        if walls.contains(&((y, test_x), dir)) {
                            checked.push(((y, test_x), dir));
                        } else {
                            break;
                        }
                        test_x -= 1;
                    }
                }
            }
        }
        cost += wall_count * region.len();
    }

    println!("{cost}\nTime: {:.2?}", start.elapsed());
}