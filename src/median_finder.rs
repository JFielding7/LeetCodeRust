struct Tree {
    root_val: i32,
    count: usize,
    height: usize,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>
}

impl Tree {
    fn new(root_val: i32) -> Self {
        Tree { root_val, count: 1, height: 1, left: None, right: None }
    }

    fn print(&self) {
        print!("[{}: {}] ", self.root_val, self.count);
    }
}

pub(crate) struct MedianFinder {
    root: Option<Box<Tree>>
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder { root: None }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.root.is_none() {
            self.root = Option::from(Box::from(Tree::new(num)));
        }
        else {
            let mut curr = &mut self.root;
            while let Some(ref mut node) = curr {
                node.count += 1;
                if node.root_val > num {
                    curr = &mut node.left;
                } else {
                    curr = &mut node.right;
                }
            }
            let _ = curr.insert(Box::from(Tree::new(num)));
        }
    }

    pub(crate) fn get(&self, mut i: usize) -> i32 {
        let mut curr_opt = &self.root;
        if curr_opt.is_none() {
            return 0;
        }
        loop {
            let curr = curr_opt.as_ref().unwrap();
            let left_count = if let Some(ref node) = curr.left { node.count } else { 0 };
            if left_count == i {
                return curr.root_val;
            }
            else if left_count > i {
                curr_opt = &curr.left;
            }
            else {
                curr_opt = &curr.right;
                i -= left_count + 1;
            }
        }
    }

    fn recursive_print(node_option: &Option<Box<Tree>>) {
        if let Some(node) = node_option {
            Self::recursive_print(&node.left);
            node.print();
            Self::recursive_print(&node.right);
        }
    }

    pub fn print(&self) {
        Self::recursive_print(&self.root);
    }

    fn find_median(&self) -> f64 {
        let length = self.root.as_ref().unwrap().count;
        (self.get(length - 1 >> 1) + self.get(length >> 1)) as f64 / 2.0
    }
}