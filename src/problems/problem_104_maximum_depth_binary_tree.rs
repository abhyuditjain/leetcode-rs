//! 104. Maximum Depth of Binary Tree
//!
//! Easy
//!
//! Given the root of a binary tree, return its maximum depth.
//! A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: 3
//!
//! Example 2:
//! Input: root = [1,null,2]
//! Output: 2
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 104].
//! -100 <= Node.val <= 100

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

pub fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
    match root {
        Some(node) => {
            1 + std::cmp::max(
                depth(node.borrow().left.clone()),
                depth(node.borrow().right.clone()),
            )
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(depth(root), 3);
    }

    #[test]
    fn test_depth_2() {
        let root = TreeNode::from_arr(&[Some(1)]);
        assert_eq!(depth(root), 1);
    }

    #[test]
    fn test_depth_3() {
        let root = TreeNode::from_arr(&[]);
        assert_eq!(depth(root), 0);
    }

    #[test]
    fn test_depth_4() {
        let root = TreeNode::from_arr(&[Some(1), None, Some(2)]);
        assert_eq!(depth(root), 2);
    }
}
