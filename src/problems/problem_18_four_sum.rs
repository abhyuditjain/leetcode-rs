//! 18. 4Sum
//!
//! Medium
//!
//! Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
//! 0 <= a, b, c, d < n
//! a, b, c, and d are distinct.
//! nums[a] + nums[b] + nums[c] + nums[d] == target
//!
//! You may return the answer in any order.
//!
//! Example 1:
//! Input: nums = [1,0,-1,0,-2,2], target = 0
//! Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
//!
//! Example 2:
//! Input: nums = [2,2,2,2,2], target = 8
//! Output: [[2,2,2,2]]
//!
//! Constraints:
//! 1 <= nums.length <= 200
//! -109 <= nums[i] <= 109
//! -109 <= target <= 109

use std::cmp::Ordering;

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    k_sum(nums, target, 4)
}

pub fn k_sum(mut nums: Vec<i32>, target: i32, k: usize) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    k_sum_helper(&mut nums, target, k, 0)
}

fn k_sum_helper(nums: &mut [i32], target: i32, k: usize, start: usize) -> Vec<Vec<i32>> {
    if start >= nums.len() {
        return vec![];
    }

    let average = target / k as i32;
    if nums[start] > average || *nums.last().unwrap() < average {
        return vec![];
    }

    if k == 2 {
        return two_sum(nums, target, start);
    }

    let mut result = vec![];

    for i in start..nums.len() {
        if i == start || nums[i] != nums[i - 1] {
            for subset in k_sum_helper(
                nums,
                target.checked_sub(nums[i]).unwrap_or(i32::MAX),
                k - 1,
                i + 1,
            ) {
                let mut res = vec![nums[i]];
                res.extend(&subset);
                result.push(res);
            }
        }
    }

    result
}

fn two_sum(nums: &mut [i32], target: i32, start: usize) -> Vec<Vec<i32>> {
    let mut pairs = vec![];

    let mut left = start;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];

        match sum.cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => {
                pairs.push(vec![nums[left], nums[right]]);
                left += 1;
                right -= 1;

                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
            }
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum_1() {
        assert_eq!(
            four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn test_four_sum_2() {
        assert_eq!(four_sum(vec![2, 2, 2, 2, 2], 8), vec![vec![2, 2, 2, 2]]);
    }

    #[test]
    fn test_four_sum_3() {
        assert_eq!(
            four_sum(
                vec![0, 0, 0, 1000000000, 1000000000, 1000000000, 1000000000],
                1000000000
            ),
            vec![vec![0, 0, 0, 1000000000]]
        );
    }

    #[test]
    fn test_four_sum_4() {
        assert_eq!(
            four_sum(
                vec![1000000000, 1000000000, 1000000000, 1000000000],
                -294967296
            ),
            vec![vec![]; 0]
        );
    }

    #[test]
    fn test_four_sum_5() {
        assert_eq!(
            four_sum(
                vec![
                    -1000000000,
                    -1000000000,
                    1000000000,
                    -1000000000,
                    -1000000000
                ],
                294967296
            ),
            vec![vec![]; 0]
        );
    }
}
