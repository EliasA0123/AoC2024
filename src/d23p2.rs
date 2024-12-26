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

// Bron Kerbosch, still dont <em>entirely</em> understand this, but I get what it's all meant to do
// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn get_all_groups(links: &HashMap<String, Vec<String>>, group: Vec<String>, mut opts: VecDeque<String>, mut checked: Vec<String>, all_groups: &mut Vec<Vec<String>>) {
    if opts.len() == 0 && checked.len() == 0 {
        all_groups.push(group.clone());
    }

    for _ in 0..opts.len() {
        let opt = opts.pop_front().unwrap();

        let mut new_group = group.clone();
        new_group.push(opt.clone());

        let neighbors = links.get(&opt).unwrap();
        let new_opts: VecDeque<String> = opts.clone().into_iter().filter(|cpu| neighbors.contains(cpu)).collect();
        let new_checked: Vec<String> = checked.clone().into_iter().filter(|cpu| neighbors.contains(cpu)).collect();

        get_all_groups(links, new_group, new_opts, new_checked, all_groups);

        checked.push(opt);
    }
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let mut links: HashMap<String, Vec<String>> = HashMap::new();
    let mut cpus = VecDeque::new();

    for ln in input.lines() {
        if !cpus.contains(&ln[0..2].to_owned()) {
            cpus.push_back(ln[0..2].to_owned());
        }
        if !cpus.contains(&ln[3..5].to_owned()) {
            cpus.push_back(ln[3..5].to_owned());
        }
        links.entry(ln[0..2].to_owned())
            .and_modify(|e| {e.push(ln[3..5].to_owned());})
            .or_insert(vec![ln[3..5].to_owned()]);
        links.entry(ln[3..5].to_owned())
            .and_modify(|e| {e.push(ln[0..2].to_owned());})
            .or_insert(vec![ln[0..2].to_owned()]);
    }

    let mut all_groups: Vec<Vec<String>> = Vec::new();
    get_all_groups(&links, Vec::new(), cpus, Vec::new(), &mut all_groups);

    let mut biggest = all_groups.into_iter().max_by_key(|g| g.len()).unwrap();
    biggest.sort();
    let pass = biggest.join(",");

    println!("{}\nTime: {:.2?}", pass, start.elapsed());
}