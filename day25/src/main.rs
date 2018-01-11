use std::collections::HashMap;

#[cfg(windows)]
pub const NL: &'static str = "\r\n";
#[cfg(not(windows))]
pub const NL: &'static str = "\n";

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");

    let mut lines = input.split(NL);

    // skip start, always A
    lines.next();

    // get steps count
    let step_line = lines.next().unwrap();
    let last_space = step_line.chars().rev().position(|c| c == ' ').unwrap();
    let second_last_space = step_line
        .chars()
        .rev()
        .skip(last_space + 1)
        .position(|c| c == ' ')
        .unwrap();
    let count: usize = step_line
        .chars()
        .skip(step_line.len() - last_space - 1 - second_last_space)
        .take(second_last_space)
        .collect::<String>()
        .parse()
        .unwrap();
    println!("count {}", count);

    let join: String = lines.map(|l| [l, NL].concat()).collect();

    let states: HashMap<char, State> = join.split(&[NL, NL].concat())
        .map(|s| {
            let state = State::from_str(s);
            (state.name, state)
        })
        .collect();

    let mut machine = Machine::new(states);

    machine.execute_steps(count);

    println!("Part 1: {}", machine.diag_checksum());
}

#[derive(Debug)]
struct Machine {
    store: HashMap<isize, bool>,
    states: HashMap<char, State>,
    pos: isize
}

impl Machine {
    fn new(states: HashMap<char, State>) -> Self {
        Self {
            store: HashMap::new(),
            states,
            pos: 0
        }
    }

    fn execute_steps(&mut self, step_count: usize) {
        let mut state_name = 'A';
        for _ in 0..step_count {
            let state = self.states[&state_name].clone();
            state_name = self.execute_state(&state);
            // println!("{}", self.dump_store());
        }
    }

    fn execute_state(&mut self, state: &State) -> char {
        use Instruction::*;

        // println!("{:?}", state);

        let current = &mut *self.store.entry(self.pos).or_insert(false);

        let instructions = if *current {
            &state.instructions_true
        }
        else {
            &state.instructions_false
        };

        for instruction in instructions {
            match instruction {
                &Write(state) => *current = state,
                &Move(val) => self.pos += val,
                &ChangeState(next) => return next
            }
        }

        unreachable!()
    }

    fn diag_checksum(&self) -> usize {
        self.store.values().filter(|v| **v).count()
    }

    fn dump_store(&self) -> String {
        let min = self.store.keys().min().unwrap();
        let max = self.store.keys().max().unwrap();

        let size = ((max - min) + 1).abs() as usize;

        if size == 0 {
            return "".into();
        }

        let mut store = vec![0; size];
        
        for (pos, val) in &self.store {
            store[(pos - min) as usize] = if *val { 1 } else { 0 };
        }

        store.iter().enumerate().map(|(i,v)| {
            if i as isize == self.pos - min {
                format!("[{}]", v)
            }
            else {
                format!(" {} ", v)
            }
        }).collect()
    }
}

#[derive(Debug, Clone)]
struct State {
    name: char,
    instructions_true: Vec<Instruction>,
    instructions_false: Vec<Instruction>,
}

impl State {
    fn from_str(input: &str) -> Self {
        let mut lines = input.split(NL).filter(|&s| s != "");

        let first_line = lines.next().unwrap();

        let last_space = first_line.chars().rev().position(|c| c == ' ').unwrap();
        let name = first_line
            .chars()
            .skip(first_line.len() - last_space)
            .next()
            .unwrap();

        // skip the if 0
        lines.next();

        let mut instructions_false = Vec::new();
        for _ in 0..3 {
            instructions_false.push(Instruction::from_str(lines.next().unwrap()))
        }

        // skip the if 1
        lines.next();

        let mut instructions_true = Vec::new();
        for _ in 0..3 {
            instructions_true.push(Instruction::from_str(lines.next().unwrap()))
        }

        Self {
            name,
            instructions_true,
            instructions_false,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Write(bool),
    Move(isize),
    ChangeState(char),
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        use Instruction::*;

        let last_space = input.chars().rev().position(|c| c == ' ').unwrap();
        let val: String = input
            .chars()
            .skip(input.len() - last_space)
            .take(last_space - 1)
            .collect();

        match input.chars().nth(6).unwrap() {
            'W' => Write(if val == "1" { true } else { false }),
            'M' => Move((if val == "right" { 1 } else { -1 })),
            'C' => ChangeState(val.chars().next().unwrap()),
            _ => unreachable!(),
        }
    }
}
