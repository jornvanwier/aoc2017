use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");
    // let input = include_str!("first_example");

    let instructions: Vec<Instruction> = input
        .split("\r\n")
        .map(|l| Instruction::from_str(l))
        .collect();

    let mut zero = DuetRegisters::new(&instructions, 0);
    let mut one = DuetRegisters::new(&instructions, 1);

    while !(zero.is_waiting && one.is_waiting) {
        if let Some(send) = zero.execute_next_instruction() {
            one.queue.insert(0, send);
        }

        if let Some(send) = one.execute_next_instruction() {
            zero.queue.insert(0, send);
        }
    }

    println!("Part 2: {}", one.send_count);
}

#[derive(Debug)]
struct DuetRegisters<'a> {
    registers: HashMap<char, i64>,
    instructions: &'a Vec<Instruction>,
    queue: Vec<i64>,
    is_waiting: bool,
    index: i64,
    send_count: i64,
}

impl<'a> DuetRegisters<'a> {
    fn new(instructions: &'a Vec<Instruction>, p: i64) -> Self {
        let mut r = Self {
            registers: HashMap::new(),
            instructions,
            index: 0,
            queue: Vec::new(),
            is_waiting: false,
            send_count: 0,
        };

        r.registers.insert('p', p);

        r
    }

    fn execute_next_instruction(&mut self) -> Option<i64> {
        use Instruction::*;

        let instruction = self.instructions.get(self.index as usize).unwrap();

        match *instruction {
            Snd(register) => {
                self.send_count += 1;
                self.index += 1; // early return
                return Some(*self.get_or_create_register(register));
            }
            Set(register, ref value) => {
                let value = value.get_value(&mut self.registers);
                let reg = self.get_or_create_register(register);
                *reg = value;
            }
            Add(register, ref value) => {
                let value = value.get_value(&mut self.registers);
                let reg = self.get_or_create_register(register);
                *reg += value;
            }
            Mul(register, ref value) => {
                let value = value.get_value(&mut self.registers);
                let reg = self.get_or_create_register(register);
                *reg *= value;
            }
            Mod(register, ref value) => {
                let value = value.get_value(&mut self.registers);
                let reg = self.get_or_create_register(register);
                *reg %= value;
            }
            Receive(register) => {
                if self.queue.len() > 0 {
                    self.is_waiting = false;
                    let value = self.queue.pop().unwrap();
                    let reg = self.get_or_create_register(register);
                    *reg = value;
                } else {
                    self.is_waiting = true;
                    return None;
                }
            }
            Jump(ref register, ref value) => {
                if register.get_value(&mut self.registers) > 0 {
                    self.index += value.get_value(&mut self.registers);
                    return None;
                }
            }
        }

        self.index += 1;

        None
    }

    fn get_or_create_register(&mut self, register: char) -> &mut i64 {
        self.registers.entry(register).or_insert(0)
    }
}

#[derive(Debug)]
enum Instruction {
    Snd(char),
    Set(char, RegisterValue),
    Add(char, RegisterValue),
    Mul(char, RegisterValue),
    Mod(char, RegisterValue),
    Receive(char),
    Jump(RegisterValue, RegisterValue),
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        use Instruction::*;

        let mut parts = input.split(" ");

        match parts.next().unwrap() {
            "snd" => {
                let register = extract_register(&mut parts);
                Snd(register)
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
                Receive(register)
            }
            "jgz" => {
                let register = RegisterValue::from_parts(&mut parts);
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
    Value(i64),
}

impl RegisterValue {
    fn from_parts<'a, T>(parts: &mut T) -> Self
    where
        T: Iterator<Item = &'a str>,
    {
        let val = parts.next().expect("Empty iterator");
        match val.parse::<i64>() {
            Ok(n) => RegisterValue::Value(n),
            Err(_) => RegisterValue::Register(val.chars().next().expect("No chars")),
        }
    }

    fn get_value(&self, registers: &mut HashMap<char, i64>) -> i64 {
        use RegisterValue::*;
        match *self {
            Register(r) => *registers.entry(r).or_insert(0),
            Value(n) => n,
        }
    }
}


fn extract_register<'a, T>(parts: &mut T) -> char
where
    T: Iterator<Item = &'a str>,
{
    parts
        .next()
        .expect("Empty iterator")
        .chars()
        .next()
        .expect("No chars")
}
