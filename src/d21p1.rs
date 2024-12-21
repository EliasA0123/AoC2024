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

fn get_all_paths(prev_instr: (i32, i32), new_instr: (i32, i32), locs: &HashMap<char, (i32, i32)>, dirs_to_chars: &HashMap<(i32, i32), char>) -> HashSet<Vec<char>> {
    if dist(prev_instr, new_instr) == 0 {
        return HashSet::from([vec!['A']]);
    }

    let mut all_paths: HashSet<Vec<char>> = HashSet::new();
    let dir_y = if new_instr.0 == prev_instr.0 {0} else if new_instr.0 > prev_instr.0 {1} else {-1};
    let dir_x = if new_instr.1 == prev_instr.1 {0} else if new_instr.1 > prev_instr.1 {1} else {-1};
    if dir_y != 0 && locs.values().any(|v| *v == (prev_instr.0 + dir_y, prev_instr.1)) {
        for endpath in get_all_paths((prev_instr.0 + dir_y, prev_instr.1), new_instr, locs, dirs_to_chars) {
            let mut path = vec![*dirs_to_chars.get(&(dir_y, 0)).unwrap()];
            path.extend(endpath);
            all_paths.insert(path);
        }
    }
    if dir_x != 0 && locs.values().any(|v| *v == (prev_instr.0, prev_instr.1 + dir_x)) {
        for endpath in get_all_paths((prev_instr.0, prev_instr.1 + dir_x), new_instr, locs, dirs_to_chars) {
            let mut path = vec![*dirs_to_chars.get(&(0, dir_x)).unwrap()];
            path.extend(endpath);
            all_paths.insert(path);
        }
    }
    all_paths
}

fn get_best_path_len(prev_instr: (i32, i32), new_instr: (i32, i32), num_locs: &HashMap<char, (i32, i32)>, dir_locs: &HashMap<char, (i32, i32)>, dirs_to_chars: &HashMap<(i32, i32), char>, depth: usize) -> usize {
    let opts: HashSet<Vec<char>>;
    if depth == 0 {
        opts = get_all_paths(prev_instr, new_instr, num_locs, dirs_to_chars);
    } else {
        opts = get_all_paths(prev_instr, new_instr, dir_locs, dirs_to_chars);
    }

    if depth == 2 {
        return opts.into_iter().map(|path| path.len()).min().unwrap();
    }

    return opts.into_iter().map(|opt| {
        let mut opt_min = 0;
        let mut prev_instr = *dir_locs.get(&'A').unwrap();
        for instr in opt {
            let instr = *dir_locs.get(&instr).unwrap();
            opt_min += get_best_path_len(prev_instr, instr, num_locs, dir_locs, dirs_to_chars, depth + 1);
            prev_instr = instr;
        }
        opt_min
    }).min().unwrap();
}

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

    let mut dirs_to_chars = HashMap::new();
    dirs_to_chars.insert((-1, 0), '^');
    dirs_to_chars.insert((0, 0), 'A');
    dirs_to_chars.insert((0, -1), '<');
    dirs_to_chars.insert((1, 0), 'v');
    dirs_to_chars.insert((0, 1), '>');

    let mut complexity = 0;
    for code in input.lines() {
        let code_num: usize = code[0..code.len()-1].parse().unwrap();

        let code = code.chars().map(|c| *num_locs.get(&c).unwrap());
        let mut prev_instr = *num_locs.get(&'A').unwrap();
        let mut len = 0;
        for instr in code {
            len += get_best_path_len(prev_instr, instr, &num_locs, &dir_locs, &dirs_to_chars, 0);
            prev_instr = instr;
        }
        complexity += len * code_num;
    }
    println!("{}\nTime: {:.2?}", complexity, start.elapsed());
}