//! 19. Remove Nth Node From End of List
//!
//! Medium
//!
//! Given the head of a linked list, remove the nth node from the end of the list and return its head.
//!
//! Example 1:
//! Input: head = [1,2,3,4,5], n = 2
//! Output: [1,2,3,5]
//!
//! Example 2:
//! Input: head = [1], n = 1
//! Output: []
//!
//! Example 3:
//! Input: head = [1,2], n = 1
//! Output: [1]
//!
//! Constraints:
//! The number of nodes in the list is sz.
//! 1 <= sz <= 30
//! 0 <= Node.val <= 100
//! 1 <= n <= sz
//!
//! Follow up: Could you do this in one pass?

use crate::utils::listnode::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    dummy.next = head;

    let mut dummy = Box::new(dummy);

    let mut fast = dummy.clone();
    let mut slow = dummy.as_mut();

    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    while fast.next.is_some() {
        slow = slow.next.as_mut().unwrap();
        fast = fast.next.unwrap();
    }

    let next = slow.next.as_mut().unwrap();

    slow.next = next.next.clone();

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end_1() {
        let list = ListNode::from_arr(&[1, 2, 3, 4, 5]);
        let answer = ListNode::from_arr(&[1, 2, 3, 5]);

        assert_eq!(remove_nth_from_end(list, 2), answer);
    }

    #[test]
    fn test_remove_nth_from_end_2() {
        let list = ListNode::from_arr(&[1]);
        let answer = ListNode::from_arr(&[]);

        assert_eq!(remove_nth_from_end(list, 1), answer);
    }

    #[test]
    fn test_remove_nth_from_end_3() {
        let list = ListNode::from_arr(&[1, 2]);
        let answer = ListNode::from_arr(&[1]);

        assert_eq!(remove_nth_from_end(list, 1), answer);
    }
}
