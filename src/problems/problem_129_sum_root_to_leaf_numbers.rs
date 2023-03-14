//! 129. Sum Root to Leaf Numbers
//!
//! Medium
//!
//! You are given the root of a binary tree containing digits from 0 to 9 only.
//! Each root-to-leaf path in the tree represents a number.
//!
//! For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
//!
//! Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a 32-bit integer.
//!
//! A leaf node is a node with no children.
//!
//! Example 1:
//! Input: root = [1,2,3]
//! Output: 25
//! Explanation:
//! The root-to-leaf path 1->2 represents the number 12.
//! The root-to-leaf path 1->3 represents the number 13.
//! Therefore, sum = 12 + 13 = 25.
//!
//! Example 2:
//! Input: root = [4,9,0,5,1]
//! Output: 1026
//! Explanation:
//! The root-to-leaf path 4->9->5 represents the number 495.
//! The root-to-leaf path 4->9->1 represents the number 491.
//! The root-to-leaf path 4->0 represents the number 40.
//! Therefore, sum = 495 + 491 + 40 = 1026.
//!
//! Constraints:
//! The number of nodes in the tree is in the range [1, 1000].
//! 0 <= Node.val <= 9
//! The depth of the tree will not exceed 10.

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    sum_helper(root.as_ref(), 0)
}

fn sum_helper(root: Option<&Rc<RefCell<TreeNode>>>, curr_sum: i32) -> i32 {
    if root.is_none() {
        return 0;
    }

    let root = root.unwrap();

    let curr_sum = curr_sum * 10 + root.borrow().val;

    if root.borrow().left.is_none() && root.borrow().right.is_none() {
        return curr_sum;
    }

    let left_sum = sum_helper(root.borrow().left.as_ref(), curr_sum);
    let right_sum = sum_helper(root.borrow().right.as_ref(), curr_sum);

    left_sum + right_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_numbers_1() {
        let tree = TreeNode::build_tree(&[Some(1), Some(2), Some(3)]);

        assert_eq!(sum_numbers(tree), 25);
    }

    #[test]
    fn test_sum_numbers_2() {
        let tree = TreeNode::build_tree(&[Some(4), Some(9), Some(0), Some(5), Some(1)]);

        assert_eq!(sum_numbers(tree), 1026);
    }

    #[test]
    fn test_sum_numbers_3() {
        let tree = TreeNode::build_tree(&[]);

        assert_eq!(sum_numbers(tree), 0);
    }
}
