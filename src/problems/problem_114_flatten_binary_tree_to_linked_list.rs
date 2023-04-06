//! 114. Flatten Binary Tree to Linked List
//!
//! Medium
//!
//! Given the root of a binary tree, flatten the tree into a "linked list":
//! The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
//! The "linked list" should be in the same order as a pre-order traversal of the binary tree.
//!
//! Example 1:
//! Input: root = [1,2,5,3,4,null,6]
//! Output: [1,null,2,null,3,null,4,null,5,null,6]
//!
//! Example 2:
//! Input: root = []
//! Output: []
//!
//! Example 3:
//! Input: root = [0]
//! Output: [0]
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 2000].
//! -100 <= Node.val <= 100
//!
//! Follow up: Can you flatten the tree in-place (with O(1) extra space)?

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub enum Algorithm {
    Recursive,
    Morris,
}

pub fn flatten(root: &mut Node, alg: Algorithm) {
    match alg {
        Algorithm::Recursive => flatten_rec(root),
        Algorithm::Morris => flatten_morris(root),
    }
}

fn flatten_morris(root: &mut Node) {
    let mut root = root.clone();
    while let Some(r) = root {
        let mut r = r.borrow_mut();
        if let Some(mut prec) = r.left.clone() {
            loop {
                let right = prec.borrow().right.clone();
                if let Some(ri) = right {
                    prec = ri;
                } else {
                    break;
                }
            }
            let mut b = prec.borrow_mut();
            b.right = r.right.take();
            r.right = r.left.take()
        }
        root = r.right.clone();
    }
}

fn flatten_rec(root: &mut Node) {
    if let Some(root) = root {
        flatten_rec_helper(root.clone());
    }
}

fn flatten_rec_helper(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    {
        let mut node = root.borrow_mut();
        if node.left.is_some() {
            let lr = flatten_rec_helper(Rc::clone(node.left.as_ref().unwrap()));

            lr.borrow_mut().right = node.right.clone();
            node.right = node.left.clone();
            node.left = None;
        }

        if node.right.is_some() {
            return flatten_rec_helper(Rc::clone(node.right.as_ref().unwrap()));
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_rec_1() {
        let mut root =
            TreeNode::from_arr(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
        let correct = TreeNode::from_arr(&[
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);

        flatten_rec(&mut root);

        assert_eq!(root, correct);
    }

    #[test]
    fn test_flatten_rec_2() {
        let mut root = TreeNode::from_arr(&[]);
        let correct = TreeNode::from_arr(&[]);

        flatten_rec(&mut root);

        assert_eq!(root, correct);
    }

    #[test]
    fn test_flatten_rec_3() {
        let mut root = TreeNode::from_arr(&[Some(0)]);
        let correct = TreeNode::from_arr(&[Some(0)]);

        flatten_rec(&mut root);

        assert_eq!(root, correct);
    }

    #[test]
    fn test_flatten_morris_1() {
        let mut root =
            TreeNode::from_arr(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
        let correct = TreeNode::from_arr(&[
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);

        flatten_morris(&mut root);

        assert_eq!(root, correct);
    }

    #[test]
    fn test_flatten_morris_2() {
        let mut root = TreeNode::from_arr(&[]);
        let correct = TreeNode::from_arr(&[]);

        flatten_morris(&mut root);

        assert_eq!(root, correct);
    }

    #[test]
    fn test_flatten_morris_3() {
        let mut root = TreeNode::from_arr(&[Some(0)]);
        let correct = TreeNode::from_arr(&[Some(0)]);

        flatten_morris(&mut root);

        assert_eq!(root, correct);
    }
}
