//! 46. Permutations
//!
//! Medium
//!
//! Given an array nums of distinct integers, return all the possible permutations.
//! You can return the answer in any order.
//!
//! Example 1:
//! Input: nums = [1,2,3]
//! Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//!
//! Example 2:
//! Input: nums = [0,1]
//! Output: [[0,1],[1,0]]
//!
//! Example 3:
//! Input: nums = [1]
//! Output: [[1]]
//!
//! Constraints:
//! 1 <= nums.length <= 6
//! -10 <= nums[i] <= 10
//! All the integers of nums are unique.

pub fn permutations(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut permutations = vec![];
    backtrack(&mut nums, &mut permutations, 0);
    permutations
}

fn backtrack(nums: &mut Vec<i32>, permutations: &mut Vec<Vec<i32>>, start: usize) {
    if start == nums.len() {
        permutations.push(nums.clone());
        return;
    }

    for i in start..nums.len() {
        nums.swap(start, i);
        backtrack(nums, permutations, start + 1);
        nums.swap(start, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_1() {
        assert_eq!(
            permutations(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1],
                vec![3, 1, 2],
            ]
        );
    }

    #[test]
    fn test_permutations_2() {
        assert_eq!(permutations(vec![0, 1]), vec![vec![0, 1], vec![1, 0],]);
    }
}
