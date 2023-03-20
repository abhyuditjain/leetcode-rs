//! 45. Jump Game II
//!
//! Medium
//!
//! You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].
//! Each element nums[i] represents the maximum length of a forward jump from index i.
//! In other words, if you are at nums[i], you can jump to any nums[i + j] where:
//! 0 <= j <= nums[i] and
//! i + j < n
//!
//! Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].
//!
//! Example 1:
//! Input: nums = [2,3,1,1,4]
//! Output: 2
//! Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
//!
//! Example 2:
//! Input: nums = [2,3,0,1,4]
//! Output: 2
//!
//! Constraints:
//! 1 <= nums.length <= 104
//! 0 <= nums[i] <= 1000
//! It's guaranteed that you can reach nums[n - 1].

pub fn min_jumps_to_reach_end(nums: &[usize]) -> usize {
    if nums.len() < 2 {
        return 0;
    }

    let mut jumps = vec![0; nums.len()];

    for start in (0..nums.len() - 1).rev() {
        let mut min = 10001;

        ((start + 1)..=(nums[start] + start)).for_each(|i| {
            let jump = if i >= nums.len() { 0 } else { jumps[i] };
            min = min.min(1 + jump);
        });

        jumps[start] = min;
    }
    jumps[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_jumps_to_reach_end_1() {
        assert_eq!(min_jumps_to_reach_end(&[2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_min_jumps_to_reach_end_2() {
        assert_eq!(min_jumps_to_reach_end(&[2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn test_min_jumps_to_reach_end_3() {
        assert_eq!(min_jumps_to_reach_end(&[2]), 0);
    }
}
