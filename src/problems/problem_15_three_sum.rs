//! 15. 3Sum
//!
//! Medium
//!
//! Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//! Notice that the solution set must not contain duplicate triplets.
//!
//! Example 1:
//! Input: nums = [-1,0,1,2,-1,-4]
//! Output: [[-1,-1,2],[-1,0,1]]
//! Explanation:
//! nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
//! nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
//! nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
//! The distinct triplets are [-1,0,1] and [-1,-1,2].
//! Notice that the order of the output and the order of the triplets does not matter.
//!
//! Example 2:
//! Input: nums = [0,1,1]
//! Output: []
//! Explanation: The only possible triplet does not sum up to 0.
//!
//! Example 3:
//! Input: nums = [0,0,0]
//! Output: [[0,0,0]]
//! Explanation: The only possible triplet sums up to 0.
//!
//! Constraints:
//! 3 <= nums.length <= 3000
//! -105 <= nums[i] <= 105

use std::cmp::Ordering;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut triplets = vec![];

    for i in 0..nums.len() - 2 {
        if i > 0 && nums[i - 1] == nums[i] {
            continue;
        }

        let mut pairs = two_sum(&nums, -nums[i], i + 1);

        for pair in pairs.iter_mut() {
            pair.push(nums[i]);
        }

        triplets.append(&mut pairs);
    }

    triplets
}

fn two_sum(nums: &[i32], target: i32, start: usize) -> Vec<Vec<i32>> {
    let mut left = start;
    let mut right = nums.len() - 1;

    let mut pairs = vec![];

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
    fn test_three_sum() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, 2, -1], vec![0, 1, -1]]
        );

        let correct: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(vec![0, 1, 1]), correct);
    }
}
