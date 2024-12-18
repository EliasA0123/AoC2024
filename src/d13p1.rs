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

    for game in games {
        let lines = game.lines().collect::<Vec<&str>>();
        let (line_a, line_b, line_p) = (lines[0], lines[1], lines[2]);

        let line_a = line_a.split(": X+").collect::<Vec<&str>>()[1];
        let a: Vec<u32> = line_a.split(", Y+").map(|s| s.parse::<u32>().unwrap()).collect();

        let line_b = line_b.split(": X+").collect::<Vec<&str>>()[1];
        let b: Vec<u32> = line_b.split(", Y+").map(|s| s.parse::<u32>().unwrap()).collect();

        let line_p = line_p.split(": X=").collect::<Vec<&str>>()[1];
        let prize: Vec<u32> = line_p.split(", Y=").map(|s| s.parse::<u32>().unwrap()).collect();

        let mut min_cost = 0;
        for nums_a in 0..100 {
            'num: for nums_b in 0..100 {
                if (0..2).all(|i| nums_a * a[i] + nums_b * b[i] == prize[i]) {
                    if min_cost == 0 {
                        min_cost = 3 * nums_a + nums_b;
                    } else {
                        min_cost = std::cmp::min(min_cost, 3 * nums_a + nums_b);
                    }
                }
            }
        }
        cost += min_cost;

        // let mut max_b = std::cmp::min(prize[0] / b[0], prize[1] / b[1]);
        // if max_b > 100 {
        //     max_b = 100;
        // }
        // loop {
        //     let mut works = true;
        //     for i in 0..2 {
        //         if (prize[i] - (b[i] * max_b)) % a[i] != 0 {
        //             works = false;
        //         }
        //     }
        //     if works {
        //         cost += max_b + 3 * ((prize[0] - (b[0] * max_b)) / a[0]);
        //         break;
        //     }
        //     if max_b == 0 {
        //         break;
        //     }
        //     max_b -= 1;
        // }
    }

    println!("{}\nTime: {:.2?}", cost, start.elapsed());
}