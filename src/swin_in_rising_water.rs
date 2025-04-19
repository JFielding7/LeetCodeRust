use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

pub struct Solution;

struct Node {
    cell: (i32, i32),
    rank: i32,
    parent: Option<Rc<RefCell<Node>>>,
}

fn find(mut node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    if node.borrow().parent.is_none() {
        return node;
    }

    let mut rep = node.clone();
    let mut parent = node.borrow().parent.clone();
    while let Some(curr) = parent {
        rep = curr.clone();
        parent = curr.borrow().parent.clone();
    }

    let mut curr = node;
    while !Rc::ptr_eq(&curr, &rep) {
        let next = curr.borrow_mut().parent.clone().unwrap();
        curr.borrow_mut().parent = Some(rep.clone());
        curr = next;
    }

    rep
}

fn union(node0: Rc<RefCell<Node>>, node1: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let rep0 = find(node0);
    let rep1 = find(node1);

    if !Rc::ptr_eq(&rep0, &rep1) {
        return if rep0.borrow().rank > rep1.borrow().rank {
            rep0.borrow_mut().rank += 1;
            rep1.borrow_mut().parent = Some(rep0.clone());
            rep0
        } else {
            rep1.borrow_mut().rank += 1;
            rep0.borrow_mut().parent = Some(rep1.clone());
            rep1
        }
    }

    rep0
}

fn add_node(cell: (i32, i32), n: i32, nodes: &mut HashMap<(i32, i32), Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
    let node = Rc::new(RefCell::new(Node { cell, rank: 0, parent: None }));
    nodes.insert(cell, node.clone());

    let mut set = node.clone();

    const DELTA: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for change in DELTA {
        let row = cell.0 + change.0;
        let col = cell.1 + change.1;

        if row >= 0 && row < n && col >= 0 && col < n {
            if let Some(neighbor) = nodes.get(&(row, col)) {
                set = union(set, neighbor.clone());
            }
        }
    }

    node
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        if n == 1 {
            return 0;
        }

        let mut cells = Vec::with_capacity((n * n) as usize);

        for (i, row) in grid.iter().enumerate() {
            for (j, elevation) in row.iter().enumerate() {
                cells.push((-elevation, (i as i32, j as i32)));
            }
        }

        let mut start = None;
        let mut end = None;
        let mut nodes = HashMap::new();
        let mut heap = BinaryHeap::from(cells);

        loop {
            let curr = heap.pop().unwrap();
            let elevation = curr.0;
            let cell = curr.1;
            let node = add_node(cell, n, &mut nodes);

            if cell == (0, 0) {
                start = Some(node);
            } else if cell == (n - 1, n - 1) {
                end = Some(node);
            }

            if start.is_some() && end.is_some() &&
                Rc::ptr_eq(
                    &find(start.as_ref().unwrap().clone()),
                    &find(end.as_ref().unwrap().clone())
                ) {
                return -elevation;
            }
        }
    }
}
