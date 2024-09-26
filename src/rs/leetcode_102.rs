// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![root.unwrap()];

    while !queue.is_empty() {
        let mut current_level: Vec<i32> = vec![];
        let level_size = queue.len();
        for _ in 0..level_size {
            let node = queue.remove(0); // 类似于 `shift`
            let node_ref = node.borrow();
            current_level.push(node_ref.val);

            if let Some(left) = &node_ref.left {
                queue.push(left.clone());
            }
            if let Some(right) = &node_ref.right {
                queue.push(right.clone());
            }
        }
        result.push(current_level);
    }

    return result;
}
