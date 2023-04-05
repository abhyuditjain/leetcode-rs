//! 109. Convert Sorted List to Binary Search Tree
//!
//! Medium
//!
//! Given the head of a singly linked list where elements are sorted in ascending order, convert it to a height-balanced binary search tree.
//!
//! Example 1:
//! Input: head = [-10,-3,0,5,9]
//! Output: [0,-3,9,-10,null,5]
//! Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
//!
//! Example 2:
//! Input: head = []
//! Output: []
//!
//! Constraints:
//! The number of nodes in head is in the range [0, 2 * 10^4].
//! -10^5 <= Node.val <= 10^5

use std::{cell::RefCell, rc::Rc};

use crate::utils::{listnode::ListNode, treenode::TreeNode};

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Node {
    match head {
        None => None,
        Some(_) => sorted_list_to_bst_helper(head.as_ref(), None),
    }
}

#[allow(clippy::borrowed_box)]
fn sorted_list_to_bst_helper(
    head: Option<&Box<ListNode>>,
    tail: Option<&Box<ListNode>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if head == tail {
        return None;
    }

    let mut slow = head;
    let mut fast = head;

    while fast != tail {
        let fast_next = fast.and_then(|node| node.next.as_ref());
        if fast_next == tail {
            break;
        }

        slow = slow.and_then(|node| node.next.as_ref());
        fast = fast_next.and_then(|node| node.next.as_ref());
    }

    Some(Rc::new(RefCell::new(TreeNode {
        val: slow.unwrap().val,
        left: sorted_list_to_bst_helper(head, slow),
        right: sorted_list_to_bst_helper(slow.unwrap().next.as_ref(), tail),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_list_to_bst_1() {
        let correct = TreeNode::from_arr(&[Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]);
        let head = ListNode::from_arr(&[-10, -3, 0, 5, 9]);

        assert_eq!(sorted_list_to_bst(head), correct);
    }

    #[test]
    fn test_sorted_list_to_bst_2() {
        let correct = TreeNode::from_arr(&[Some(3), Some(1)]);
        let head = ListNode::from_arr(&[1, 3]);

        assert_eq!(sorted_list_to_bst(head), correct);
    }

    #[test]
    fn test_sorted_list_to_bst_3() {
        let correct = TreeNode::from_arr(&[]);
        let head = ListNode::from_arr(&[]);

        assert_eq!(sorted_list_to_bst(head), correct);
    }
}
