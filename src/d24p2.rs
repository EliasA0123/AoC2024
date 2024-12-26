/*
I ended up solving this basically manually, just using the uncommented code here to get a rough idea of where the swaps were.
But before that, I tried for a long time to solve it by computer and I think an
    approach close to this will work once I think it through some more.
*/

use std::collections::HashMap;
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

    const NUM_BITS: usize = 44;

    let mut input = input.split("\n\n");
    let config = input.next().unwrap();
    let mut values = HashMap::new();
    for ln in config.lines() {
        values.insert(&ln[0..3], if &ln[5..6] == "1" {true} else {false});
    }

    let ops = input.next().unwrap();
    let mut operations: HashMap<(&str, &str), (&str, &str)> = HashMap::new();
    let mut reverse_ops: HashMap<(&str, &str, &str), &str> = HashMap::new();
    for ln in ops.lines() {
        let ln: Vec<&str> = ln.split(" ").collect();
        operations.insert((ln[1], ln[4]), (ln[0], ln[2]));
        reverse_ops.insert((ln[0], ln[2], ln[1]), ln[4]);
    }

    let mut correct = [false; NUM_BITS + 1];
    let mut carry = false;
    for i in 0..NUM_BITS {
        let x = *values.get(&format!("x{:02}", i).as_str()).unwrap();
        let y = *values.get(&format!("y{:02}", i).as_str()).unwrap();
        correct[i] = (x != y) != carry;
        carry = (x && y) || ((x != y) && carry);
    }
    correct[NUM_BITS] = carry;

    // let mut wrong_list: Vec<&str> = Vec::new();
    
    // let mut l_c_in = *reverse_ops.get(&("x00", "y00", "AND")).unwrap();
    // let mut x_and_y = "";
    // let mut c_and_xor = "";
    // for i in 1..NUM_BITS { // first bit not carrying could mess things up
    //     let l_x_xor_y = *reverse_ops.get(&(format!("x{:02}", i).as_str(), format!("y{:02}", i).as_str(), "XOR"))
    //         .unwrap_or(reverse_ops.get(&(format!("y{:02}", i).as_str(), format!("x{:02}", i).as_str(), "XOR")).unwrap());

    //     let z_in = *operations.get(&("XOR", format!("z{:02}", i).as_str())).unwrap();
    //     if l_c_in != z_in.0 && l_c_in != z_in.1 {
    //         wrong_list.push(l_c_in);
    //     }
    //     if l_x_xor_y != z_in.0 && l_x_xor_y != z_in.1 {
    //         wrong_list.push(l_x_xor_y);
    //     }

    //     let l_x_and_y = *reverse_ops.get(&(format!("x{:02}", i).as_str(), format!("y{:02}", i).as_str(), "AND"))
    //         .unwrap_or(reverse_ops.get(&(format!("y{:02}", i).as_str(), format!("x{:02}", i).as_str(), "AND")).unwrap());
    //     let l_c_and_xor = *reverse_ops.get(&(l_c_in, l_x_xor_y, "AND"))
    //         .unwrap_or(reverse_ops.get(&(l_x_xor_y, carry, "AND")).unwrap());
        
    //     // let l_c_out = *reverse_ops.get(&(l_x_and_y, l_c_and_xor, "OR"))
    //     //     .unwrap_or(reverse_ops.get(&(l_c_and_xor, l_x_and_y, "AND")).unwrap());

    //     let next_z_in = *operations.get(&("XOR", format!("z{:02}", i + 1).as_str())).unwrap();

    //     let c_out_in = *operations.get(&("OR", next_z_in.0))
    //         .unwrap_or(operations.get(&("OR", next_z_in.1)).unwrap());
        
    //     let mut check_c = true;
    //     if l_x_and_y != c_out_in.0 && l_x_and_y != c_out_in.1 {
    //         wrong_list.push(l_x_and_y);
    //     }
    //     if l_c_and_xor != c_out_in.0 && l_c_and_xor != c_out_in.1 {
    //         wrong_list.push(l_c_and_xor);
    //     }

        // if format!("z{:02}", i).as_str() != *reverse_ops.get(&(carry, x_xor_y, "XOR"))
        //     .unwrap_or(reverse_ops.get(&(x_xor_y, carry, "XOR")).unwrap()) {
        //     wrong_list.push(format!("z{:02}", i).as_str());
        // }

        // let (val1, val2) = *operations.get(&("XOR", format!("z{:02}", i).as_str())).unwrap();

        // if let Some(&(val2a, val2b)) = operations.get(&("XOR", val2)) {
        //     if &val2a[1..3] != format!("z{:02}", i).as_str() {
        //         wrong_list.push(format!("z{:02}", i));
        //         wrong_list.push(format!("z{}", &val2a[1..3]));
        //     }
        // } else {
        //     if &val2a[1..3] != format!("z{:02}", i).as_str() {
        //         wrong_list.push(format!("z{:02}", i));
        //         wrong_list.push(format!("z{}", &val2a[1..3]));
        //     }
        // }
        
        // if let Some(&(val2a, val2b)) = operations.get(&("OR", val2)) {
        //     let (carry1, carry2) = *operations.get(&("OR", val2)).unwrap();
        //     if &val2a[1..3] != format!("z{:02}", i).as_str() {
        //         wrong_list.push(format!("z{:02}", i));
        //         wrong_list.push(format!("z{}", &val2a[1..3]));
        //     }
        // }
    // }

    while operations.len() > 0 {
        let mut to_remove = Vec::new();
        for ((op, res), (val1, val2))  in operations.iter() {
            if values.contains_key(val1) && values.contains_key(val2) {
                let v1 = *values.get(val1).unwrap();
                let v2 = *values.get(val2).unwrap();
                values.insert(*res, match *op {
                    "AND" => v1 && v2,
                    "OR" => v1 || v2,
                    "XOR" => v1 != v2,
                    _ => panic!("something went real wrong")
                });
                to_remove.push((*op, *res));
            }
        }
        for k in &to_remove {
            operations.remove(k);
        }
    }

    // 9, 10, 20, 37, 38, 39, 40, 41, 42, 43
    // 9-10, 20, 37-40, 40-43
    // cwt, z09, jmv, css, z37, pqt, gdd, z05
    // css,cwt,gdd,jmv,pqt,z05,z09,z37
    let wrong_bits: Vec<usize> = (0..NUM_BITS)
        .filter(|&i| *values.get(&format!("z{:02}", i).as_str()).unwrap() != correct[i]).collect();

    println!("{:?}\nTime: {:.2?}", wrong_bits, start.elapsed());
}