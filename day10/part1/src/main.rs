fn main() {
    const EXAMPLE: bool = false;

    let (input, mut list) = if EXAMPLE {
        (include_str!("example"), (0..5).collect())
    } else {
        (include_str!("input"), (0..256).collect())
    };

    let mut position = 0;
    let mut skip_size = 0;

    let length_list: Vec<usize> = input.split(',').map(|n| n.parse().unwrap()).collect();

    for length in length_list {
        reverse_segment(position, length, &mut list);
        position += length + skip_size;
        skip_size += 1;
    }

    println!("{}", list[0] * list[1]);
}

fn reverse_segment(from: usize, length: usize, collection: &mut Vec<i32>) {
    println!(
        "Start\t{:?}:{}",
        collection
            .iter()
            .cycle()
            .skip(from)
            .take(length)
            .collect::<Vec<&i32>>(),
        length
    );
    let list_length = collection.len();
    let mut take_length = length;
    for position in from..from+take_length / 2 {
        collection.swap(
            position % list_length,
            (position + take_length - 1) % list_length,
        );
        take_length -= if take_length > 2 { 2 } else { 0 };
    }
    println!(
        "End\t{:?}:{}",
        collection
            .iter()
            .cycle()
            .skip(from)
            .take(length)
            .collect::<Vec<&i32>>(),
        length
    );
}
