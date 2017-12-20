#![feature(vec_remove_item)]

use std::ops::{Add, AddAssign};

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");

    const SIMULATION: usize = 1000;
    let zero = Vec3::empty();

    let particles: Vec<Particle> = input.split("\r\n").map(|l| Particle::from_str(l)).collect();
    let mut part1ticles = particles.clone();

    let mut closest = None;

    for _ in 0..SIMULATION {
        part1ticles.iter_mut().for_each(|p| {
            p.tick();
        });

        closest = part1ticles
            .clone()
            .iter()
            .min_by_key(|p| p.position.distance_to(zero))
            .map(|c| *c);
    }

    let closest_index = part1ticles
        .iter()
        .position(|p| *p == closest.unwrap())
        .unwrap();

    println!("Part 1: {}", closest_index);

    let mut part2ticles = particles.clone();

    for _ in 0..SIMULATION {
        part2ticles.iter_mut().for_each(|p| {
            p.tick();
        });

        let mut remove = Vec::new();
        for particle in &part2ticles {
            let mut group: Vec<Particle> = part2ticles
                .iter()
                .filter(|p| p.position == particle.position)
                .map(|p| p.clone())
                .collect();

            if group.len() != 1 {
                remove.append(&mut group)
            }
        }

        part2ticles.retain(|p| !remove.contains(&p));
    }

    println!("Part 2: {}", part2ticles.len());
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Particle {
    fn new(position: Vec3, velocity: Vec3, acceleration: Vec3) -> Self {
        Self {
            position,
            velocity,
            acceleration,
        }
    }

    fn from_str(input: &str) -> Self {
        let vectors: Vec<Vec3> = input
            .split('=')
            .skip(1)
            .map(|v| Vec3::from_str(v))
            .collect();

        Self::new(vectors[0], vectors[1], vectors[2])
    }

    fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn from_str(input: &str) -> Self {
        let open = input.chars().position(|c| c == '<').unwrap();
        let close = input.chars().skip(open).position(|c| c == '>').unwrap();

        let slice: String = input
            .chars()
            .skip(open + 1)
            .take(close - open - 1)
            .collect();
        let values: Vec<i32> = slice
            .split(',')
            .map(|n| n.trim().parse().expect("Not a number"))
            .collect();

        Self::new(values[0], values[1], values[2])
    }

    fn empty() -> Self {
        Self::new(0, 0, 0)
    }

    fn distance_to(&self, to: Self) -> i32 {
        (self.x - to.x).abs() + (self.y - to.y).abs() + (self.z - to.z).abs()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
