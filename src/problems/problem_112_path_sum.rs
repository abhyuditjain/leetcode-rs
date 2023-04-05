//! 112. Path Sum
//!
//! Easy
//!
//! Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
//! A leaf is a node with no children.
//!
//! Example 1:
//! Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
//! Output: true
//! Explanation: The root-to-leaf path with the target sum is shown.
//!
//! Example 2:
//! Input: root = [1,2,3], targetSum = 5
//! Output: false
//! Explanation: There two root-to-leaf paths in the tree:
//! (1 --> 2): The sum is 3.
//! (1 --> 3): The sum is 4.
//! There is no root-to-leaf path with sum = 5.
//!
//! Example 3:
//! Input: root = [], targetSum = 0
//! Output: false
//! Explanation: Since the tree is empty, there are no root-to-leaf paths.
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 5000].
//! -1000 <= Node.val <= 1000
//! -1000 <= targetSum <= 1000

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn has_path_sum(root: Node, mut target: i32) -> bool {
    match root {
        None => false,
        Some(node) => {
            target -= node.borrow().val;

            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return target == 0;
            }

            has_path_sum(node.borrow().left.clone(), target)
                || has_path_sum(node.borrow().right.clone(), target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_path_sum_1() {
        let root = TreeNode::from_arr(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);

        assert!(has_path_sum(root, 22));
    }

    #[test]
    fn test_has_path_sum_2() {
        let root = TreeNode::from_arr(&[Some(1), Some(2), Some(3)]);

        assert!(!has_path_sum(root, 5));
    }

    #[test]
    fn test_has_path_sum_3() {
        let root = TreeNode::from_arr(&[]);

        assert!(!has_path_sum(root, 0));
    }
}
