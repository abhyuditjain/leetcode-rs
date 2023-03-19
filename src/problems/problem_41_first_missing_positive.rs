//! 41. First Missing Positive
//!
//! Hard
//!
//! Given an unsorted integer array nums, return the smallest missing positive integer.
//! You must implement an algorithm that runs in O(n) time and uses constant extra space.
//!
//! Example 1:
//! Input: nums = [1,2,0]
//! Output: 3
//! Explanation: The numbers in the range [1,2] are all in the array.
//!
//! Example 2:
//! Input: nums = [3,4,-1,1]
//! Output: 2
//! Explanation: 1 is in the array but 2 is missing.
//!
//! Example 3:
//! Input: nums = [7,8,9,11,12]
//! Output: 1
//! Explanation: The smallest positive integer 1 is missing.
//!
//! Constraints:
//! 1 <= nums.length <= 105
//! -231 <= nums[i] <= 231 - 1

const INF: i32 = i32::max_value();

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        if !(1 <= nums[i] && nums[i] <= nums.len() as i32) {
            nums[i] = INF;
        }
    }

    for i in 0..nums.len() {
        let index = nums[i].abs() - 1;
        if index != INF - 1 {
            nums[index as usize] = -nums[index as usize].abs();
        }
    }

    for (i, num) in nums.iter().enumerate() {
        if num.is_positive() {
            return i as i32 + 1;
        }
    }

    nums.len() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive_1() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn test_first_missing_positive_2() {
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn test_first_missing_positive_3() {
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
