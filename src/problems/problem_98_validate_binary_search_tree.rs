//! 98. Validate Binary Search Tree
//!
//! Medium
//!
//! Given the root of a binary tree, determine if it is a valid binary search tree (BST).
//! A valid BST is defined as follows:
//! The left subtree of a node contains only nodes with keys less than the node's key.
//! The right subtree of a node contains only nodes with keys greater than the node's key.
//! Both the left and right subtrees must also be binary search trees.
//!
//! Example 1:
//! Input: root = [2,1,3]
//! Output: true
//!
//! Example 2:
//! Input: root = [5,1,4,null,null,3,6]
//! Output: false
//! Explanation: The root node's value is 5 but its right child's value is 4.
//!
//! Constraints:
//! The number of nodes in the tree is in the range [1, 104].
//! -2^31 <= Node.val <= 2^31 - 1

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type OptionalNode = Option<Rc<RefCell<TreeNode>>>;

pub fn is_bst_valid(root: OptionalNode) -> bool {
    is_bst_valid_helper(root.as_ref(), None, None)
}

fn is_bst_valid_helper(
    node: Option<&Rc<RefCell<TreeNode>>>,
    lower_limit: Option<i32>,
    upper_limit: Option<i32>,
) -> bool {
    if node.is_none() {
        return true;
    }

    if let Some(limit) = lower_limit {
        if node.unwrap().borrow().val <= limit {
            return false;
        }
    }

    if let Some(limit) = upper_limit {
        if node.unwrap().borrow().val >= limit {
            return false;
        }
    }

    if !is_bst_valid_helper(
        node.unwrap().borrow().left.as_ref(),
        lower_limit,
        Some(node.unwrap().borrow().val),
    ) {
        return false;
    }

    if !is_bst_valid_helper(
        node.unwrap().borrow().right.as_ref(),
        Some(node.unwrap().borrow().val),
        upper_limit,
    ) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bst_valid_1() {
        let tree = TreeNode::from_arr(&[Some(2), Some(1), Some(3)]);

        assert!(is_bst_valid(tree));
    }

    #[test]
    fn test_is_bst_valid_2() {
        let tree = TreeNode::from_arr(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);

        assert!(!is_bst_valid(tree));
    }

    #[test]
    fn test_is_bst_valid_3() {
        let tree = TreeNode::from_arr(&[Some(2), Some(2), Some(2)]);

        assert!(!is_bst_valid(tree));
    }
}
