// 226. Invert Binary Tree
// From: https://leetcode.com/problems/invert-binary-tree
//
// Given the root of a binary tree, invert the tree, and return its root.
//
// Example 1:
//
// Input: root = [4,2,7,1,3,6,9]
// Output: [4,7,2,9,6,3,1]
//
// Example 2:
//
// Input: root = [2,1,3]
// Output: [2,3,1]
//
// Example 3:
//
// Input: root = []
// Output: []
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
//
// Solved in 10 minutes, 23 seconds

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    Self {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

//

use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root) = root else {
            return None;
        };

        invert_node(root.clone());

        Some(root)
    }
}

fn invert_node(node_rc: Rc<RefCell<TreeNode>>) {
    let mut node_mut = node_rc.borrow_mut();

    // invert left
    if let Some(left_rc) = node_mut.left.clone() {
        invert_node(left_rc);
    }

    // invert right
    if let Some(right_rc) = node_mut.right.clone() {
        invert_node(right_rc);
    }

    // switch left and right
    let left = node_mut.left.clone();
    let right = node_mut.right.clone();

    node_mut.right = left;
    node_mut.left = right;
}