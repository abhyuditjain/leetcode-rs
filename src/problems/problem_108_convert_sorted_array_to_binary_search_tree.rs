//! 108. Convert Sorted Array to Binary Search Tree
//!
//! Easy
//!
//! Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.
//!
//! Example 1:
//! Input: nums = [-10,-3,0,5,9]
//! Output: [0,-3,9,-10,null,5]
//! Explanation: [0,-10,5,null,-3,null,9] is also accepted:
//!
//! Example 2:
//! Input: nums = [1,3]
//! Output: [3,1]
//! Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
//!
//! Constraints:
//! 1 <= nums.length <= 10^4
//! -10^4 <= nums[i] <= 10^4
//! nums is sorted in a strictly increasing order.

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn sorted_array_to_bst(nums: &[i32]) -> Node {
    if nums.is_empty() {
        return Node::None;
    }

    let mid = nums.len() / 2;

    Some(Rc::new(RefCell::new(TreeNode {
        val: nums[mid],
        left: sorted_array_to_bst(&nums[..mid]),
        right: sorted_array_to_bst(&nums[mid + 1..]),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array_to_bst_1() {
        let correct = TreeNode::from_arr(&[Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]);

        assert_eq!(sorted_array_to_bst(&[-10, -3, 0, 5, 9]), correct);
    }

    #[test]
    fn test_sorted_array_to_bst_2() {
        let correct = TreeNode::from_arr(&[Some(3), Some(1)]);

        assert_eq!(sorted_array_to_bst(&[1, 3]), correct);
    }

    #[test]
    fn test_sorted_array_to_bst_3() {
        let correct = TreeNode::from_arr(&[]);

        assert_eq!(sorted_array_to_bst(&[]), correct);
    }
}
