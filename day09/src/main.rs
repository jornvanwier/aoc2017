fn main() {
    // let input = include_str!("example");
    let input = include_str!("input");

    let mut open = 0;
    let mut in_garbage = false;
    let mut skip = false;

    let mut score_one = 0;
    let mut score_two = 0;

    for c in input.chars() {
        if skip {
            skip = false;
            continue;
        }

        if in_garbage {
            match c {
                '!' => skip = true,
                '>' => in_garbage = false,
                _ => score_two += 1,
            }
        } else {
            match c {
                '<' => in_garbage = true,
                '{' => {
                    open += 1;
                }
                '}' => {
                    score_one += open;
                    open -= 1;
                }
                _ => {}
            }
        }
    }

    println!("1 {}", score_one);
    println!("2 {}", score_two);
}
