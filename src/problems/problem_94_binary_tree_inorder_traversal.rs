//! 94. Binary Tree Inorder Traversal
//!
//! Easy
//!
//! Given the root of a binary tree, return the inorder traversal of its nodes' values.
//!
//! Example 1:
//! Input: root = [1,null,2,3]
//! Output: [1,3,2]
//!
//! Example 2:
//! Input: root = []
//! Output: []
//!
//! Example 3:
//! Input: root = [1]
//! Output: [1]
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 100].
//! -100 <= Node.val <= 100

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type OptionalNode = Option<Rc<RefCell<TreeNode>>>;

pub fn inorder_traversal(root: OptionalNode) -> Vec<i32> {
    let mut values = vec![];

    inorder(&root, &mut values);

    values
}

fn inorder(node: &OptionalNode, values: &mut Vec<i32>) {
    if let Some(node) = node {
        let borrowed = node.borrow();
        inorder(&borrowed.left, values);
        values.push(borrowed.val);
        inorder(&borrowed.right, values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder_traversal_1() {
        let root = TreeNode::build_tree(&[Some(1), None, Some(2), Some(3)]);
        assert_eq!(inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn test_inorder_traversal_2() {
        let root = TreeNode::build_tree(&[]);
        assert_eq!(inorder_traversal(root), vec![]);
    }

    #[test]
    fn test_inorder_traversal_3() {
        let root = TreeNode::build_tree(&[Some(1)]);
        assert_eq!(inorder_traversal(root), vec![1]);
    }
}
