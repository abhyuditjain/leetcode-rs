//! 1. Two Sum
//!
//! Easy
//!
//! Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//! You can return the answer in any order.
//!
//! Example 1:
//! Input: nums = [2,7,11,15], target = 9
//! Output: [0,1]
//! Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//!
//! Example 2:
//! Input: nums = [3,2,4], target = 6
//! Output: [1,2]
//!
//! Example 3:
//! Input: nums = [3,3], target = 6
//! Output: [0,1]
//!
//! Constraints:
//! 2 <= nums.length <= 104
//! -109 <= nums[i] <= 109
//! -109 <= target <= 109
//!
//! Only one valid answer exists.
//!
//! Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    assert!(nums.len() > 1);

    let mut map = HashMap::new();

    for (idx, val) in nums.iter().enumerate() {
        if let Some(idx2) = map.get(val) {
            return (*idx2, idx);
        }
        map.insert(target - val, idx);
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), (0, 1));
        assert_eq!(two_sum(&[3, 2, 4], 6), (1, 2));
        assert_eq!(two_sum(&[3, 3], 6), (0, 1));
    }
}
