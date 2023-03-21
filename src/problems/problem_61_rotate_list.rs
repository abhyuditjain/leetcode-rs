//! 61. Rotate List
//!
//! Medium
//!
//! Given the head of a linked list, rotate the list to the right by k places.
//!
//! Example 1:
//! Input: head = [1,2,3,4,5], k = 2
//! Output: [4,5,1,2,3]
//!
//! Example 2:
//! Input: head = [0,1,2], k = 4
//! Output: [2,0,1]
//!
//! Constraints:
//! The number of nodes in the list is in the range [0, 500].
//! -100 <= Node.val <= 100
//! 0 <= k <= 2 * 10^9

use crate::utils::listnode::ListNode;

pub fn rotate_list_right(head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    let len = len(&head);

    if len < 2 {
        return head;
    }

    let k = len - k % len;

    if k == len {
        return head;
    }

    let mut head = head;
    let mut curr = head.as_mut();

    for _ in 0..k - 1 {
        curr = curr.unwrap().next.as_mut();
    }

    let mut new_head = curr.unwrap().next.take();
    let mut curr = new_head.as_mut();

    while let Some(node) = curr {
        if node.next.is_none() {
            node.next = head;
            break;
        }
        curr = node.next.as_mut();
    }

    new_head
}

fn len(head: &Option<Box<ListNode>>) -> usize {
    let mut len = 0;

    let mut curr = head.as_ref();

    while let Some(node) = curr {
        curr = node.next.as_ref();
        len += 1;
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_list_right_1() {
        let head = ListNode::from_arr(&[1, 2, 3, 4, 5]);
        let correct = ListNode::from_arr(&[4, 5, 1, 2, 3]);

        assert_eq!(rotate_list_right(head, 2), correct);
    }

    #[test]
    fn test_rotate_list_right_2() {
        let head = ListNode::from_arr(&[0, 1, 2]);
        let correct = ListNode::from_arr(&[2, 0, 1]);

        assert_eq!(rotate_list_right(head, 4), correct);
    }

    #[test]
    fn test_rotate_list_right_3() {
        let head = ListNode::from_arr(&[]);
        let correct = ListNode::from_arr(&[]);

        assert_eq!(rotate_list_right(head, 4), correct);
    }

    #[test]
    fn test_rotate_list_right_4() {
        let head = ListNode::from_arr(&[1, 2, 3, 4]);
        let correct = ListNode::from_arr(&[1, 2, 3, 4]);

        assert_eq!(rotate_list_right(head, 4), correct);
    }

    #[test]
    fn test_len() {
        let list = ListNode::from_arr(&[1, 2, 3]);
        assert_eq!(len(&list), 3);

        let list = ListNode::from_arr(&[]);
        assert_eq!(len(&list), 0);
    }
}
