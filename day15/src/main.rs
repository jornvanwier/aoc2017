#![feature(test)]

extern crate test;

use std::i32;

// const START_A: i64 = 65;
// const START_B: i64 = 8921;
const START_A: i64 = 516;
const START_B: i64 = 190;

const FACTOR_A: i64 = 16807;
const FACTOR_B: i64 = 48271;


fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let a = Generator::new(START_A, FACTOR_A);
    let b = Generator::new(START_B, FACTOR_B);

    // const TAKE: usize = 5;
    const TAKE: usize = 40_000_000;

    a.zip(b)
        .take(TAKE)
        .filter(|&(a_num, b_num)| last_bytes_match(a_num, b_num, 2))
        .count()
}

fn part2() -> usize {
    let a = Generator::create_picky(START_A, FACTOR_A, 4);
    let b = Generator::create_picky(START_B, FACTOR_B, 8);

    const TAKE: usize = 5_000_000;

    a.zip(b)
        .take(TAKE)
        .filter(|&(a_num, b_num)| last_bytes_match(a_num, b_num, 2))
        .count()
}

fn last_bytes_match(a: i32, b: i32, count: usize) -> bool {
    !to_bytes(&a)
        .iter()
        .zip(to_bytes(&b).iter())
        .take(count)
        .any(|(a_byte, b_byte)| a_byte != b_byte)
}

fn to_bytes(n: &i32) -> &[u8; 8] {
    let val = unsafe { std::mem::transmute(n) };
    val
}



struct Generator {
    previous: i64,
    factor: i64,
    modulo_val: Option<i64>,
}

impl Generator {
    fn new(start: i64, factor: i64) -> Self {
        Generator {
            previous: start,
            factor,
            modulo_val: None,
        }
    }

    fn create_picky(start: i64, factor: i64, modulo_val: i64) -> Self {
        Generator {
            previous: start,
            factor,
            modulo_val: Some(modulo_val),
        }
    }
}

impl Iterator for Generator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous = (self.previous * self.factor) % i32::MAX as i64;

        if let Some(modulo_val) = self.modulo_val {
            if self.previous % modulo_val == 0 {
                Some(self.previous as i32)
            } else {
                self.next()
            }
        } else {
            Some(self.previous as i32)
        }
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| part1())
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| part2())
    }
}