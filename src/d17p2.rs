// FYI: this is all hardcoded for my input. I might go back and generalize it later

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

fn find_template(program: &Vec<usize>, a: Vec<Option<usize>>, i: usize) {
    let goal = program[i];
    'byt: for byt in 0..8 {
        let div_byt = byt ^ goal ^ 2;
        let div_shift = byt ^ 6;
        for bit in 0..3 {
            // front bits
            if let Some(b) = a[3 * i + bit] {
                if b != (byt >> bit) % 2 {
                    continue 'byt;
                }
            }
            // modifying bits
            if let Some(b) = match a.get(3 * i + bit + div_shift) {
                Some(&bb) => bb,
                None => Some(0)
            } {
                if b != (div_byt >> bit) % 2 {
                    continue 'byt;
                }
            }
        }

        let mut test_a = a.clone();            
        test_a[3*i] = Some(byt % 2);
        test_a[3*i + 1] = Some((byt >> 1) % 2);
        test_a[3*i + 2] = Some((byt >> 2) % 2);

        if 3*i + div_shift < test_a.len() {
            test_a[3*i + div_shift] = Some(div_byt % 2);
        }
        if 3*i + 1 + div_shift < test_a.len() {
            test_a[3*i + 1 + div_shift] = Some((div_byt >> 1) % 2);
        }
        if 3*i + 2 + div_shift < test_a.len() {
            test_a[3*i + 2 + div_shift] = Some((div_byt >> 2) % 2);
        }

        if !test_a.contains(&None) {
            println!("{}", test_a.iter().enumerate().map(|(i, b)| b.unwrap() << i).sum::<usize>());
            std::process::exit(0);
        }
        find_template(program, test_a, i + 1);
    }
}

fn main() {
    let input = get_input();
    let start = Instant::now();

    let program: Vec<usize> = input.split("\n\n").nth(1).unwrap()
        .split(": ").collect::<Vec<&str>>()[1].split(",")
        .map(|i| i.parse::<usize>().unwrap()).collect();

    // let mut a = 0;
    // 'a: loop {
    //     let mut registers: Vec<usize> = vec![a, 0, 0];
    //     let mut output = Vec::new();
    //     let mut output_i = 0;
    //     let mut instr = 0;
    //     while instr < program.len() - 1 {
    //         let opcode = *program.get(instr).unwrap();
    //         let operand = *program.get(instr + 1).unwrap();

    //         match opcode {
    //             0 => {
    //                 let operand = if operand < 4 {operand} else {registers[operand - 4]};
    //                 registers[0] = registers[0] / (1 << operand);
    //             },
    //             1 => {
    //                 registers[1] = registers[1] ^ operand;
    //             },
    //             2 => {
    //                 let operand = if operand < 4 {operand} else {registers[operand - 4]};
    //                 registers[1] = operand % 8;
    //             },
    //             3 => {
    //                 if registers[0] != 0 {
    //                     instr = operand;
    //                 } else {
    //                     instr += 2;
    //                 }
    //             },
    //             4 => {
    //                 registers[1] = registers[1] ^ registers[2];
    //             },
    //             5 => {
    //                 let operand = if operand < 4 {operand} else {registers[operand - 4]};
    //                 if operand % 8 != program[output_i] {
    //                     a += 1;
    //                     continue 'a;
    //                 }
    //                 output.push(operand % 8);
    //                 output_i += 1;
    //             },
    //             6 => {
    //                 let operand = if operand < 4 {operand} else {registers[operand - 4]};
    //                 registers[1] = registers[0] / (1 << operand);
    //             },
    //             7 => {
    //                 let operand = if operand < 4 {operand} else {registers[operand - 4]};
    //                 registers[2] = registers[0] / (1 << operand);
    //             },
    //             _ => ()
    //         }
    //         if opcode != 3 {
    //             instr += 2;
    //         }
    //     }

    //     if output == program {
    //         break;
    //     }

    //     a += 1;
    // }

    // let mut a_counter = 1 << 47;
    // 'a: loop {
    //     if a_counter % (1 << 40) == 0 {
    //         println!("{}", a_counter);
    //     }
    //     let mut i = 0;
    //     let mut a = a_counter;
    //     // let mut output = Vec::new();
    //     loop {
    //         if ((a % 8) ^ (a >> ((a % 8) ^ 6)) ^ 2) % 8 == program[i] {
    //             i += 1;
    //             a = a >> 3;
    //             if a == 0 || i > program.len() {
    //                 break;
    //             }
    //         } else {
    //             if a_counter == 117440 {
    //                 println!("{}, {}", ((a % 8) ^ (a >> ((a % 8) ^ 6)) ^ 2) % 8, program[i]);
    //             }
    //             a_counter += 1;
    //             continue 'a;
    //         }
    //     }
    //     if a_counter == 117440 {
    //         println!("{}", i);
    //     }
    //     if i == program.len() {
    //         break;
    //     }
    //     a_counter += 1;
    // }

    let mut a = vec![None; 48];
    find_template(&program, a.clone(), 0);

    // println!("{}", a);
    println!("Time: {:.2?}", start.elapsed());
}

// B = A % 8    last three bits of a, changes each loop
// B = B ^ 6
// B = B ^ (A >> B)   only last 3 bits matter
// B = B ^ 4    reversible
// OUTPUT B % 8 
// A = A / 8
// JMP 0

// 16 * 3 = 48 bits long, in between 2^47 and 2^48

// The three bits specify three bits based on the three bits and the three bits bitwise with the three bits to output three bits