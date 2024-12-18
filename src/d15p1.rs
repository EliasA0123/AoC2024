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

fn can_move(grid: &Vec<Vec<char>>, y: i32, x: i32, dir: (i32, i32)) -> bool {
    if is_on_map(y + dir.0, x + dir.1, grid.len(), grid[0].len()) {
        // println!("{}, {}", y + dir.0, x + dir.1);
        return match grid[(y + dir.0) as usize][(x + dir.1) as usize] {
            '.' => true,
            '#' => false,
            'O' => {
                // println!("box");
                can_move(grid, y + dir.0, x + dir.1, dir)
            },
            _ => false // should never happen
        }
    }
    return false;
}

fn move_thing(grid: &mut Vec<Vec<char>>, y: i32, x: i32, dir: (i32, i32)) {
    let thing = grid[y as usize][x as usize];
    if grid[(y + dir.0) as usize][(x + dir.1) as usize] == 'O' {
        move_thing(grid, y + dir.0, x + dir.1, dir);
    }
    grid[(y + dir.0) as usize][(x + dir.1) as usize] = thing;
    grid[y as usize][x as usize] = '.';
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut input = input.split("\n\n");
    let grid_str = input.next().unwrap();
    let moves = input.next().unwrap().chars();

    let mut grid: Vec<Vec<char>> = grid_str.lines()
        .map(|ln| ln.chars().collect::<Vec<char>>()).collect();

    let sizey = grid.len();
    let sizex = grid[0].len();

    let mut robot_y = -1;
    let mut robot_x = -1;
    for y in 0..sizey {
        for x in 0..sizex {
            if grid[y][x] == '@' {
                robot_y = y as i32;
                robot_x = x as i32;
            }
        }
    }

    for dir in moves {
        let dir = match dir {
            '<' => (0, -1),
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => continue
        };

        if can_move(&grid, robot_y, robot_x, dir) {
            move_thing(&mut grid, robot_y, robot_x, dir);
            robot_y += dir.0;
            robot_x += dir.1;
        }

        // println!("{:?}", dir);
        // let mut printstr = String::new();
        // for row in &grid {
        //     for char in row {
        //         printstr += char.to_string().as_str();
        //     }
        //     printstr += "\n";
        // }
        // print!("{}", printstr);
    }

    let mut count = 0;
    for y in 0..sizey {
        for x in 0..sizex {
            if grid[y][x] == 'O' {
                count += 100 * y + x;
            }
        }
    }

    println!("{}", count);
    println!("Time: {:.2?}", start.elapsed());
}