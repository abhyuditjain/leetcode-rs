//! 24. Swap Nodes in Pairs
//!
//! Medium
//!
//! Given a linked list, swap every two adjacent nodes and return its head.
//! You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
//!
//! Example 1:
//! Input: head = [1,2,3,4]
//! Output: [2,1,4,3]
//!
//! Example 2:
//! Input: head = []
//! Output: []
//!
//! Example 3:
//! Input: head = [1]
//! Output: [1]
//!
//! Constraints:
//! The number of nodes in the list is in the range [0, 100].
//! 0 <= Node.val <= 100

use crate::utils::listnode::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;
    let mut head = head;
    let mut cur_node = &mut head;

    // while we have something to work with (.next has something)
    while cur_node.is_some() && cur_node.as_ref().unwrap().next.is_some() {
        // .take() Takes the value out of the option, leaving a None in its place.
        let mut even_node = cur_node.as_mut().unwrap().next.take();

        // taking next odd node's value
        let next_odd_node = even_node.as_mut().unwrap().next.take();
        // cur_node's .next points to next odd node
        cur_node.as_mut().unwrap().next = next_odd_node;
        // even node's .next points to cur_node
        even_node.as_mut().unwrap().next = cur_node.take();
        // switching cur_node with even node
        cur_node.replace(even_node.unwrap());
        // point cursor to .next.next
        cur_node = &mut cur_node.as_mut().unwrap().next.as_mut().unwrap().next;
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pairs_1() {
        let list = ListNode::from_arr(&[1, 2, 3, 4]);
        let correct = ListNode::from_arr(&[2, 1, 4, 3]);

        assert_eq!(swap_pairs(list), correct);
    }

    #[test]
    fn test_swap_pairs_2() {
        let list = ListNode::from_arr(&[]);
        let correct = ListNode::from_arr(&[]);

        assert_eq!(swap_pairs(list), correct);
    }

    #[test]
    fn test_swap_pairs_3() {
        let list = ListNode::from_arr(&[1]);
        let correct = ListNode::from_arr(&[1]);

        assert_eq!(swap_pairs(list), correct);
    }
}
