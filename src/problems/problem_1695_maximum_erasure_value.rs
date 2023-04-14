//! 1695. Maximum Erasure Value
//!
//! Medium
//!
//! You are given an array of positive integers nums and want to erase a subarray containing unique elements. The score you get by erasing the subarray is equal to the sum of its elements.
//! Return the maximum score you can get by erasing exactly one subarray.
//! An array b is called to be a subarray of a if it forms a contiguous subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r] for some (l,r).
//!
//! Example 1:
//! Input: nums = [4,2,4,5,6]
//! Output: 17
//! Explanation: The optimal subarray here is [2,4,5,6].
//!
//! Example 2:
//! Input: nums = [5,2,1,2,5,2,1,2,5]
//! Output: 8
//! Explanation: The optimal subarray here is [5,2,1] or [1,2,5].
//!
//! Constraints:
//! 1 <= nums.length <= 10^5
//! 1 <= nums[i] <= 10^4

use std::collections::HashMap;

pub fn maximum_unique_subarray(nums: &[i32]) -> i32 {
    let mut max_sum = 0;

    let mut window_count = HashMap::new();
    let mut window_sum = 0;
    let mut left = 0;

    for right in 0..nums.len() {
        let num_right = nums[right];
        *window_count.entry(num_right).or_insert(0) += 1;
        window_sum += num_right;

        while window_count.get(&num_right).unwrap() > &1 {
            let num_left = nums[left];
            window_sum -= num_left;
            window_count.entry(num_left).and_modify(|x| *x -= 1);

            left += 1;
        }

        max_sum = max_sum.max(window_sum);
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_unique_subarray() {
        assert_eq!(maximum_unique_subarray(&[4, 2, 4, 5, 6]), 17);
        assert_eq!(maximum_unique_subarray(&[5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
    }
}
