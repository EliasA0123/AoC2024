use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::{HashMap, VecDeque};

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

// struct Pose {
//     pos: (i32, i32),
//     dir: (i32, i32),
// }

// fn get_scores(grid: &Vec<Vec<char>>, y: i32, x: i32, facing: (i32, i32), mut visited: Vec<(i32, i32)>, score: i32) -> Vec<i32> {
//     println!("({}, {}): {}", y, x, visited.len());
//     visited.push((y, x));
//     let mut scores = Vec::new();
//     for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
//         if !visited.contains(&(y + dir.0, x + dir.1)) {
//             let new_score;
//             if dir == facing {
//                 new_score = score + 1;
//             } else {
//                 new_score = score + 1001;
//             }

//             if grid[(y + dir.0) as usize][(x + dir.1) as usize] == 'E' {
//                 scores.push(new_score);
//             }
//             else if grid[(y + dir.0) as usize][(x + dir.1) as usize] == '.' {
//                 scores.extend(get_scores(grid, y + dir.0, x + dir.1, dir, visited.clone(), new_score).into_iter());
//             }
//         }
//     }
//     // println!("({}, {}): {:?}", y, x, scores);
//     scores
// }

fn main() {
    let input = get_input();
    let start = Instant::now();

    let grid: Vec<Vec<char>> = input.lines()
        .map(|ln| ln.chars().collect::<Vec<char>>()).collect();

    let sizey = grid.len();
    let sizex = grid[0].len();

    let mut start_y = -1;
    let mut start_x = -1;
    let mut end_y = -1;
    let mut end_x = -1;
    for y in 0..sizey {
        for x in 0..sizex {
            if grid[y][x] == 'S' {
                start_y = y as i32;
                start_x = x as i32;
            }
            else if grid[y][x] == 'E' {
                end_y = y as i32;
                end_x = x as i32;
            }
        }
    }

    let mut scores = HashMap::new();
    scores.insert(((start_y, start_x), (0, 1)), 0);
    for dir in [(1, 0), (-1, 0), (0, -1)] {
        scores.insert(((start_y, start_x), dir), 1000);
    }

    let mut queue: VecDeque<((i32, i32), (i32, i32))> = VecDeque::new();
    queue.push_back(((start_y, start_x), (0, 1)));
    for dir in [(1, 0), (-1, 0), (0, -1)] {
        queue.push_back(((start_y, start_x), dir));
    }

    while queue.len() > 0 {
        let entry = queue.pop_front().unwrap();
        let &score = scores.get(&entry).unwrap();

        let ((y, x), (dir_y, dir_x)) = entry;
        if grid[(y + dir_y) as usize][(x + dir_x) as usize] != '#' {
            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                match scores.get(&((y + dir_y, x + dir_x), dir)) {
                    Some(&old_score) => {
                        if old_score > score + 1 && dir == (dir_y, dir_x) {
                            scores.insert(((y + dir_y, x + dir_x), dir), score + 1);
                            if !queue.contains(&((y + dir_y, x + dir_x), dir)) {
                                queue.push_back(((y + dir_y, x + dir_x), dir));
                            }
                        }
                        else if old_score > score + 1001 {
                            scores.insert(((y + dir_y, x + dir_x), dir), score + 1001);
                            if !queue.contains(&((y + dir_y, x + dir_x), dir)) {
                                queue.push_back(((y + dir_y, x + dir_x), dir));
                            }
                        }
                    },
                    None => {
                        if dir == (dir_y, dir_x) {
                            scores.insert(((y + dir_y, x + dir_x), dir), score + 1);
                        } else {
                            scores.insert(((y + dir_y, x + dir_x), dir), score + 1001);
                        }
                        if !queue.contains(&((y + dir_y, x + dir_x), dir)) {
                            queue.push_back(((y + dir_y, x + dir_x), dir));
                        }
                    }
                }
            }
        }
    }

    let score = [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter()
        .map(|dir| *scores.get(&((end_y, end_x), dir)).unwrap()).min().unwrap();

    let mut good_squares = vec![(end_y, end_x)];
    let mut antiscores = HashMap::new();

    for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        antiscores.insert(((end_y, end_x), dir), score);
        if scores.contains_key(&((end_y - dir.0, end_x - dir.1), dir)) {
            queue.push_back(((end_y - dir.0, end_x - dir.1), dir));
        }
    }

    while queue.len() > 0 {
        let entry = queue.pop_front().unwrap();
        let score = *scores.get(&entry).unwrap();
        let ((y, x), (dir_y, dir_x)) = entry;
        let antiscore = *antiscores.get(&((y + dir_y, x + dir_x), (dir_y, dir_x))).unwrap();

        if !good_squares.contains(&(y, x)) && score < antiscore {
            good_squares.push((y, x));
            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if dir == (dir_y, dir_x) {
                    antiscores.insert(((y, x), dir), antiscore - 1);
                } else {
                    antiscores.insert(((y, x), dir), antiscore - 1001);
                }

                if scores.contains_key(&((y - dir.0, x - dir.1), dir)) && !good_squares.contains(&(y - dir.0, x - dir.1)) {
                    // println!("{:?}", ((y - dir.0, x - dir.1), dir));
                    queue.push_back(((y - dir.0, x - dir.1), dir));
                }
            }
        }
    }

    println!("{}\nTime: {:.2?}", good_squares.len(), start.elapsed());
    // println!("{:?}", good_squares);
}