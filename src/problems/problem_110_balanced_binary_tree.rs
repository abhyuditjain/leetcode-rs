//! 110. Balanced Binary Tree
//!
//! Easy
//!
//! Given a binary tree, determine if it is height-balanced.
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: true
//!
//! Example 2:
//! Input: root = [1,2,2,3,3,null,null,4,4]
//! Output: false
//!
//! Example 3:
//! Input: root = []
//! Output: true
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 5000].
//! -10^4 <= Node.val <= 10^4

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

use super::problem_104_maximum_depth_binary_tree::depth;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(node) => {
            let left_right_subtree_height_diff =
                depth(node.borrow().left.clone()).abs_diff(depth(node.borrow().right.clone()));

            left_right_subtree_height_diff <= 1
                && is_balanced(node.borrow().left.clone())
                && is_balanced(node.borrow().right.clone())
        }
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        assert!(is_balanced(root));
    }

    #[test]
    fn test_is_balanced_2() {
        let root = TreeNode::from_arr(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);

        assert!(!is_balanced(root));
    }

    #[test]
    fn test_is_balanced_3() {
        let root = TreeNode::from_arr(&[]);

        assert!(is_balanced(root));
    }
}
