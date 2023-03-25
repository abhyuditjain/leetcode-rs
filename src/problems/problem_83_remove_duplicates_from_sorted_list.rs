//! 83. Remove Duplicates from Sorted List
//!
//! Easy
//!
//! Given the head of a sorted linked list, delete all duplicates such that each element appears only once.
//! Return the linked list sorted as well.
//!
//! Example 1:
//! Input: head = [1,1,2]
//! Output: [1,2]
//!
//! Example 2:
//! Input: head = [1,1,2,3,3]
//! Output: [1,2,3]
//!
//! Constraints:
//! The number of nodes in the list is in the range [0, 300].
//! -100 <= Node.val <= 100
//! The list is guaranteed to be sorted in ascending order.

use crate::utils::listnode::ListNode;

pub fn remove_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr_option = head.as_mut();

    while let Some(curr) = curr_option {
        let mut next_option = curr.next.take();

        while let Some(next) = next_option.as_mut() {
            if next.val == curr.val {
                next_option = next.next.take();
            } else {
                curr.next = next_option;
                break;
            }
        }
        curr_option = curr.next.as_mut();
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_1() {
        let head = ListNode::from_arr(&[1, 2, 3, 3, 4, 4, 5]);
        let correct = ListNode::from_arr(&[1, 2, 3, 4, 5]);

        assert_eq!(remove_duplicates(head), correct);
    }

    #[test]
    fn test_remove_duplicates_2() {
        let head = ListNode::from_arr(&[1, 1, 1, 2, 3]);
        let correct = ListNode::from_arr(&[1, 2, 3]);

        assert_eq!(remove_duplicates(head), correct);
    }

    #[test]
    fn test_remove_duplicates_3() {
        let head = ListNode::from_arr(&[1, 1, 1]);
        let correct = ListNode::from_arr(&[1]);

        assert_eq!(remove_duplicates(head), correct);
    }

    #[test]
    fn test_remove_duplicates_4() {
        let head = ListNode::from_arr(&[]);
        let correct = ListNode::from_arr(&[]);

        assert_eq!(remove_duplicates(head), correct);
    }

    #[test]
    fn test_remove_duplicates_5() {
        let head = ListNode::from_arr(&[1, 1, 2]);
        let correct = ListNode::from_arr(&[1, 2]);

        assert_eq!(remove_duplicates(head), correct);
    }

    #[test]
    fn test_remove_duplicates_6() {
        let head = ListNode::from_arr(&[1, 1, 2, 3, 3]);
        let correct = ListNode::from_arr(&[1, 2, 3]);

        assert_eq!(remove_duplicates(head), correct);
    }
}
