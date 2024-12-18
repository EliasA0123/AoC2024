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

    let games = input.split("\n\n");

    let mut cost = 0;

    let mut count = 0;
    for game in games {
        count += 1;
        let lines = game.lines().collect::<Vec<&str>>();
        let (line_a, line_b, line_p) = (lines[0], lines[1], lines[2]);

        let line_a = line_a.split(": X+").collect::<Vec<&str>>()[1];
        let a: Vec<i64> = line_a.split(", Y+").map(|s| s.parse::<i64>().unwrap()).collect();

        let line_b = line_b.split(": X+").collect::<Vec<&str>>()[1];
        let b: Vec<i64> = line_b.split(", Y+").map(|s| s.parse::<i64>().unwrap()).collect();

        let line_p = line_p.split(": X=").collect::<Vec<&str>>()[1];
        let prize: Vec<i64> = line_p.split(", Y=").map(|s| s.parse::<i64>().unwrap() + 10000000000000).collect();

        if (prize[1] * a[0] - prize[0] * a[1]) % (b[1] * a[0] - b[0] * a[1]) == 0 {
            let nums_b = (prize[1] * a[0] - prize[0] * a[1]) / (b[1] * a[0] - b[0] * a[1]);
            if (prize[0] - nums_b * b[0]) % a[0] == 0 {
                // I spent an hour staring at my screen because I didn't realize that this could happen.
                // Lesson: the more cautiously you code the more sleep you get
                let nums_a = (prize[0] - nums_b * b[0]) / a[0];
                cost += 3 * nums_a + nums_b;
            }
        }
    }
    println!("{}\nTime: {:.2?}", cost, start.elapsed());
}