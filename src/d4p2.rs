use std::fs::File;
use std::io::prelude::*;

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
    let input = input.lines();

    let mut grid: Vec<Vec<char>> = vec![];
    for line in input {
        grid.push(line.chars().collect());
    }

    let mut count = 0;
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            if grid[y][x] == 'A' {
                if (
                    grid[y+1][x+1] == 'M' && grid[y-1][x-1] == 'S'
                    || grid[y+1][x+1] == 'S' && grid[y-1][x-1] == 'M'
                ) && (
                    grid[y-1][x+1] == 'M' && grid[y+1][x-1] == 'S'
                    || grid[y-1][x+1] == 'S' && grid[y+1][x-1] == 'M'
                ) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}