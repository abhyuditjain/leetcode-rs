//! 107. Binary Tree Level Order Traversal II
//!
//! Medium
//!
//! Given the root of a binary tree, return the bottom-up level order traversal of its nodes' values. (i.e., from left to right, level by level from leaf to root).
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [[15,7],[9,20],[3]]
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

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

use super::problem_102_binary_tree_level_order_traversal::{level_order, Algorithm};

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let levels = level_order(root, Algorithm::Bfs);
    levels.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order_bottom_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(
            level_order_bottom(root),
            vec![vec![15, 7], vec![9, 20], vec![3]]
        );
    }

    #[test]
    fn test_level_order_bottom_2() {
        let root = TreeNode::from_arr(&[Some(1)]);
        assert_eq!(level_order_bottom(root), vec![vec![1]]);
    }

    #[test]
    fn test_level_order_bottom_3() {
        let root = TreeNode::from_arr(&[]);
        assert_eq!(level_order_bottom(root), vec![vec![]; 0]);
    }
}
