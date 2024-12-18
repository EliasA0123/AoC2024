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

fn get_region(grid: &Vec<Vec<char>>, region: &mut HashSet<(usize, usize)>, y: usize, x: usize) {
    region.insert((y, x));
    for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        if is_on_map(y as i32 + dir.0, x as i32 + dir.1, grid.len(), grid[0].len()) {
            if grid[(y as i32 + dir.0) as usize][(x as i32 + dir.1) as usize] == grid[y][x] 
                && !region.contains(&((y as i32 + dir.0) as usize, (x as i32 + dir.1) as usize)) {
                get_region(grid, region, (y as i32 + dir.0) as usize, (x as i32 + dir.1) as usize);
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

    // let mut region_lookup = HashMap::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    for y in 0..sizey {
        for x in 0..sizex {
            if !seen.contains(&(y, x)) {
                let mut region = HashSet::new();
                get_region(&grid, &mut region, y, x);
                for pos in &region {
                    seen.insert(*pos);
                }
                regions.push(region);
            }
        }
    }

    let mut cost = 0;
    for region in &regions {
        let mut perim = 0;
        for &(y, x) in region {
            for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                if is_on_map(y as i32 + dir.0, x as i32 + dir.1, sizey, sizex) {
                    if !region.contains(&((y as i32 + dir.0) as usize, (x as i32 + dir.1) as usize)) {
                        perim += 1;
                    }
                } else {
                    perim += 1;
                }
            }
        }
        cost += perim * region.len();
    }

    println!("{cost}\nTime: {:.2?}", start.elapsed());
}