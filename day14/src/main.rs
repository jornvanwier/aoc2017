use std::num::Wrapping;
use std::fmt::Write;

fn main() {
    let input = "stpzcrnm";
    // let input = "flqrgnkx";
    // let input = "a0c2017";

    let row_hashes: Vec<String> = (0..128)
        .map(|n| knot_hash(&[input, &n.to_string()].join("-")))
        .collect();

    let binary_sequences: Vec<String> = row_hashes.iter().map(|h| hash_to_binary(&h)).collect();

    let mut count = 0;

    binary_sequences.iter().for_each(|b| {
        b.chars().for_each(|c| {
            if c == '1' {
                count += 1
            }
        })
    });

    println!("Part 1: {}", count);

    let mut regions = 0;

    let mut grid: Vec<Vec<char>> = binary_sequences
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    for (y, b) in binary_sequences.iter().enumerate() {
        for (x, _) in b.chars().enumerate() {
            if at_position((x, y), &grid) == '1' {
                regions += 1;
                flow_region((x, y), &mut grid)
            }
        }
    }

    println!("Part 2: {}", regions);
}

fn flow_region(position: (usize, usize), grid: &mut Vec<Vec<char>>) {
    let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    set_at_position('x', position, grid);

    for direction in directions {
        let (x, y) = position;
        let (dir_x, dir_y) = direction;

        let n_x = x as i32 + dir_x;
        let n_y = y as i32 + dir_y;

        if check_bounds(n_y, grid.len()) && check_bounds(n_x, grid.first().unwrap().len()) {
            let neighbour_pos = (n_x as usize, n_y as usize);
            if at_position(neighbour_pos, grid) == '1' {
                flow_region(neighbour_pos, grid)
            }
        }
    }
}

fn at_position(position: (usize, usize), grid: &Vec<Vec<char>>) -> char {
    let (x, y) = position;
    grid[y][x]
}

fn set_at_position(val: char, position: (usize, usize), grid: &mut Vec<Vec<char>>) {
    let (x, y) = position;
    grid[y][x] = val;
}

fn check_bounds(coord: i32, size: usize) -> bool {
    match coord {
        _ if coord < 0 => false,
        _ if coord as usize >= size => false,
        _ => true,
    }
}

fn hash_to_binary(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        let hex = i32::from_str_radix(&c.to_string(), 16).expect("Invalid hex");
        let _ = write!(&mut result, "{:04b}", hex);
    }

    result
}

fn knot_hash(input: &str) -> String {
    let mut list = (0..256).collect();

    let mut position = 0;
    let mut skip_size = 0;

    let mut length_list: Vec<usize> = input.chars().map(|c| c as usize).collect();
    length_list.append(&mut vec![17, 31, 73, 47, 23]);

    for _ in 0..64 {
        for length in &mut length_list {
            reverse_segment(position, *length, &mut list);
            position += *length + skip_size;
            skip_size += 1;
        }
    }

    assert_eq!(list.len(), 256);
    let dense: String = list.chunks(16)
        .map(|chunk| {
            let val = chunk.iter().skip(1).fold(chunk[0], |b, item| b ^ item);
            format!("{:0>2x}", val)
        })
        .collect();

    assert_eq!(dense.len(), 32);

    dense
}

fn reverse_segment(from: usize, length: usize, collection: &mut Vec<i32>) {
    let list_length = collection.len();
    let mut take_length = length;
    for position in from..from + take_length / 2 {
        collection.swap(
            position % list_length,
            (position + take_length - 1) % list_length,
        );
        take_length = (Wrapping(take_length) - Wrapping(2)).0
    }
}
