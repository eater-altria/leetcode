use std::{cell::RefCell, rc::Rc};

use leetcode::rs::leetcode_102::{TreeNode, level_order};

#[test]
fn case_1() {
    let node_15 = TreeNode::new(15);
    let node_7 = TreeNode::new(7);
    let mut node_20 = TreeNode::new(20);
    node_20.left = Some(Rc::new(RefCell::new(node_15)));
    node_20.right = Some(Rc::new(RefCell::new(node_7)));
    let node_9 = TreeNode::new(9);
    let mut node_root = TreeNode::new(3);
    node_root.left = Some(Rc::new(RefCell::new(node_9)));
    node_root.right = Some(Rc::new(RefCell::new(node_20)));
    let result = level_order(Some(Rc::new(RefCell::new(node_root))));

    assert_eq!(result, vec![vec![3], vec![9, 20], vec![15, 7],]);
}

#[test]
fn case_2() {
    let node_root = TreeNode::new(1);
    let result = level_order(Some(Rc::new(RefCell::new(node_root))));

    assert_eq!(result, vec![vec![1],]);
}

#[test]
fn case_3() {
    let result = level_order(None);
    assert_eq!(result, Vec::new() as Vec<Vec<i32>>);
}
