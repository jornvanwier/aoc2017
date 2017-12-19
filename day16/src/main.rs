use DanceMove::*;

fn main() {
    let input = include_str!("input");
    //    let input = include_str!("example");
    // let input = include_str!("ruurd");

    let instructions: Vec<DanceMove> = input.split(",").map(|s| DanceMove::from_str(s)).collect();

    let ascii_iter = (0..16).map(|x| (x + 'a' as u8) as char);
    let mut programs: Vec<char> = ascii_iter.collect();

    let mut set = Vec::new();

    let times = 1_000_000_000;
    for i in 0..times {
        if set.contains(&programs) {
            println!("{:?}", set);
            println!("Part 2: {} {}", set[times % i].iter().collect::<String>(), i);
            break;
        }

        set.push(programs.clone());

        instructions.iter().for_each(|instruction| {
            instruction.perform(&mut programs)
        });
    }

    let result: String = programs.iter().collect();
    println!("Part 1: {}", result);

    let mut a: Vec<char> = "abcde".chars().collect();
    Spin(1).perform(&mut a);
    println!("{}", a.iter().collect::<String>());
}

#[derive(Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl DanceMove {
    fn from_str(input: &str) -> Self {
        let mut chars = input.chars();
        let instruction = chars.next().unwrap();

        match instruction {
            's' => {
                let count = chars.collect::<String>().parse().unwrap();
                Spin(count)
            }
            'x' => {
                let num_str = chars.collect::<String>();
                let mut nums = num_str.split("/");
                let a = nums.next().unwrap().parse().unwrap();
                let b = nums.next().unwrap().parse().unwrap();
                Exchange(a, b)
            }
            'p' => {
                let a = chars.next().unwrap();
                let b = chars.nth(1).unwrap();
                Partner(a, b)
            }
            _ => unreachable!(),
        }
    }

    fn perform(&self, programs: &mut Vec<char>) {
        match *self {
            Spin(count) => {
                for _ in 0..count {
                    let first;
                    let last;
                    {
                        let mut iter = programs.iter();
                        first = *iter.next().unwrap();
                        last = *iter.last().unwrap();
                    }

                    (1..programs.len())
                        .rev()
                        .zip((1..programs.len() - 1).rev())
                        .for_each(|(a, b)| { programs.swap(a, b); });

                    programs[0] = last;
                    programs[1] = first;
                }
            }
            Exchange(a, b) => {
                programs.swap(a, b);
            }
            Partner(a, b) => {
                let a = programs.iter().position(|c| c == &a).unwrap();
                let b = programs.iter().position(|c| c == &b).unwrap();
                programs.swap(a, b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spin1() {
        let mut r: Vec<char> = "abcde".chars().collect();
        Spin(1).perform(&mut r);
        assert_eq!(r, "eabcd".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_spin3() {
        let mut r: Vec<char> = "abcde".chars().collect();
        Spin(3).perform(&mut r);
        assert_eq!(r, "cdeab".chars().collect::<Vec<char>>());
    }

    #[test]
    fn exchange() {
        let mut r: Vec<char> = "eabcd".chars().collect();
        Exchange(3, 4).perform(&mut r);
        assert_eq!(r, "eabdc".chars().collect::<Vec<char>>());
    }

    #[test]
    fn partner() {
        let mut r: Vec<char> = "eabdc".chars().collect();
        Partner('e', 'b').perform(&mut r);
        assert_eq!(r, "baedc".chars().collect::<Vec<char>>());
    }
}
