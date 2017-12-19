fn main() {
    let input = 344;

    let mut position = 0;
    let mut buffer = vec![0];

    let max = 2017;

    for i in 1..max + 1 {
        let len = buffer.len();
        let index = 1 + (position + input) % len;
        buffer.insert(index, i);
        position = index;
    }

    println!("Part 1: {}", buffer[position + 1]);

    let mut answer = 0;
    let mut next = 0;
    for i in 1..50_000_000 {
        next = (next + input) % i;
        if next == 0 {
            answer = i;
        }

        next += 1;
    }

    println!("Part 2: {}", answer);
}