use std::num::Wrapping;

fn main() {
    let (input, mut list) = (include_str!("input"), (0..256).collect());

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

    println!("{:?}", list);

    let dense: String = list.chunks(16).map(|chunk| {
        format!("{:x}", chunk.iter().skip(1).fold(chunk[0], |b, item| b ^ item))
    }).collect();

    println!("{}", dense);
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
