//! 16. 3Sum Closest
//!
//! Medium
//!
//! Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
//! Return the sum of the three integers.
//!
//! You may assume that each input would have exactly one solution.
//!
//! Example 1:
//! Input: nums = [-1,2,1,-4], target = 1
//! Output: 2
//! Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
//!
//! Example 2:
//! Input: nums = [0,0,0], target = 1
//! Output: 0
//! Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
//!
//! Constraints:
//! 3 <= nums.length <= 500
//! -1000 <= nums[i] <= 1000
//! -104 <= target <= 104

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut closest_sum = i32::MAX;
    let mut min_diff = i32::MAX;

    for i in 0..nums.len() - 2 {
        let (mut left, mut right) = (i + 1, nums.len() - 1);

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            let diff = (target - sum).abs();

            if diff < min_diff {
                closest_sum = sum;
                min_diff = diff;
            }

            if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    closest_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
