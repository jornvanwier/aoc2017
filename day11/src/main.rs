use std::ops::{Add, AddAssign};

fn main() {
    let input = include_str!("input");
    // let input = "ne,ne,s,s";

    let directions = input.split(',');

    let start = Cube::empty();
    let mut hex = start;
    let mut max_dist = 0;

    for direction in directions {
        hex += Cube::direction(direction);

        let dist = start.distance_to(hex);
        if dist > max_dist {
            max_dist = dist;
        }
    }

    println!("max dist {}", max_dist);
    println!("distance {}", start.distance_to(hex));
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Cube { x, y, z }
    }

    fn empty() -> Self {
        Cube { x: 0, y: 0, z: 0 }
    }

    fn direction(direction: &str) -> Self {
        match direction {
            "n" => Cube::new(0, 1, -1),
            "ne" => Cube::new(1, 0, -1),
            "se" => Cube::new(1, -1, 0),
            "s" => Cube::new(0, -1, 1),
            "sw" => Cube::new(-1, 0, 1),
            "nw" => Cube::new(-1, 1, 0),
            _ => Cube::empty(),
        }
    }

    fn distance_to(&self, other: Cube) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}

impl Add for Cube {
    type Output = Cube;

    fn add(self, other: Cube) -> Self {
        Cube::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Cube {
    fn add_assign(&mut self, other: Cube) {
        let new = &self.add(other);
        *self = *new;
    }
}
