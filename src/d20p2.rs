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

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn get_possible_squares(pos: (i32, i32)) -> [(i32, i32); 836] { // 836 = 4 * sum(2..20)
    let mut squares = [(-1, -1); 836];
    let (y, x) = pos;
    let mut i = 0;
    for d in 2..21 {
        let mut shift1 = d;
        let mut shift2 = 0;
        for _ in 0..d {
            squares[i] = (y + shift1, x + shift2);
            squares[i+1] = (y - shift2, x + shift1);
            squares[i+2] = (y - shift1, x - shift2);
            squares[i+3] = (y + shift2, x - shift1);
            
            shift1 -= 1;
            shift2 += 1;
            i += 4;
        }
    }
    squares
}

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
        for pos2 in get_possible_squares(pos1) {
            if positions.contains_key(&pos2) {
                if *positions.get(&pos1).unwrap() + 100 + dist(pos1, pos2) <= *positions.get(&pos2).unwrap() {
                    cheat_count += 1;
                }
            }
        }
    }

    // 6.28s -> 3.25s and definitely scales better as the track gets bigger
    println!("{}\nTime: {:.2?}", cheat_count, start.elapsed());
}