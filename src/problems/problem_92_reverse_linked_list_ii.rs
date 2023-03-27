//! 92. Reverse Linked List II
//!
//! Medium
//!
//! Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position
//! left to position right, and return the reversed list.
//!
//! Example 1:
//! Input: head = [1,2,3,4,5], left = 2, right = 4
//! Output: [1,4,3,2,5]
//!
//! Example 2:
//! Input: head = [5], left = 1, right = 1
//! Output: [5]
//!
//! Constraints:
//! The number of nodes in the list is n.
//! 1 <= n <= 500
//! -500 <= Node.val <= 500
//! 1 <= left <= right <= n
//!
//! Follow up: Could you do it in one pass?

use crate::utils::listnode::ListNode;

/// We keep 2 pointers => previous (p) and tail (t)
/// Example:
/// head = 1 -> 2 -> 3 -> 4 -> 5 -> null, left = 2, right = 4
/// This list is attached to dummy
/// dummy -> 1 -> 2 -> 3 -> 4 -> 5 -> null
///          p    t
/// Then we start reversing:
///     tmp = p.next
///     p.next = tail.next
///     tail.next = tail.next.next
///     p.next.next = temp
/// After one iteration
/// dummy -> 1 -> 3 -> 2 -> 4 -> 5 -> null

pub fn reverse_range(
    head: Option<Box<ListNode>>,
    left: usize,
    right: usize,
) -> Option<Box<ListNode>> {
    if head.is_none() || left == right {
        return head;
    }

    let mut dummy = Some(Box::new(ListNode {
        val: -1,
        next: head,
    }));

    let mut previous = &mut dummy;

    for _ in 1..left {
        previous = &mut previous.as_mut().unwrap().next;
    }

    let mut tail = previous.as_mut().unwrap().next.take();

    for _ in left..right {
        let mut temp = tail.as_mut().unwrap().next.take();
        tail.as_mut().unwrap().next = temp.as_mut().unwrap().next.take();
        temp.as_mut().unwrap().next = previous.as_mut().unwrap().next.take();
        previous.as_mut().unwrap().next = temp;
    }

    while previous.as_ref().unwrap().next.is_some() {
        previous = &mut previous.as_mut().unwrap().next;
    }

    previous.as_mut().unwrap().next = tail;

    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_range_1() {
        let head = ListNode::from_arr(&[1, 2, 3, 4, 5]);
        let correct = ListNode::from_arr(&[1, 4, 3, 2, 5]);

        assert_eq!(reverse_range(head, 2, 4), correct);
    }

    #[test]
    fn test_reverse_range_2() {
        let head = ListNode::from_arr(&[5]);
        let correct = ListNode::from_arr(&[5]);

        assert_eq!(reverse_range(head, 1, 1), correct);
    }
}
