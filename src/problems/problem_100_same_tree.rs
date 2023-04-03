//! 100. Same Tree
//!
//! Easy
//!
//! Given the roots of two binary trees p and q, write a function to check if they are the same or not.
//! Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
//!
//! Example 1:
//! Input: p = [1,2,3], q = [1,2,3]
//! Output: true
//!
//! Example 2:
//! Input: p = [1,2], q = [1,null,2]
//! Output: false
//!
//! Example 3:
//! Input: p = [1,2,1], q = [1,1,2]
//! Output: false
//!
//! Constraints:
//! The number of nodes in both trees is in the range [0, 100].
//! -10^4 <= Node.val <= 10^4

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn is_same_tree(p: Node, q: Node) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(p), Some(q)) => {
            p.borrow().val == q.borrow().val
                && is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                && is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree_1() {
        let p = TreeNode::from_arr(&[Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_arr(&[Some(1), Some(2), Some(3)]);

        assert!(is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_2() {
        let p = TreeNode::from_arr(&[Some(1), Some(2)]);
        let q = TreeNode::from_arr(&[Some(1), None, Some(2)]);

        assert!(!is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_3() {
        let p = TreeNode::from_arr(&[Some(1), Some(2), Some(1)]);
        let q = TreeNode::from_arr(&[Some(1), Some(1), Some(2)]);

        assert!(!is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_4() {
        let p = TreeNode::from_arr(&[]);
        let q = TreeNode::from_arr(&[]);

        assert!(is_same_tree(p, q));
    }
}
