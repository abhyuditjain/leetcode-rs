//! 101. Symmetric Tree
//!
//! Easy
//!
//! Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
//!
//! Example 1:
//! Input: root = [1,2,2,3,4,4,3]
//! Output: true
//!
//! Example 2:
//! Input: root = [1,2,2,null,3,null,3]
//! Output: false
//!
//! Constraints:
//! The number of nodes in the tree is in the range [1, 1000].
//! -100 <= Node.val <= 100

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn is_symmetric(root: Node) -> bool {
    is_symmetric_priv(&root, &root)
}

fn is_symmetric_priv(left: &Node, right: &Node) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            left.borrow().val == right.borrow().val
                && is_symmetric_priv(&left.borrow().left, &right.borrow().right)
                && is_symmetric_priv(&left.borrow().right, &right.borrow().left)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symmetric_true() {
        let tree = TreeNode::build_tree(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);

        assert!(is_symmetric(tree));
    }

    #[test]
    fn test_is_symmetric_false() {
        let tree = TreeNode::build_tree(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]);

        assert!(!is_symmetric(tree));
    }
}
