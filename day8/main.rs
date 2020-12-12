use std::fs;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    NoOp,
    Accumulate (i32),
    Jump (bool, usize)
}

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_8\\handheld_halting\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let instructions: Vec<Instruction> = contents.split("\r\n")
                                                 .map(|line| parse_instruction(line))
                                                 .collect();
    
    let count_jmp = instructions.iter()
                                .filter(|inst| {
                                    match inst {
                                        Instruction::NoOp => false,
                                        Instruction::Accumulate (_) => false,
                                        Instruction::Jump (_, _) => true
                                    }
                                })
                                .count();

    for occ in 1..count_jmp {
        let result = test_instruction_set(&swap_jmp_to_noop(&instructions, occ));
        if result.1 {
            println!("swap pos: {}, accumulator: {}, reached end: {}", occ, result.0, result.1);
        }
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let inst_regex = Regex::new(r"^[a-z]+").unwrap();
    let inst = inst_regex.find(line).unwrap();

    let val_regex = Regex::new(r"[+-]\d+$").unwrap();
    let val = val_regex.find(line).unwrap();

    match inst.as_str() {
        "nop" => return Instruction::NoOp,
        "acc" => return Instruction::Accumulate(val.as_str().replace("+", "").parse().unwrap()),
        "jmp" => return Instruction::Jump(val.as_str().contains("-"), val.as_str().replace("+", "").replace("-", "").parse().unwrap()),
        _ => return Instruction::NoOp
    }
}

fn swap_jmp_to_noop(instructions: &Vec<Instruction>, occurance: usize) -> Vec<Instruction> {
    let mut new_inst = Vec::with_capacity(instructions.len());
    let mut jmp_count: usize = 0;
    for i in 0..instructions.len() {
        match instructions[i] {
            Instruction::NoOp => new_inst.push(Instruction::NoOp),
            Instruction::Accumulate (acc_val)=> new_inst.push(Instruction::Accumulate (acc_val)),
            Instruction::Jump (neg, jmp_val) => {
                jmp_count += 1;
                if jmp_count == occurance {
                    new_inst.push(Instruction::NoOp);
                }
                else {
                    new_inst.push(Instruction::Jump(neg, jmp_val));
                }
            }
        }
    }
    return new_inst;
}

fn test_instruction_set(instructions: &Vec<Instruction>) -> (i32, bool) {
    let mut executed_instructions = HashSet::with_capacity(instructions.len());
    let mut accumulator: i32 = 0;
    let mut index: usize = 0;
    let mut reached_end: bool = false;
    while !executed_instructions.contains(&index) {
        if index >= instructions.len() {
            reached_end = true;
            break;
        }
        executed_instructions.insert(index);

        let instruction = &instructions[index];
        match instruction {
            Instruction::NoOp => index += 1,
            Instruction::Accumulate (acc_val) => {
                accumulator += acc_val;
                index += 1
            },
            Instruction::Jump (negative, jmp_val) => index = if *negative {index - jmp_val} else {index + jmp_val}
        }
    }
    return (accumulator, reached_end);
}