mod matrix;

use std::collections::HashMap;
use matrix::*;

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");
    // let input = include_str!("ruurd");
    // let input = include_str!("other");

    let rules: HashMap<Matrix<bool>, Matrix<bool>> = input
        .split("\r\n")
        .map(|l| {
            let mut split = l.split(" => ");
            (
                from_rule_str(split.next().unwrap()),
                from_rule_str(split.next().unwrap()),
            )
        })
        .collect();

    // rules.keys().for_each(|m| {
    //     println!("{}", m);
    // });

    let start = [
        [false, true, false],
        [false, false, true],
        [true, true, true],
    ];

    let mut matrix = Matrix::from_2d_array(start.iter().map(|v| v.to_vec()).collect());

    // const ITERATIONS: usize = 5;
    const ITERATIONS: usize = 18;
    // const ITERATIONS: usize = 2;

    for _ in 0..ITERATIONS {
        println!("{}", matrix);

        let mut divided = if matrix.size() % 2 == 0 {
            matrix.divide(2)
        } else {
            assert!(matrix.size() % 3 == 0);
            matrix.divide(3)
        };

        divided
            .data
            .iter_mut()
            .for_each(|mut m| replace_by_rule(&mut m, &rules));

        matrix = divided.join();

        println!("{}", count_on(&matrix));
    }

    println!("Part 1: {}", count_on(&matrix));
}

fn replace_by_rule(matrix: &mut Matrix<bool>, rules: &HashMap<Matrix<bool>, Matrix<bool>>) {
    if let Some(replace) = find_matching_rule(matrix, rules) {
        *matrix = replace;
    } else {
        panic!("No matching rule!");
    }
}

fn find_matching_rule(
    matrix: &Matrix<bool>,
    rules: &HashMap<Matrix<bool>, Matrix<bool>>,
) -> Option<Matrix<bool>> {
    let mut options = Vec::new();

    let mut matrix = matrix.clone();
    for _ in 0..4 {
        matrix = matrix.rotate_cw();

        options.push(matrix.clone());
        options.push(matrix.flip());
    }

    for option in options {
        if rules.contains_key(&option) {
            return Some(rules[&option].clone());
        }
    }


    None
}

fn count_on(matrix: &Matrix<bool>) -> usize {
    matrix
        .data
        .iter()
        .fold(0, |count, val| count + if *val { 1 } else { 0 })
}

fn from_rule_str(input: &str) -> Matrix<bool> {
    let length = input.chars().position(|c| c == '/').expect("Invalid rule");
    let data = str::replace(input, "/", "")
        .chars()
        .map(|c| c == '#')
        .collect();
    Matrix::new(length, data)
}
