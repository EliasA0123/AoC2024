use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

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

fn get_all_paths(prev_instr: (i32, i32), new_instr: (i32, i32), locs: &HashMap<char, (i32, i32)>) -> HashSet<Vec<char>> {
    // println!("get_all_paths: {:?} -> {:?}", prev_instr, new_instr);
    if dist(prev_instr, new_instr) == 0 {
        return HashSet::from([vec!['A']]);
    }

    let mut all_paths: HashSet<Vec<char>> = HashSet::new();
    let dir_y = if new_instr.0 == prev_instr.0 {0} else if new_instr.0 > prev_instr.0 {1} else {-1};
    let dir_x = if new_instr.1 == prev_instr.1 {0} else if new_instr.1 > prev_instr.1 {1} else {-1};
    if dir_y != 0 && locs.values().any(|v| *v == (prev_instr.0 + dir_y, prev_instr.1)) {
        for endpath in get_all_paths((prev_instr.0 + dir_y, prev_instr.1), new_instr, locs) {
            let mut path = vec![match dir_y {
                -1 => '^',
                1 => 'v',
                _ => ' '
            }];
            path.extend(endpath);
            all_paths.insert(path);
        }
    }
    if dir_x != 0 && locs.values().any(|v| *v == (prev_instr.0, prev_instr.1 + dir_x)) {
        for endpath in get_all_paths((prev_instr.0, prev_instr.1 + dir_x), new_instr, locs) {
            let mut path = vec![match dir_x {
                -1 => '<',
                1 => '>',
                _ => ' '
            }];
            path.extend(endpath);
            all_paths.insert(path);
        }
    }
    
    // println!("result: {:?}", all_paths);
    all_paths
}

fn get_best_path_len(prev_instr: (i32, i32), new_instr: (i32, i32), num_locs: &HashMap<char, (i32, i32)>, dir_locs: &HashMap<char, (i32, i32)>, known_lens: &mut HashMap<((i32, i32), (i32, i32), usize), usize>, depth: usize) -> usize {
    // println!("get_best_path_len: {:?} -> {:?} (depth {})", prev_instr, new_instr, depth);
    let opts: HashSet<Vec<char>>;
    if depth == 0 {
        opts = get_all_paths(prev_instr, new_instr, num_locs);
    } else {
        opts = get_all_paths(prev_instr, new_instr, dir_locs);
    }
    // println!("{:?}", opts);

    if depth == 25 {
        // println!("min: {}", opts.iter().map(|path| path.len()).min().unwrap());
        return opts.into_iter().map(|path| path.len()).min().unwrap();
    }

    let mut min = usize::MAX;
    for opt in opts {
        let mut prev_instr = *dir_locs.get(&'A').unwrap();
        let mut len = 0;
        for instr in opt {
            let instr = *dir_locs.get(&instr).unwrap();
            if let Some(l) = known_lens.get(&(prev_instr, instr, depth)) {
                len += *l;
            }
            else {
                let instr_len = get_best_path_len(prev_instr, instr, num_locs, dir_locs, known_lens, depth + 1);
                // println!("{:?} -> {:?} = {}", prev_instr, instr, instr_len);
                known_lens.insert((prev_instr, instr, depth), instr_len);
                len += instr_len;
            }
            prev_instr = instr;
        }
        if len < min {
            min = len
        }
    }
    return min;
}

// fn expand(path: String, expansions: &HashMap<(char, char), &str>) -> String {
//     let mut expanded = String::new();
//     let mut prev_dir = 'A';
//     for dir in path.chars() {
//         expanded += *expansions.get(&(prev_dir, dir)).unwrap();
//         prev_dir = dir;
//     }
//     expanded
// }

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut num_locs: HashMap<char, (i32, i32)> = HashMap::new();
    num_locs.insert('7', (0, 0));
    num_locs.insert('8', (0, 1));
    num_locs.insert('9', (0, 2));
    num_locs.insert('4', (1, 0));
    num_locs.insert('5', (1, 1));
    num_locs.insert('6', (1, 2));
    num_locs.insert('1', (2, 0));
    num_locs.insert('2', (2, 1));
    num_locs.insert('3', (2, 2));
    num_locs.insert('0', (3, 1));
    num_locs.insert('A', (3, 2));

    let mut dir_locs: HashMap<char, (i32, i32)> = HashMap::new();
    dir_locs.insert('^', (0, 1));
    dir_locs.insert('A', (0, 2));
    dir_locs.insert('<', (1, 0));
    dir_locs.insert('v', (1, 1));
    dir_locs.insert('>', (1, 2));

    let mut known_lens = HashMap::new();

    let mut complexity = 0;
    for code in input.lines() {
        let code_num: usize = code[0..code.len()-1].parse().unwrap();

        let mut prev_instr = *num_locs.get(&'A').unwrap();
        let mut len = 0;
        for instr in code.chars() {
            let instr = *num_locs.get(&instr).unwrap();
            len += get_best_path_len(prev_instr, instr, &num_locs, &dir_locs, &mut known_lens, 0);
            prev_instr = instr;
        }

        // println!("{}, {}", code_num, len);
        complexity += len * code_num;
    }

    println!("{}\nTime: {:.2?}", complexity, start.elapsed());
}