use std::cmp::min;

struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: vec![] }
    }

    fn push(&mut self, val: i32) {
        let curr_min = if let Some(node) = self.stack.last() { node.1 }
        else { i32::MAX };
        self.stack.push((val, min(curr_min, val)));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}