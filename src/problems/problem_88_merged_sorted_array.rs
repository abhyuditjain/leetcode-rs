//! 88. Merge Sorted Array
//!
//! Easy
//!
//! You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n,
//! representing the number of elements in nums1 and nums2 respectively.
//! Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//! The final sorted array should not be returned by the function, but instead be stored inside the array nums1.
//! To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged,
//! and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
//!
//! Example 1:
//! Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
//! Output: [1,2,2,3,5,6]
//! Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
//! The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
//!
//! Example 2:
//! Input: nums1 = [1], m = 1, nums2 = [], n = 0
//! Output: [1]
//! Explanation: The arrays we are merging are [1] and [].
//! The result of the merge is [1].
//!
//! Example 3:
//! Input: nums1 = [0], m = 0, nums2 = [1], n = 1
//! Output: [1]
//! Explanation: The arrays we are merging are [] and [1].
//! The result of the merge is [1].
//! Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.
//!
//! Constraints:
//! nums1.length == m + n
//! nums2.length == n
//! 0 <= m, n <= 200
//! 1 <= m + n <= 200
//! -10^9 <= nums1[i], nums2[j] <= 10^9
//!
//! Follow up: Can you come up with an algorithm that runs in O(m + n) time?

use std::cmp::Ordering;

pub fn merge_arrays(nums1: &mut [i32], m: usize, nums2: &mut [i32], n: usize) {
    let mut i_it = (0..m).rev().peekable();
    let mut j_it = (0..n).rev().peekable();
    let mut k_it = (0..m + n).rev().peekable();

    while let (Some(&i), Some(&j), Some(&k)) = (i_it.peek(), j_it.peek(), k_it.peek()) {
        nums1[k] = match nums1[i].cmp(&nums2[j]) {
            Ordering::Greater => {
                i_it.next();
                nums1[i]
            }
            _ => {
                j_it.next();
                nums2[j]
            }
        };

        k_it.next();
    }

    while let (Some(j), Some(k)) = (j_it.next(), k_it.next()) {
        nums1[k] = nums2[j];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_arrays_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        merge_arrays(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, &[1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_merge_arrays_2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];

        merge_arrays(&mut nums1, 1, &mut nums2, 0);

        assert_eq!(nums1, &[1]);
    }

    #[test]
    fn test_merge_arrays_3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];

        merge_arrays(&mut nums1, 0, &mut nums2, 1);

        assert_eq!(nums1, &[1]);
    }
}
