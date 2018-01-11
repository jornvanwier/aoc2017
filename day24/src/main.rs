#![feature(vec_remove_item)]

use std::fmt::{Display, Formatter, self};

#[cfg(windows)]
pub const NL: &'static str = "\r\n";
#[cfg(not(windows))]
pub const NL: &'static str = "\n";

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");

    let components: Vec<Component> = input.split(NL).map(|l| Component::from_str(l)).collect();

    let part1 = solve1(components.clone());

    println!("Part 1: {}", part1);

    let part2 = solve2(components);

    println!("Part 2: {}", part2);
}

fn solve1(components: Vec<Component>) -> u16 {
    let all_options = get_bridges(Vec::new(), &Component::new(0,0), components);

    let scores = all_options.iter().map(|b| {
        b.iter().fold(0, |acc,x| acc + x.score)
    });

    scores.max().unwrap()
}

fn solve2(components: Vec<Component>) -> u16 {
    let all_options = get_bridges(Vec::new(), &Component::new(0,0), components);
    
    all_options.iter().max_by(|a ,b| a.len().cmp(&b.len())).unwrap().iter().fold(0, |acc,x| acc + x.score)
}

fn get_bridges(existing: Vec<Component>, connect_to: &Component, components: Vec<Component>) -> Vec<Vec<Component>> {
    let mut all_options = Vec::new();
    for component in &components {
        if let Some(port) = connect_to.fit(&component) {
            let mut component_clone = component.clone();
            component_clone.open.remove_item(&port);
            let mut components_clone = components.clone();
            components_clone.remove_item(component);
            let mut existing_clone = existing.clone();
            existing_clone.push(component_clone.clone());

            all_options.push(existing_clone.clone());

            let mut sub_bridges = get_bridges(existing_clone, &component_clone, components_clone);
            all_options.append(&mut sub_bridges);
        }
    }

    all_options
}

#[derive(Debug, Clone, PartialEq)]
struct Component {
    original: Vec<u8>,
    open: Vec<u8>,
    score: u16,
}

impl Component {
    fn from_str(input: &str) -> Self {
        let mut split = input.split("/");
        let a = split.next().unwrap().parse().unwrap();
        let b = split.next().unwrap().parse().unwrap();

        Self::new(a, b)
    }

    fn new(a: u8, b: u8) -> Self {
        Component {
            original: vec![a, b],
            open: vec![a, b],
            score: a as u16 + b as u16,
        }
    }

    fn fit(&self, other: &Component) -> Option<u8> {
        for oa in &self.open {
            if other.open.iter().find(|&ob| ob == oa).is_some() {
                return Some(*oa);
            }
        }
        None
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "-{}/{}-", self.original[0], self.original[1])
    }
}