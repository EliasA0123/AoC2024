use std::collections::{HashMap, VecDeque};
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

    let end = 70;

    let all_bad_coords: Vec<(i32, i32)> = input.lines().map(
        |ln| ln.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|v| (v[1], v[0])).collect();

    let mut min_bytes = 0usize;
    let mut max_bytes = all_bad_coords.len();
    'byte: while min_bytes != max_bytes {
        let num_bytes = (min_bytes + max_bytes) / 2;
        println!("{}, {} | {}", min_bytes, max_bytes, num_bytes);
        let bad_coords = &all_bad_coords[0..num_bytes];

        let mut dists = HashMap::new();    
        let mut queue = VecDeque::new();
        queue.push_back(((0, 0), 0));

        while queue.len() > 0 {
            let (pos, dist) = queue.pop_front().unwrap();
            if !dists.contains_key(&pos) {
                dists.insert(pos, dist);
            }

            if pos == (end, end) {
                min_bytes = num_bytes + 1;
                continue 'byte;
            }

            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let newpos = (pos.0 + dir.0, pos.1 + dir.1);
                if newpos.0 >= 0 && newpos.0 <= end && newpos.1 >= 0 && newpos.1 <= end
                && !dists.contains_key(&newpos)
                && !bad_coords.contains(&newpos)
                && !queue.contains(&(newpos, dist + 1)) {
                    queue.push_back((newpos, dist + 1));
                }
            }
        }
        max_bytes = num_bytes;
    }

    println!("({}, {}) {}", all_bad_coords[max_bytes - 1].1, all_bad_coords[max_bytes - 1].0, max_bytes - 1);
    println!("\nTime: {:.2?}", start.elapsed());
}