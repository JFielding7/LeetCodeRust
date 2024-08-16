use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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

struct StackNode {
    node: Rc<RefCell<TreeNode>>,
    offset: usize,
    serialized_left: bool
}

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn push_node(node_ref: Rc<RefCell<TreeNode>>, stack: &mut Vec<StackNode>, serialized: &mut Vec<i32>) {
        stack.push(StackNode {
            node: node_ref,
            offset: serialized.len(),
            serialized_left: false
        });
        serialized.extend(vec![0; 2]);
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root == None {
            return "".to_string()
        }
        let mut buffer: Vec<i32> = vec![0; 2];
        let mut stack = vec![StackNode { node: root.unwrap(), offset: 0, serialized_left: false }];

        while let Some(node) = stack.last_mut() {
            let tree_node_rc = node.node.clone();
            let tree_node = tree_node_rc.borrow();

            if node.serialized_left {
                buffer[node.offset + 1] = tree_node.val;
                let right_offset = (buffer.len() - node.offset) as i32;

                if let Some(right) = tree_node.right.as_ref() {
                    buffer[node.offset] = right_offset;
                    stack.pop();
                    Self::push_node(right.clone(), &mut stack, &mut buffer);
                }
                else {
                    buffer[node.offset] = right_offset + 1;
                    stack.pop();
                }
            }
            else {
                node.serialized_left = true;
                if let Some(left) = tree_node.left.as_ref() {
                    Self::push_node(left.clone(), &mut stack, &mut buffer);
                }
            }
        }
        buffer.iter().map(|word| format!("{:>5}", word)).collect::<Vec<String>>().join("")
    }

    fn create_node(buffer: &Vec<i32>, offset: usize) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val: buffer[offset],
            left: None,
            right: None
        }))
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let length = data.len();
        if length == 0 { return None; }

        let words = length / 5;
        let mut buffer: Vec<i32> = (0..words).map(
            |i| data[i * 5..i * 5 + 5].trim().parse::<i32>().unwrap()
        ).collect();

        let root = Rc::new(RefCell::new(TreeNode {
            val: buffer[1],
            left: None,
            right: None
        }));

        let mut queue = VecDeque::from([(root.clone(), 0)]);
        while let Some((mut node, offset)) = queue.pop_front() {
            let right_offset = buffer[offset];
            if right_offset > 3 {
                let left_child = Self::create_node(&buffer, offset + 3);
                node.borrow_mut().left = Some(left_child.clone());
                queue.push_back((left_child, offset + 2))
            }
            if (right_offset & 1) == 0 {
                let right_child = Self::create_node(&buffer, offset + right_offset as usize + 1);
                node.borrow_mut().right = Some(right_child.clone());
                queue.push_back((right_child, offset + right_offset as usize));
            }
        }
        Option::from(root)
    }
}
