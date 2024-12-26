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

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut sizey = 0;
    let mut sizex = 0;

    for schem in input.split("\n\n") {
        let is_lock = &schem[0..1] == "#";
        let schem: Vec<Vec<char>> = schem.lines()
            .map(|ln| ln.chars().collect::<Vec<char>>()).collect();
        if sizey == 0 {
            sizey = schem.len();
            sizex = schem[0].len();
        }
        let schem: Vec<usize> = (0..sizex)
            .map(|x| (0..sizey).filter(|&y| schem[y][x] == '#').count()).collect();
        match is_lock {
            true => locks.push(schem),
            false => keys.push(schem)
        };
    }

    let mut count = 0;
    for lock in &locks {
        'pair: for key in &keys {
            for x in 0..sizex {
                if lock[x] + key[x] > sizey {
                    continue 'pair;
                }
            }
            count += 1;
        }
    }

    println!("{}\nTime: {:.2?}", count, start.elapsed());
}