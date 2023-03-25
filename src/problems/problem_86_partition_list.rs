//! 86. Partition List
//!
//! Medium
//!
//! Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
//! You should preserve the original relative order of the nodes in each of the two partitions.
//!
//! Example 1:
//! Input: head = [1,4,3,2,5,2], x = 3
//! Output: [1,2,2,4,3,5]
//!
//! Example 2:
//! Input: head = [2,1], x = 2
//! Output: [1,2]
//!
//! Constraints:
//! The number of nodes in the list is in the range [0, 200].
//! -100 <= Node.val <= 100
//! -200 <= x <= 200

use crate::utils::listnode::ListNode;

pub fn partition_list(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut smaller_dummy = ListNode::new(-1);
    let mut greater_dummy = ListNode::new(-1);

    let mut curr_smaller = &mut smaller_dummy;
    let mut curr_greater = &mut greater_dummy;

    while let Some(mut node) = head {
        head = node.next.take();

        if node.val < x {
            curr_smaller.next = Some(node);
            curr_smaller = curr_smaller.next.as_mut().unwrap();
        } else {
            curr_greater.next = Some(node);
            curr_greater = curr_greater.next.as_mut().unwrap();
        }
    }

    curr_smaller.next = greater_dummy.next;

    smaller_dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_list_1() {
        let head = ListNode::from_arr(&[1, 4, 3, 2, 5, 2]);
        let correct = ListNode::from_arr(&[1, 2, 2, 4, 3, 5]);

        assert_eq!(partition_list(head, 3), correct);
    }

    #[test]
    fn test_partition_list_2() {
        let head = ListNode::from_arr(&[2, 1]);
        let correct = ListNode::from_arr(&[1, 2]);

        assert_eq!(partition_list(head, 2), correct);
    }

    #[test]
    fn test_partition_list_3() {
        let head = ListNode::from_arr(&[]);
        let correct = ListNode::from_arr(&[]);

        assert_eq!(partition_list(head, 2), correct);
    }

    #[test]
    fn test_partition_list_4() {
        let head = ListNode::from_arr(&[1]);
        let correct = ListNode::from_arr(&[1]);

        assert_eq!(partition_list(head, 2), correct);
    }
}
