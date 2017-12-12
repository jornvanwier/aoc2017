use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let input = include_str!("input");
    // let input = include_str!("example");

    let preparse: Vec<(i32, Vec<i32>)> = input
        .split("\r\n")
        .map(|l| {
            let mut components = l.split(" <-> ");
            let id: i32 = components.nth(0).unwrap().parse().unwrap();
            let connections: Vec<i32> = components
                .nth(0) // not 1 because we're iterating
                .expect("no element at 1")
                .split(", ")
                .map(|c| c.parse().expect("not a number"))
                .collect();
            (id, connections)
        })
        .collect();

    let mut programs: HashMap<i32, Rc<RefCell<Program>>> = preparse
        .iter()
        .map(|&(id, _)| (id, Rc::new(RefCell::new(Program::new(id)))))
        .collect();

    preparse.iter().for_each(|&(id, ref connections)| {
        programs.get_mut(&id).unwrap().borrow_mut().children = Some(
            connections
                .iter()
                .map(|c_id| programs[c_id].clone())
                .collect(),
        )
    });

    println!("1: {}", programs[&0].borrow().size());

    let mut groups = 0;
    let mut master_set = HashSet::new();
    programs.iter().for_each(|(id, program)| {
        if !master_set.contains(id) {
            groups += 1;
            let group = program.borrow().in_group();

            group.iter().for_each(|c_id| {
                master_set.insert(*c_id);
            });
        }
    });
    
    println!("2: {}", groups);
}

#[derive(Debug, Clone)]
struct Program {
    id: i32,
    children: Option<Vec<Rc<RefCell<Program>>>>,
}

impl Program {
    fn new(id: i32) -> Self {
        Program { id, children: None }
    }

    fn size(&self) -> usize {
        self.calculate_size(&mut HashSet::new())
    }

    fn calculate_size(&self, visited: &mut HashSet<i32>) -> usize {
        if visited.contains(&self.id) {
            return 0;
        }

        visited.insert(self.id);

        match &self.children {
            &Some(ref c) => c.iter()
                .fold(1, |b, a| b + a.borrow().calculate_size(visited)),
            &None => 1,
        }
    }

    fn in_group(&self) -> HashSet<i32> {
        let mut set = HashSet::new();
        self.calculate_in_group(&mut set);
        set
    }

    fn calculate_in_group(&self, visited: &mut HashSet<i32>) {
        if !visited.contains(&self.id) {
            visited.insert(self.id);
            match &self.children {
                &Some(ref children) => children.iter().for_each(|c| {
                    c.borrow().calculate_in_group(visited);
                }),
                &None => {}
            };
        }
    }
}
