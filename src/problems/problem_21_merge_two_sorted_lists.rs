//! 21. Merge Two Sorted Lists
//!
//! Easy
//!
//! You are given the heads of two sorted linked lists list1 and list2.
//! Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
//! Return the head of the merged linked list.
//!
//! Example 1:
//! Input: list1 = [1,2,4], list2 = [1,3,4]
//! Output: [1,1,2,3,4,4]
//!
//! Example 2:
//! Input: list1 = [], list2 = []
//! Output: []
//!
//! Example 3:
//! Input: list1 = [], list2 = [0]
//! Output: [0]
//!
//! Constraints:
//! The number of nodes in both lists is in the range [0, 50].
//! -100 <= Node.val <= 100
//!
//! Both list1 and list2 are sorted in non-decreasing order.

use crate::utils::listnode::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    }

    if list2.is_none() {
        return list1;
    }

    let mut dummy = Box::new(ListNode::new(-1));
    let mut tail = &mut dummy;

    let mut curr1 = list1;
    let mut curr2 = list2;

    while curr1.is_some() && curr2.is_some() {
        if curr1.as_ref().unwrap().val < curr2.as_ref().unwrap().val {
            tail.next = curr1.take();
            tail = tail.next.as_mut().unwrap();
            curr1 = tail.next.take();
        } else {
            tail.next = curr2.take();
            tail = tail.next.as_mut().unwrap();
            curr2 = tail.next.take();
        }
    }

    if curr1.is_some() {
        tail.next = curr1.take();
    }

    if curr2.is_some() {
        tail.next = curr2.take();
    }

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists_1() {
        let list1 = ListNode::from_arr(&[1, 2, 4]);
        let list2 = ListNode::from_arr(&[1, 3, 4]);
        let merged = ListNode::from_arr(&[1, 1, 2, 3, 4, 4]);

        assert_eq!(merge_two_lists(list1, list2), merged);
    }

    #[test]
    fn test_merge_two_lists_2() {
        let list1 = ListNode::from_arr(&[]);
        let list2 = ListNode::from_arr(&[]);
        let merged = ListNode::from_arr(&[]);

        assert_eq!(merge_two_lists(list1, list2), merged);
    }

    #[test]
    fn test_merge_two_lists_3() {
        let list1 = ListNode::from_arr(&[]);
        let list2 = ListNode::from_arr(&[0]);
        let merged = ListNode::from_arr(&[0]);

        assert_eq!(merge_two_lists(list1, list2), merged);
    }
}
