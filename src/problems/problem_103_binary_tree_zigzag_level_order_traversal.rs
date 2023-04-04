//! 103. Binary Tree Zigzag Level Order Traversal
//!
//! Medium
//!
//! Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [[3],[20,9],[15,7]]
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
//! -100 <= Node.val <= 100

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut levels = vec![];

    if root.is_none() {
        return levels;
    }

    let mut stack1 = vec![];
    let mut stack2 = vec![];

    stack1.push(root.unwrap());

    while !stack1.is_empty() || !stack2.is_empty() {
        let mut level = vec![];

        while !stack1.is_empty() {
            if let Some(node) = stack1.pop() {
                level.push(node.borrow().val);

                if let Some(left) = node.borrow_mut().left.take() {
                    stack2.push(left);
                }

                if let Some(right) = node.borrow_mut().right.take() {
                    stack2.push(right);
                }
            }
        }

        levels.push(level);

        let mut level = vec![];

        while !stack2.is_empty() {
            if let Some(node) = stack2.pop() {
                level.push(node.borrow().val);

                if let Some(right) = node.borrow_mut().right.take() {
                    stack1.push(right);
                }

                if let Some(left) = node.borrow_mut().left.take() {
                    stack1.push(left);
                }
            }
        }

        if !level.is_empty() {
            levels.push(level);
        }
    }

    levels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag_level_order_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(
            zigzag_level_order(root),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }

    #[test]
    fn test_zigzag_level_order_2() {
        let root = TreeNode::from_arr(&[Some(1)]);
        assert_eq!(zigzag_level_order(root), vec![vec![1]]);
    }

    #[test]
    fn test_zigzag_level_order_3() {
        let root = TreeNode::from_arr(&[]);
        assert_eq!(zigzag_level_order(root), vec![vec![]; 0]);
    }
}
