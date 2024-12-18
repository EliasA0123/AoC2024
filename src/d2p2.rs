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

// fn is_safe(levels: &Vec<&str>, skip_index: Option<usize>) -> bool {
//     let levels = levels.iter();
//     let mut last_num: Option<i32> = None;
//     let mut prev_comp: Option<bool> = None;

//     for (i, level) in levels.enumerate() {
//         if skip_index.is_some() {
//             if i == skip_index.unwrap() {
//                 continue;
//             }
//         }
//         let num: i32 = level.parse().unwrap();
//         if last_num.is_some() {
//             let last_num = last_num.unwrap();
//             if (num - last_num).abs() > 3 || num == last_num {
//                 return false;
//             }
//             if prev_comp.is_some() {
//                 if (num > last_num) != prev_comp.unwrap() {
//                     return false;
//                 }
//             }
//             prev_comp = Some(num > last_num)
//         }
//         last_num = Some(num);
//     }
//     return true;
// }

// fn main() {
//     let input = get_input();
//     let input = input.lines();

//     let start = Instant::now();

//     let mut safe_count = 0;
//     for line in input {
//         let levels: Vec<&str> = line.split(" ").collect();
//         if is_safe(&levels, None) {
//             safe_count += 1;
//         }
//         else {
//             for i in 0..levels.len() {
//                 if is_safe(&levels, Some(i)) {
//                     safe_count += 1;
//                     break;
//                 }
//             }
//         }
//     }
//     println!("{} \n Time: {:.2?}", safe_count, start.elapsed());
// }

/////////////////////////////////////////////////////////////////////////

fn check_deep_levels(levels: &Vec<i32>, mut prev_num: i32, mut prev_comp: bool, mut can_skip: bool) -> bool {
    for &num in levels {
        if (num - prev_num).abs() > 3
            || num == prev_num
            || (num > prev_num) != prev_comp
        {
            if can_skip {
                can_skip = false;
                continue;
            }
            return false;
        }
        prev_comp = num > prev_num;
        prev_num = num;
    }
    return true;
}

fn main() {
    let input = get_input();
    let input = input.lines();

    let start = Instant::now();

    let mut safe_count = 0;
    for line in input {
        let mut levels = line.split(" ").map(|s| s.parse::<i32>().unwrap());

        let num1 = levels.next().unwrap();
        let num2 = levels.next().unwrap();
        let num3 = levels.next().unwrap();

        let levels: Vec<i32> = levels.collect();

        if !((num2 - num1).abs() > 3 || num1 == num2)
            && !((num3 - num2).abs() > 3 || num2 == num3)
            && (num2 > num1) == (num3 > num2) {
            // if we don't need to skip one of the first three
            if check_deep_levels(&levels, num3, num3 > num2, true) {
                safe_count += 1;
            }
        }
        else {
            if !((num3 - num2).abs() > 3 || num2 == num3) { // skip num1
                if check_deep_levels(&levels, num3, num3 > num2, false) {
                    safe_count += 1;
                    continue;
                }
            }
            if !((num3 - num1).abs() > 3 || num1 == num3) { // skip num2
                if check_deep_levels(&levels, num3, num3 > num1, false) {
                    safe_count += 1;
                    continue;
                }
            }
            if !((num2 - num1).abs() > 3 || num1 == num2) { // skip num3
                if check_deep_levels(&levels, num2, num2 > num1, false) {
                    safe_count += 1;
                    continue;
                }
            }
        }
    }
    println!("{} \n Time: {:.2?}", safe_count, start.elapsed());
}