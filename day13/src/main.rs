use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

fn main() {
    let input = include_str!("example");
    let input = include_str!("input");

    let firewall: HashMap<i32, Layer> = input
        .split("\r\n")
        .map(|l| {
            let mut parts = l.split(": ");
            let depth = parts.nth(0).unwrap().parse().unwrap();

            (
                depth,
                Layer::new(depth, parts.nth(0).unwrap().parse().unwrap()),
            )
        })
        .collect();

    let length = *firewall.keys().max().unwrap() + 1;

    println!("Part 1: {}", get_severity(&firewall, length).1);

    let mut score_two = -1;
    for i in 0..3933130 {
        if !firewall.iter().any(|(d, l)| is_hit(*d + i, l.range)) {
            score_two = i;
            break;
        }
    }

    println!("Part 2: {}", score_two);
}
fn is_hit(n: i32, range: i32) -> bool {
    n % (2 * (range - 1)) == 0
}

fn firewall_tick(firewall: &mut HashMap<i32, Layer>) {
    firewall.iter_mut().for_each(|(_, l)| l.tick());
}

fn print_firewall(firewall: &HashMap<i32, Layer>) {
    println!("Firewall:");
    let mut wall = firewall.values().collect::<Vec<&Layer>>();
    wall.sort_by_key(|l| l.depth);
    for layer in wall.iter() {
        println!("{}", layer);
    }
    println!("");
}

fn get_severity(firewall: &HashMap<i32, Layer>, length: i32) -> (bool, i32) {
    let mut firewall = firewall.clone();
    let mut score = 0;
    let mut ever_caught = false;

    for i in 0..length {
        if firewall.contains_key(&i) {
            let layer = &firewall[&i];
            if layer.is_at_top() {
                ever_caught = true;
                score += layer.depth * layer.range;
            }
        }

        firewall_tick(&mut firewall);
    }
    (ever_caught, score)
}

#[derive(Debug, Clone)]
struct Layer {
    depth: i32,
    range: i32,
    position: i32,
    direction: i32,
}

impl Layer {
    fn new(depth: i32, range: i32) -> Self {
        Self {
            depth,
            range,
            position: 0,
            direction: 1,
        }
    }

    fn tick(&mut self) {
        self.position += self.direction;
        if self.position == -1 || self.position == self.range {
            self.direction *= -1;
            self.position += self.direction * 2;
        }
    }

    fn is_at_top(&self) -> bool {
        self.position == 0
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut range = "".into();
        for i in 0..self.range {
            if i == self.position {
                range = [range, "[x]".into()].join(" ")
            } else {
                range = [range, "[ ]".into()].join(" ")
            }
        }
        write!(f, "{}: {}", self.depth, range)
    }
}
