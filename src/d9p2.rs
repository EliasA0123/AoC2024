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

#[derive(Clone, Debug)]
struct FileBlock {
    id: usize,
    len: usize,
}

#[derive(Clone, Debug)]
struct FreeBlock {
    len: usize
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let input: Vec<usize> = input.chars()
        .map(|s| s.to_digit(10).unwrap())
        .map(|n| n as usize).collect();

    let mut files: HashMap<usize, FileBlock> = HashMap::new();
    let mut frees: HashMap<usize, FreeBlock> = HashMap::new();

    let mut file_indices = Vec::new();
    let mut free_indices = Vec::new();

    let mut index = 0;
    for i in 0..input.len() {
        if i % 2 == 0 {
            file_indices.push(index);
            files.insert(index, FileBlock {
                id: i / 2,
                len: input[i],
            });
        } else {
            free_indices.push(index);
            frees.insert(index, FreeBlock {
                len: input[i]
            });
        }
        index += input[i];
    }

    file_indices.reverse();
    for &file_ind in &file_indices {
        let file = files.get(&file_ind).unwrap();
        for i in 0..free_indices.len() {
            let free_ind = free_indices[i];
            if free_ind > file_ind {
                break;
            }
            let free = frees.get(&free_ind).unwrap();
            if free.len >= file.len {
                if free.len > file.len {
                    let mut free = free.clone();
                    free.len -= file.len;
                    frees.insert(free_ind + file.len, free);
                    free_indices[i] = free_ind + file.len;
                }
                else {
                    free_indices.remove(i);
                }

                frees.remove(&free_ind);
                frees.insert(file_ind, FreeBlock {len: file.len});

                files.insert(free_ind, file.clone());
                files.remove(&file_ind);

                break;
            }
        }
    }

    let mut sum = 0;
    for (index, file) in files.into_iter() {
        let high_index = index + file.len - 1;
        sum += file.id * (high_index - index + 1) * (index + high_index) / 2;
    }

    println!("{}", sum);
    println!("Time: {:.2?}", start.elapsed()); 
}