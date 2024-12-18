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

    let mut input = input.split("\n\n");
    let mut registers: Vec<usize> = input.next().unwrap().lines()
        .map(|ln| ln.split(": ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap()).collect();

    let program: Vec<usize> = input.next().unwrap()
        .split(": ").collect::<Vec<&str>>()[1].split(",")
        .map(|i| i.parse::<usize>().unwrap()).collect();

    let mut output = String::new();
    let mut instr = 0;
    while instr < program.len() - 1 { // is this right?
        let opcode = *program.get(instr).unwrap();
        let operand = *program.get(instr + 1).unwrap();

        match opcode {
            0 => {
                let operand = if operand < 4 {operand} else {registers[operand - 4]};
                registers[0] = registers[0] / (1 << operand);
            },
            1 => {
                registers[1] = registers[1] ^ operand;
            },
            2 => {
                let operand = if operand < 4 {operand} else {registers[operand - 4]};
                registers[1] = operand % 8;
            },
            3 => {
                if registers[0] != 0 {
                    instr = operand;
                } else {
                    instr += 2;
                }
            },
            4 => {
                registers[1] = registers[1] ^ registers[2];
            },
            5 => {
                let operand = if operand < 4 {operand} else {registers[operand - 4]};
                output += format!("{},", operand % 8).as_str();
            },
            6 => {
                let operand = if operand < 4 {operand} else {registers[operand - 4]};
                registers[1] = registers[0] / (1 << operand);
            },
            7 => {
                let operand = if operand < 4 {operand} else {registers[operand - 4]};
                registers[2] = registers[0] / (1 << operand);
            },
            _ => ()
        }
        if opcode != 3 {
            instr += 2;
        }
    }

    println!("{}", output);
    println!("Time: {:.2?}", start.elapsed());
}