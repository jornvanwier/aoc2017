extern crate regex;

use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");

    let instructions: Vec<Instruction> = input
        .split("\r\n")
        .map(|l| Instruction::from_str(l))
        .collect();

    let mut registers = HashMap::new();
    instructions
        .iter()
        .map(|i| i.register)
        .for_each(|register| {
            registers.insert(register, 0);
        });

    let mut max = 0;

    for instruction in instructions {
        if check_condition(&instruction, &registers) {
            apply_operation(&instruction, &mut registers)
        }

        let current_max = get_max_value(&registers);
        if current_max > max {
            max = current_max;
        }
    }

    println!("Max ever:\t{}", max);
    println!("Max end:\t{}", get_max_value(&registers));
}

fn get_max_value(registers: &HashMap<&str, i32>) -> i32 {
    *registers.values().max().unwrap()
}

fn check_condition(instruction: &Instruction, registers: &HashMap<&str, i32>) -> bool {
    let register_val = registers[instruction.condition_register];
    let compare_val = instruction.condition_value;

    match instruction.condition_operation {
        "==" => register_val == compare_val,
        ">" => register_val > compare_val,
        "<" => register_val < compare_val,
        "!=" => register_val != compare_val,
        ">=" => register_val >= compare_val,
        "<=" => register_val <= compare_val,
        _ => unreachable!(),
    }
}

fn apply_operation(instruction: &Instruction, registers: &mut HashMap<&str, i32>) {
    // println!(
    //     "Applying operation {} to {} with {}",
    //     instruction.operation, registers[instruction.register], instruction.value
    // );
    match instruction.operation {
        "inc" => *registers.get_mut(instruction.register).unwrap() += instruction.value,
        "dec" => *registers.get_mut(instruction.register).unwrap() -= instruction.value,
        _ => unreachable!(),
    }
}

struct Instruction<'a> {
    register: &'a str,
    operation: &'a str,
    value: i32,
    condition_register: &'a str,
    condition_operation: &'a str,
    condition_value: i32,
}

impl<'a> Instruction<'a> {
    fn from_str(input: &'a str) -> Self {
        let captures = Regex::new("(\\w+) (\\w+) (-?\\d+) if (\\w+) ([<>=!]+) (-?\\d+)")
            .unwrap()
            .captures(input)
            .unwrap();

        // println!("{:?}", captures);

        Instruction {
            register: captures.get(1).unwrap().as_str(),
            operation: captures.get(2).unwrap().as_str(),
            value: captures.get(3).unwrap().as_str().parse().unwrap(),
            condition_register: captures.get(4).unwrap().as_str(),
            condition_operation: captures.get(5).unwrap().as_str(),
            condition_value: captures.get(6).unwrap().as_str().parse().unwrap(),
        }
    }
}
