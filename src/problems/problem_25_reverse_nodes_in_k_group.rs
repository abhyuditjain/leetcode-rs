//! 25. Reverse Nodes in k-Group
//!
//! Hard
//!
//! Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
//! k is a positive integer and is less than or equal to the length of the linked list.
//! If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
//!
//! You may not alter the values in the list's nodes, only nodes themselves may be changed.
//!
//! Example 1:
//! Input: head = [1,2,3,4,5], k = 2
//! Output: [2,1,4,3,5]
//!
//! Example 2:
//! Input: head = [1,2,3,4,5], k = 3
//! Output: [3,2,1,4,5]
//!
//! Constraints:
//! The number of nodes in the list is n.
//! 1 <= k <= n <= 5000
//! 0 <= Node.val <= 1000
//!
//! Follow-up: Can you solve the problem in O(1) extra memory space?

use crate::utils::listnode::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k == 0 {
        return head;
    }
    // 3 components:
    //  - head: remaining list
    //  - k_group: current accumulating group, targeting k elements
    //  - prev_tail: tail of already processed part of list
    //
    // Basic idea:
    // Take nodes from head, prepend the node to k_group, so after doing it k times,
    // k_group will be the reversed list of size k taken from head.
    // Then we append k_group to prev_tail, and search for the new prev_tail.
    // If there is not enough nodes to form a k_group of size k, that means the current k_group is the last group
    // and its size is smaller than k. In this case, we reverse the k_group again to revert the change, and append
    // it to prev_tail, then return.
    let mut head = head;
    let mut p_head: Option<Box<ListNode>> = None;
    let mut prev_tail = &mut p_head;
    let mut k_group: Option<Box<ListNode>> = None;
    loop {
        for k_group_len in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                node.next = k_group;
                k_group = Some(node);
            } else {
                let mut reverted_k_group: Option<Box<ListNode>> = None;
                for _ in 0..k_group_len {
                    let mut node = k_group.unwrap();
                    k_group = node.next.take();
                    node.next = reverted_k_group;
                    reverted_k_group = Some(node);
                }
                *prev_tail = reverted_k_group;
                return p_head;
            }
        }
        *prev_tail = k_group;
        for _ in 0..k {
            prev_tail = &mut prev_tail.as_mut().unwrap().next;
        }
        k_group = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_k_group_1() {
        let list = ListNode::from_arr(&[1, 2, 3, 4, 5]);
        let correct = ListNode::from_arr(&[2, 1, 4, 3, 5]);

        assert_eq!(reverse_k_group(list, 2), correct);
    }

    #[test]
    fn test_reverse_k_group_2() {
        let list = ListNode::from_arr(&[1, 2, 3, 4, 5]);
        let correct = ListNode::from_arr(&[3, 2, 1, 4, 5]);

        assert_eq!(reverse_k_group(list, 3), correct);
    }

    #[test]
    fn test_reverse_k_group_3() {
        let list = ListNode::from_arr(&[1, 2, 3, 4, 5]);
        let correct = ListNode::from_arr(&[1, 2, 3, 4, 5]);

        assert_eq!(reverse_k_group(list, 1), correct);
    }

    #[test]
    fn test_reverse_k_group_4() {
        let list = ListNode::from_arr(&[1, 2, 3, 4, 5]);
        let correct = ListNode::from_arr(&[5, 4, 3, 2, 1]);

        assert_eq!(reverse_k_group(list, 5), correct);
    }
}
