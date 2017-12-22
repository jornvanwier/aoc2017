use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");

    let grid: Vec<Vec<bool>> = input
        .split("\r\n")
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    let mut virus = InfiniteVirus::from_start_grid(&grid);

    // const ITERATIONS: usize = 7;
    // const ITERATIONS: usize = 70;
    // const ITERATIONS: usize = 10000;
    const ITERATIONS: usize = 10000000;

    for _ in 0..ITERATIONS {
        virus.burst();
    }

    // println!("{}", virus);
    println!("Part 2: {}", virus.infect_count);
}

#[derive(Debug)]
struct InfiniteVirus {
    states: HashMap<(isize, isize), NodeState>,
    infect_count: usize,
    position: (isize, isize),
    heading: Heading,
}

impl InfiniteVirus {
    fn from_start_grid(grid: &Vec<Vec<bool>>) -> Self {
        let size = grid.len() as isize;

        assert_eq!(size, grid[0].len() as isize);
        assert!(size % 2 != 0);

        let mut states = HashMap::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                let coord = (x as isize - size / 2, y as isize - size / 2);
                if *val {
                    states.insert(coord, NodeState::Infected);
                } else {
                    states.insert(coord, NodeState::Clean);
                }
            }
        }

        Self {
            states,
            infect_count: 0,
            position: (0, 0),
            heading: Heading::Up,
        }
    }

    fn burst(&mut self) {
        let node = self.states.entry(self.position).or_insert(NodeState::Clean);

        match *node {
            NodeState::Clean => {
                self.heading = self.heading.left();
            }
            NodeState::Weakened => {}
            NodeState::Infected => {
                self.heading = self.heading.right();
            }
            NodeState::Flagged => {
                self.heading = self.heading.reverse();
            }
        }

        node.advance();

        if node == &NodeState::Infected {
            self.infect_count += 1;
        }

        self.position = self.heading.go(self.position);
    }
}

impl Display for InfiniteVirus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        const GROW: isize = 2;

        let xs: Vec<isize> = self.states.keys().map(|&(x, _)| x).collect();
        let x_min = xs.iter().min().unwrap();
        let x_max = xs.iter().max().unwrap();
        let x_size = x_max + x_min.abs() + 1 + GROW * 2;

        let ys: Vec<isize> = self.states.keys().map(|&(_, y)| y).collect();
        let y_min = ys.iter().min().unwrap();
        let y_max = ys.iter().max().unwrap();
        let y_size = y_max + y_min.abs() + 1 + GROW * 2;

        let row = vec![(NodeState::Clean, false); (x_size) as usize];

        let mut grid = Vec::new();
        for _ in 0..y_size {
            grid.push(row.clone())
        }

        for (&(x, y), val) in &self.states {
            grid[(y + y_min.abs() + GROW) as usize][(x + x_min.abs() + GROW) as usize] =
                (*val, false);
        }

        {
            let (pos_x, pos_y) = self.position;
            let pos_node = &mut grid[(pos_y + y_min.abs() + GROW) as usize]
                [(pos_x + x_min.abs() + GROW) as usize];
            *pos_node = (pos_node.0, true);
        }

        writeln!(f, "{:?}", self.heading)?;
        for grid_row in grid {
            for (val, pos) in grid_row {
                let c = match val {
                    NodeState::Clean => '.',
                    NodeState::Weakened => 'W',
                    NodeState::Flagged => 'F',
                    NodeState::Infected => '#',
                };

                if pos {
                    write!(f, "[{}]", c)?;
                } else {
                    write!(f, " {} ", c)?;
                }
            }
            writeln!(f, "")?;
        }

        writeln!(f, "")
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    fn advance(&mut self) {
        use NodeState::*;

        *self = match *self {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean,
        }
    }
}

#[derive(Debug)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Heading {
    fn left(&self) -> Self {
        use Heading::*;
        match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn right(&self) -> Self {
        use Heading::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn reverse(&self) -> Self {
        // lazy
        self.right().right()
    }

    fn go(&self, coord: (isize, isize)) -> (isize, isize) {
        use Heading::*;
        let (x, y) = coord;
        match *self {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        }
    }
}
