use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");

    let instructions: Vec<Instruction> = input
        .split("\r\n")
        .map(|l| Instruction::from_str(l))
        .collect();

    let mut registers = DuetRegisters::new();
    println!("Part 1: {}", registers.part1(instructions));
}

#[derive(Debug)]
struct DuetRegisters {
    last_sound: Option<i64>,
    registers: HashMap<char, i64>,
            queue: Vec<i64>,
            is_waiting: bool
}

impl DuetRegisters {
    fn new() -> Self {
        Self {
            last_sound: None,
            registers: HashMap::new(),
            queue: Vec::new(),
            is_waiting: false
        }
    }

    fn part1(&mut self, instructions: Vec<Instruction>) -> i64 {
        use Instruction::*;

        let mut i: i64 = 0;
        while (i as usize) < instructions.len() {
            let instruction = instructions.get(i as usize).unwrap();

            match *instruction {
                Sound(register) => self.last_sound = Some(*self.get_or_create_register(register)),
                Set(register, ref value) => {
                    let value = value.get_value(&mut self.registers);
                    let mut reg = self.get_or_create_register(register);
                    *reg = value;
                }
                Add(register, ref value) => {
                    let value = value.get_value(&mut self.registers);
                    let mut reg = self.get_or_create_register(register);
                    *reg += value;
                }
                Mul(register, ref value) => {
                    let value = value.get_value(&mut self.registers);
                    let mut reg = self.get_or_create_register(register);
                    *reg *= value;
                }
                Mod(register, ref value) => {
                    let value = value.get_value(&mut self.registers);
                    let mut reg = self.get_or_create_register(register);
                    *reg %= value;
                }
                Recover(register) => {
                    if *self.get_or_create_register(register) > 0 {
                        if let Some(last_sound) = self.last_sound {
                            return last_sound;
                        }
                    }
                }
                Jump(register, ref value) => {
                    if *self.get_or_create_register(register) > 0 {
                        i += value.get_value(&mut self.registers);
                        continue; // To prevent to normal incrementation
                    }
                }
            }

            i += 1;
        }

        0
    }

    fn get_or_create_register(&mut self, register: char) -> &mut i64 {
        self.registers.entry(register).or_insert(0)
    }
}

#[derive(Debug)]
enum Instruction {
    Sound(char),
    Set(char, RegisterValue),
    Add(char, RegisterValue),
    Mul(char, RegisterValue),
    Mod(char, RegisterValue),
    Recover(char),
    Jump(char, RegisterValue),
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        use Instruction::*;

        let mut parts = input.split(" ");

        match parts.next().unwrap() {
            "snd" => {
                let register = extract_register(&mut parts);
                Sound(register)
            }
            "set" => {
                let register = extract_register(&mut parts);
                let value = RegisterValue::from_parts(&mut parts);
                Set(register, value)
            }
            "add" => {
                let register = extract_register(&mut parts);
                let value = RegisterValue::from_parts(&mut parts);
                Add(register, value)
            }
            "mul" => {
                let register = extract_register(&mut parts);
                let value = RegisterValue::from_parts(&mut parts);
                Mul(register, value)
            }
            "mod" => {
                let register = extract_register(&mut parts);
                let value = RegisterValue::from_parts(&mut parts);
                Mod(register, value)
            }
            "rcv" => {
                let register = extract_register(&mut parts);
                Recover(register)
            }
            "jgz" => {
                let register = extract_register(&mut parts);
                let value = RegisterValue::from_parts(&mut parts);
                Jump(register, value)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum RegisterValue {
    Register(char),
    Value(i64)
}

impl RegisterValue {
    fn from_parts<'a, T>(parts: &mut T) -> Self
where
    T: Iterator<Item = &'a str> {
        let val = parts.next().expect("Empty iterator");
        match val.parse::<i64>() {
            Ok(n) => RegisterValue::Value(n),
            Err(_) => RegisterValue::Register(val.chars().next().expect("No chars"))
        }
    }

    fn get_value(&self, registers: &mut HashMap<char, i64>) -> i64 {
        use RegisterValue::*;
        match *self {
            Register(r) => {
                *registers.entry(r).or_insert(0)
            }
            Value(n) => n
        }
    }
}


fn extract_register<'a, T>(parts: &mut T) -> char
where
    T: Iterator<Item = &'a str>,
{
    parts.next().expect("Empty iterator").chars().next().expect("No chars")
}
