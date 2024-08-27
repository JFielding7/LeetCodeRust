#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub struct Solution {}

use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::cmp::min;
use std::collections::HashMap;

impl Solution {

    fn find_child_counts(parent: &Option<Rc<RefCell<TreeNode>>>, i: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(parent) = parent {
            let parent_ref = parent.borrow();
            return Self::min_cameras(&parent_ref.left, i, cache) + Self::min_cameras(&parent_ref.right, i, cache);
        }
        0
    }

    fn place_camera(root: &Rc<RefCell<TreeNode>>, i: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        let root_ref = root.borrow();
        Self::find_child_counts(&root_ref.left, i, cache) + Self::find_child_counts(&root_ref.right, i, cache)
    }

    fn min_cameras(node_opt: &Option<Rc<RefCell<TreeNode>>>, i: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node) = node_opt {
            let node_ref = node.borrow();
            if let Some(count) = cache.get(&i) {
                return *count;
            }
            let count = if node_ref.left == None && node_ref.right == None {
                1
            } else if node_ref.left == None {
                Self::place_camera(node_ref.right.as_ref().unwrap(), i + 1, cache)
            } else if node_ref.right == None {
                Self::place_camera(node_ref.left.as_ref().unwrap(), i + 1, cache)
            } else {
                let a = Self::place_camera(node, i, cache) + 1;
                let b = Self::place_camera(node_ref.left.as_ref().unwrap(), i, cache) + Self::min_cameras(&node_ref.right, i, cache) + 1;
                let c = Self::place_camera(node_ref.right.as_ref().unwrap(), i, cache) + Self::min_cameras(&node_ref.left, i, cache) + 1;
                if a < b && a < c { a } else if b < a && b < c { b } else { c }
            };
            cache.insert(i, count);
            return count;
        }
        0
    }

    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}