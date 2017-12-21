fn main() {
    let input = include_str!("input");
    let input = include_str!("ruurd");

    let grid: Vec<Vec<char>> = input.split("\n").map(|l| l.chars().collect()).collect();

    const HEADINGS: &[Heading] = &[Heading::Up, Heading::Down, Heading::Left, Heading::Right];
    let mut heading = Heading::Down;

    let mut y = 0;
    let mut x = grid[y].iter().position(|c| c == &'|').unwrap();

    let mut encounted = Vec::new();

    let mut two = 0;

    loop {
        match grid[y][x] {
            '|' | '-' => {}
            '+' => {
                for h in HEADINGS {
                    if h == &heading.opposite() {
                        continue;
                    }
                    if let Some((x, y)) = h.add((x, y)) {
                        let val = grid[y][x];
                        if val == '|' || val == '-' {
                            heading = *h;
                            break;
                        }
                    }
                }
            }
            c if c.is_alphabetic() => encounted.push(c),
            _ => break,
        }

        two += 1;
        let (next_x, next_y) = heading.add((x, y)).unwrap();
        x = next_x;
        y = next_y;
    }

    println!("Part 1: {}", encounted.iter().collect::<String>());
    println!("Part 2: {}", two);
}

#[derive(PartialEq, Clone, Copy)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Heading {
    fn add(self, rhs: (usize, usize)) -> Option<(usize, usize)> {
        use Heading::*;
        match self {
            Up => {
                let (x, y) = rhs;
                if y == 0 { None } else { Some((x, y - 1)) }

            }
            Down => {
                let (x, y) = rhs;
                Some((x, y + 1))
            }
            Left => {
                let (x, y) = rhs;
                if x == 0 { None } else { Some((x - 1, y)) }
            }
            Right => {
                let (x, y) = rhs;
                Some((x + 1, y))
            }
        }
    }

    fn opposite(self) -> Self {
        use Heading::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}