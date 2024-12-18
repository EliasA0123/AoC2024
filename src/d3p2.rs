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

    let dos: Vec<usize> = input.match_indices("do()").map(|(i, _s)| i).collect();
    let donts: Vec<usize> = input.match_indices("don't()").map(|(i, _s)| i).collect();

    let mut sum = 0;
    let mut current_index = 0;
    loop {
        let input = &input[current_index..];
        let start = input.find("mul(");
        if start.is_some() {
            let start = start.unwrap() + 4;
            
            let mut do_dont_ind = start + current_index;
            loop {
                do_dont_ind -= 1;
                if donts.contains(&do_dont_ind) {
                    break;
                }
                if dos.contains(&do_dont_ind) || do_dont_ind == 0 {
                    let end = input[start..].find(")");
                    if end.is_some() {
                        let end = end.unwrap() + start;
                        
                        let params = &input[start..end];
                        let comma = params.find(",");
                        if comma.is_some() {
                            let comma = comma.unwrap() + start;
                            if comma - start <= 4 && end - comma <= 4 {
                                let num1 = input[start..comma].parse::<u32>();
                                let num2 = input[comma + 1..end].parse::<u32>();

                                if num1.is_ok() && num2.is_ok() {
                                    let num1 = num1.unwrap();
                                    let num2 = num2.unwrap();
                                    sum += num1 * num2;
                                }
                            }
                        }
                    }
                    break;
                }
            }
            
            current_index += start + 4;
        }
        else {
            break;
        }
    }
    println!("{}", sum);
}