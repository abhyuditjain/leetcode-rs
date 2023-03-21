//! 53. Maximum Subarray
//!
//! Medium
//!
//! Given an integer array nums, find the subarray with the largest sum, and return its sum.
//!
//! Example 1:
//! Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
//! Output: 6
//! Explanation: The subarray [4,-1,2,1] has the largest sum 6.
//!
//! Example 2:
//! Input: nums = [1]
//! Output: 1
//! Explanation: The subarray [1] has the largest sum 1.
//!
//! Example 3:
//! Input: nums = [5,4,-1,7,8]
//! Output: 23
//! Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
//!
//! Constraints:
//! 1 <= nums.length <= 10^5
//! -10^4 <= nums[i] <= 10^4
//!
//! Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

pub fn max_subarray(nums: &[i32]) -> i32 {
    kadane_max(nums)
}

pub fn kadane_max(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut curr_sum = nums[0];

    for &num in nums.iter().skip(1) {
        curr_sum = num.max(curr_sum + num);
        max_sum = max_sum.max(curr_sum);
    }

    max_sum
}

pub fn max_subarray_divide_and_conquer(nums: &[i32]) -> i32 {
    max_subarray_divide_and_conquer_helper(nums, 0_i32, nums.len() as i32 - 1)
}

fn max_subarray_divide_and_conquer_helper(nums: &[i32], start: i32, end: i32) -> i32 {
    if start > end {
        return i32::MIN;
    }

    let mid = start + (end - start) / 2;

    let mut left_best_sum = 0;
    let mut right_best_sum = 0;

    // [start, mid-1]
    let mut curr_sum = 0;
    for i in (start..mid).rev() {
        curr_sum += nums[i as usize];
        left_best_sum = left_best_sum.max(curr_sum);
    }

    // [mid+1, end]
    let mut curr_sum = 0;
    for i in (mid + 1)..=end {
        curr_sum += nums[i as usize];
        right_best_sum = right_best_sum.max(curr_sum);
    }

    let best_total_sum = left_best_sum + nums[mid as usize] + right_best_sum;

    let left_half_max_sum = max_subarray_divide_and_conquer_helper(nums, start, mid - 1);
    let right_half_max_sum = max_subarray_divide_and_conquer_helper(nums, mid + 1, end);

    best_total_sum.max(left_half_max_sum.max(right_half_max_sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kadane_max_1() {
        assert_eq!(kadane_max(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn test_kadane_max_2() {
        assert_eq!(kadane_max(&[1]), 1);
    }

    #[test]
    fn test_kadane_max_3() {
        assert_eq!(kadane_max(&[5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn test_max_subarray_divide_and_conquer_1() {
        assert_eq!(
            max_subarray_divide_and_conquer(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_max_subarray_divide_and_conquer_2() {
        assert_eq!(max_subarray_divide_and_conquer(&[1]), 1);
    }

    #[test]
    fn test_max_subarray_divide_and_conquer_3() {
        assert_eq!(max_subarray_divide_and_conquer(&[5, 4, -1, 7, 8]), 23);
    }
}
