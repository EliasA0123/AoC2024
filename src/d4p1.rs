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
    let dirs: Vec<(isize, isize)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for dir in dirs {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if !(
                    (dir.0 == -1 && y < 3)
                    || (dir.0 == 1 && y > grid.len() - 4)
                    || (dir.1 == -1 && x < 3)
                    || (dir.1 == 1 && x > grid[0].len() - 4)
                ) {
                    if grid[y][x] == 'X' {
                        if grid[(y as isize + dir.0) as usize][(x as isize + dir.1) as usize] == 'M' {
                            if grid[(y as isize + 2*dir.0) as usize][(x as isize + 2*dir.1) as usize] == 'A' {
                                if grid[(y as isize + 3*dir.0) as usize][(x as isize + 3*dir.1) as usize] == 'S' {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}