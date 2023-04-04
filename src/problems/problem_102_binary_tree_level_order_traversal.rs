//! 102. Binary Tree Level Order Traversal
//!
//! Medium
//!
//! Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [[3],[9,20],[15,7]]
//!
//! Example 2:
//! Input: root = [1]
//! Output: [[1]]
//!
//! Example 3:
//! Input: root = []
//! Output: []
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 2000].
//! -1000 <= Node.val <= 1000

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::treenode::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut queue = VecDeque::new();

    if let Some(node) = root {
        queue.push_back(node);
    }

    let mut levels = vec![];

    while !queue.is_empty() {
        let mut level = vec![];

        for _ in 0..queue.len() {
            if let Some(node) = queue.pop_front() {
                level.push(node.borrow().val);

                if let Some(left) = node.borrow_mut().left.take() {
                    queue.push_back(left);
                }

                if let Some(right) = node.borrow_mut().right.take() {
                    queue.push_back(right);
                }
            }
        }

        levels.push(level)
    }

    levels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(level_order(root), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn test_level_order_2() {
        let root = TreeNode::from_arr(&[Some(1)]);
        assert_eq!(level_order(root), vec![vec![1]]);
    }

    #[test]
    fn test_level_order_3() {
        let root = TreeNode::from_arr(&[]);
        assert_eq!(level_order(root), vec![vec![]; 0]);
    }
}
