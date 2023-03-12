//! 2. Add Two Numbers
//!
//! Medium
//!
//! You are given two non-empty linked lists representing two non-negative integers.
//! The digits are stored in reverse order, and each of their nodes contains a single digit.
//! Add the two numbers and return the sum as a linked list.
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! Example 1:
//! Input: l1 = [2,4,3], l2 = [5,6,4]
//! Output: [7,0,8]
//! Explanation: 342 + 465 = 807.
//!
//! Example 2:
//! Input: l1 = [0], l2 = [0]
//! Output: [0]
//!
//! Example 3:
//! Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//! Output: [8,9,9,9,0,0,0,1]
//!
//! Constraints:
//! The number of nodes in each linked list is in the range [1, 100].
//! 0 <= Node.val <= 9
//!
//! It is guaranteed that the list represents a number that does not have leading zeros.

#[derive(Debug, PartialEq)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(-1));
    let mut tail = &mut head;

    let mut l1 = l1;
    let mut l2 = l2;

    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut v1 = 0;
        let mut v2 = 0;
        if let Some(node) = l1 {
            v1 = node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            v2 = node.val;
            l2 = node.next;
        }

        tail.next = Some(Box::new(ListNode::new((v1 + v2 + carry) % 10)));
        tail = tail.next.as_mut().unwrap();
        carry = (v1 + v2 + carry) / 10;
    }

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(v: &[i32]) -> Option<Box<ListNode>> {
        let mut front = Box::new(ListNode::new(0));
        let mut current = &mut front;

        for x in v {
            current.next = Some(Box::new(ListNode::new(*x)));
            current = current.next.as_mut().unwrap();
        }

        front.next
    }

    #[test]
    fn test1() {
        assert_eq!(
            add_two_numbers(to_list(&[2, 4, 3]), to_list(&[5, 6, 4])),
            to_list(&[7, 0, 8])
        );
    }
    #[test]
    fn test2() {
        assert_eq!(add_two_numbers(to_list(&[0]), to_list(&[0])), to_list(&[0]));
    }
    #[test]
    fn test3() {
        assert_eq!(
            add_two_numbers(to_list(&[9, 9, 9, 9, 9, 9, 9]), to_list(&[9, 9, 9, 9])),
            to_list(&[8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
