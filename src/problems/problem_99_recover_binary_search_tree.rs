//! 99. Recover Binary Search Tree
//!
//! Medium
//!
//! You are given the root of a binary search tree (BST), where the values of exactly two nodes of the tree were swapped by mistake. Recover the tree without changing its structure.
//!
//! Example 1:
//! Input: root = [1,3,null,null,2]
//! Output: [3,1,null,null,2]
//! Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.
//!
//! Example 2:
//! Input: root = [3,1,4,null,null,2]
//! Output: [2,1,4,null,null,3]
//! Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.
//!
//! Constraints:
//! The number of nodes in the tree is in the range [2, 1000].
//! -2^31 <= Node.val <= 2^31 - 1
//!
//! Follow up: A solution using O(n) space is pretty straight-forward. Could you devise a constant O(1) space solution?

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn recover_tree(root: &mut Node) {
    let mut first = Node::None;
    let mut second = Node::None;

    let mut previous = Node::None;

    let mut curr = root.clone();
    let mut stack = vec![];

    while curr.is_some() || !stack.is_empty() {
        if let Some(node) = curr {
            stack.push(node.clone());
            curr = node.borrow().left.clone();
        } else if let Some(node) = &stack.pop() {
            let node_val = node.borrow().val;

            if let Some(prev_node) = &previous {
                let prev_node_val = prev_node.borrow().val;

                if prev_node_val > node_val {
                    if first.is_none() {
                        first = Some(Rc::clone(prev_node));
                        second = Some(Rc::clone(node));
                    } else {
                        second = Some(Rc::clone(node));
                        break;
                    }
                }
            }

            previous = Some(Rc::clone(node));
            curr = node.borrow().right.clone();
        }
    }

    if let (Some(first), Some(second)) = (first, second) {
        std::mem::swap(&mut first.borrow_mut().val, &mut second.borrow_mut().val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_tree_1() {
        let mut root = TreeNode::from_arr(&[Some(1), Some(3), None, None, Some(2)]);
        let correct = TreeNode::from_arr(&[Some(3), Some(1), None, None, Some(2)]);

        recover_tree(&mut root);

        assert_eq!(root, correct);
    }

    #[test]
    fn test_recover_tree_2() {
        let mut root = TreeNode::from_arr(&[Some(3), Some(1), Some(4), None, None, Some(2)]);
        let correct = TreeNode::from_arr(&[Some(2), Some(1), Some(4), None, None, Some(3)]);

        recover_tree(&mut root);

        assert_eq!(root, correct);
    }
}
