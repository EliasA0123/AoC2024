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

// fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
//     (a.0 - b.0).abs() + (a.1 - b.1).abs()
// }

fn main() {
    let input = get_input();
    let start = Instant::now();

    let grid: Vec<Vec<char>> = input.lines()
        .map(|ln| ln.chars().collect::<Vec<char>>()).collect();

    let sizey = grid.len();
    let sizex = grid[0].len();

    let mut positions: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (0, 0);
    for y in 0..sizey {
        for x in 0..sizex {
            if grid[y][x] == 'S' {
                positions.insert((y as i32, x as i32), 0);
                pos = (y as i32, x as i32);
            }
        }
    }

    let mut length = 1;
    'track: loop {
        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let newpos = (pos.0 + dir.0, pos.1 + dir.1);
            if grid[newpos.0 as usize][newpos.1 as usize] != '#'
            && !positions.contains_key(&newpos) {
                positions.insert(newpos, length);
                pos = newpos;
                length += 1;
                if grid[newpos.0 as usize][newpos.1 as usize] == 'E' {
                    break 'track;
                }
            }
        }
    }

    let mut cheat_count = 0;
    for &pos1 in positions.keys() {
        for (yshift, xshift) in [(-2, 0), (-1, -1), (0, -2), (1, -1), (2, 0), (1, 1), (0, 2), (-1, 1)] {
            let pos2 = (pos1.0 + yshift, pos1.1 + xshift);
            if positions.contains_key(&pos2) {
                if *positions.get(&pos1).unwrap() + 102 <= *positions.get(&pos2).unwrap() {
                    cheat_count += 1;
                }
            }
        }
    }

    println!("{}\nTime: {:.2?}", cheat_count, start.elapsed());
}