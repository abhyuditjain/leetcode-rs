//! 82. Remove Duplicates from Sorted List II
//!
//! Medium
//!
//! Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list.
//! Return the linked list sorted as well.
//!
//! Example 1:
//! Input: head = [1,2,3,3,4,4,5]
//! Output: [1,2,5]
//!
//! Example 2:
//! Input: head = [1,1,1,2,3]
//! Output: [2,3]
//!
//! Constraints:
//! The number of nodes in the list is in the range [0, 300].
//! -100 <= Node.val <= 100
//! The list is guaranteed to be sorted in ascending order.

use crate::utils::listnode::ListNode;

pub fn remove_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: head };

    let mut previous = &mut dummy;

    while let Some(mut node) = previous.next.as_mut() {
        if node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
            while node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
                node = node.next.as_mut().unwrap();
            }
            previous.next = node.next.take();
        } else {
            previous = previous.next.as_mut().unwrap();
        }
    }

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_1() {
        let head = ListNode::from_arr(&[1, 2, 3, 3, 4, 4, 5]);
        let correct = ListNode::from_arr(&[1, 2, 5]);

        assert_eq!(remove_duplicates(head), correct);
    }

    #[test]
    fn test_remove_duplicates_2() {
        let head = ListNode::from_arr(&[1, 1, 1, 2, 3]);
        let correct = ListNode::from_arr(&[2, 3]);

        assert_eq!(remove_duplicates(head), correct);
    }

    #[test]
    fn test_remove_duplicates_3() {
        let head = ListNode::from_arr(&[1, 1, 1]);
        let correct = ListNode::from_arr(&[]);

        assert_eq!(remove_duplicates(head), correct);
    }

    #[test]
    fn test_remove_duplicates_4() {
        let head = ListNode::from_arr(&[]);
        let correct = ListNode::from_arr(&[]);

        assert_eq!(remove_duplicates(head), correct);
    }
}
