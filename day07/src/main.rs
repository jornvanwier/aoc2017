extern crate regex;

use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");
    // let input = include_str!("ruurd");
    let nodes_flat: Vec<Node> = input.split("\r\n").map(|l| Node::from_str(l)).collect();
    let mut node_map = HashMap::new();
    nodes_flat.iter().for_each(|n| {
        node_map.insert(&*n.name, n);
    });

    println!("n: {}", node_map.len());

    let root = part1(&nodes_flat).unwrap();
    println!("{}", root);
    part2(node_map[&*root], node_map);
}

fn part2(root: &Node, map: HashMap<&str, &Node>) {
    println!("{}", root.len(&map));
    root.find_inbalance(&map)
}

fn part1(nodes: &Vec<Node>) -> Option<String> {
    'outer: for node in nodes {
        for other_node in nodes {
            if other_node.child_names.contains(&node.name) {
                continue 'outer;
            }
        }
        return Some(node.name.to_string());
    }
    None
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    weight: i32,
    child_names: Vec<String>,
}

impl Node {
    fn from_str<'input>(string: &'input str) -> Self {
        let captures = Regex::new("(\\w+) \\((\\d+)\\)(?: -> (.+))?")
            .unwrap()
            .captures(string)
            .unwrap();
        Self {
            name: captures.get(1).unwrap().as_str().to_string(),
            weight: captures.get(2).unwrap().as_str().parse().unwrap(),
            child_names: captures
                .get(3)
                .map(|m| m.as_str())
                .or(Some(""))
                .unwrap()
                .split(", ")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect(),
        }
    }

    fn weight(&self, map: &HashMap<&str, &Node>) -> i32 {
        self.weight + self.get_weight(map)
    }

    fn get_weight(&self, map: &HashMap<&str, &Node>) -> i32 {
        self.child_names
            .iter()
            .fold(0, |b, c| b + map.get::<str>(c).unwrap().weight(map))
    }

    fn len(&self, map: &HashMap<&str, &Node>) -> usize {
        1
            + self.child_names
                .iter()
                .map(|c| *map.get::<str>(c).unwrap())
                .fold(0, |b, a| b + a.len(map))
    }

    fn find_inbalance(&self, map: &HashMap<&str, &Node>) {
        let children: Vec<&Node> = self.child_names
            .iter()
            .map(|c| *map.get::<str>(c).unwrap())
            .collect();

        println!("{}\t> {:?}", self.name, self.child_names);

        children
            .iter()
            .zip(children.iter().skip(1))
            .for_each(|(a, b)| {
                let weight_a = a.weight(map);
                let weight_b = b.weight(map);
                if weight_a != weight_b {
                    println!("{}/{} != {}/{}", a.weight, weight_a, b.weight, weight_b);
                    println!(
                        "{:?}",
                        children.iter().map(|c| c.weight(map)).collect::<Vec<i32>>()
                    );
                    a.find_inbalance(map);
                    b.find_inbalance(map);
                }
            });
    }
}
