//! 55. Jump Game
//!
//! Medium
//!
//! You are given an integer array nums.
//! You are initially positioned at the array's first index,
//! and each element in the array represents your maximum jump length at that position.
//! Return true if you can reach the last index, or false otherwise.
//!
//! Example 1:
//! Input: nums = [2,3,1,1,4]
//! Output: true
//! Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
//!
//! Example 2:
//! Input: nums = [3,2,1,0,4]
//! Output: false
//! Explanation: You will always arrive at index 3 no matter what.
//! Its maximum jump length is 0, which makes it impossible to reach the last index.
//!
//! Constraints:
//! 1 <= nums.length <= 10^4
//! 0 <= nums[i] <= 10^5

pub fn can_jump_to_end(nums: &[usize]) -> bool {
    let mut can_jump = vec![false; nums.len()];
    *can_jump.last_mut().unwrap() = true;

    (0..(nums.len() - 1)).rev().for_each(|start| {
        let end = (nums.len() - 1).min(start + nums[start]);

        ((start + 1)..=end).for_each(|i| {
            if can_jump[i] {
                can_jump[start] = true;
            }
        });
    });

    can_jump[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump_to_end_1() {
        assert!(can_jump_to_end(&[2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_can_jump_to_end_2() {
        assert!(!can_jump_to_end(&[3, 2, 1, 0, 4]));
    }
}
