//! 23. Merge k Sorted Lists
//!
//! Hard
//!
//! You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
//!
//! Merge all the linked-lists into one sorted linked-list and return it.
//!
//! Example 1:
//! Input: lists = [[1,4,5],[1,3,4],[2,6]]
//! Output: [1,1,2,3,4,4,5,6]
//! Explanation: The linked-lists are:
//! [
//!   1->4->5,
//!   1->3->4,
//!   2->6
//! ]
//! merging them into one sorted list:
//! 1->1->2->3->4->4->5->6
//!
//! Example 2:
//! Input: lists = []
//! Output: []
//!
//! Example 3:
//! Input: lists = [[]]
//! Output: []
//!
//! Constraints:
//! k == lists.length
//! 0 <= k <= 104
//! 0 <= lists[i].length <= 500
//! -104 <= lists[i][j] <= 104
//! lists[i] is sorted in ascending order.
//! The sum of lists[i].length will not exceed 104.

use std::collections::BinaryHeap;

use crate::utils::listnode::ListNode;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

pub fn merge_k_lists(lists: &[Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(-1));

    let mut min_heap = BinaryHeap::new();

    for list in lists {
        if list.is_some() {
            min_heap.push(list.clone().take().unwrap());
        }
    }

    let mut tail = &mut head;

    while !min_heap.is_empty() {
        let node = min_heap.pop().unwrap();
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();

        if tail.next.is_some() {
            min_heap.push(tail.next.take().unwrap());
        }
    }

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_k_lists() {
        let data = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let answer = ListNode::from_arr(&[1, 1, 2, 3, 4, 4, 5, 6]);
        let lists = data
            .into_iter()
            .map(|list| ListNode::from_arr(&list))
            .collect::<Vec<_>>();

        assert_eq!(merge_k_lists(&lists), answer);
    }

    #[test]
    fn test_merge_k_lists_empty() {
        let answer = None;
        let lists = &[];

        assert_eq!(merge_k_lists(lists), answer);
    }

    #[test]
    fn test_merge_k_lists_list_of_empty_list() {
        let answer = None;
        let lists = &[None];

        assert_eq!(merge_k_lists(lists), answer);
    }
}
