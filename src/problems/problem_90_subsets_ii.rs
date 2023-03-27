//! 90. Subsets II
//!
//! Medium
//!
//! Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
//! The solution set must not contain duplicate subsets. Return the solution in any order.
//!
//! Example 1:
//! Input: nums = [1,2,2]
//! Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
//!
//! Example 2:
//! Input: nums = [0]
//! Output: [[],[0]]
//!
//! Constraints:
//! 1 <= nums.length <= 10
//! -10 <= nums[i] <= 10

pub fn subsets(nums: &mut [i32]) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut subsets = vec![];
    let mut subset = Vec::with_capacity(nums.len());

    backtrack(nums, &mut subsets, &mut subset, 0);

    subsets
}

fn backtrack(nums: &[i32], subsets: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, idx: usize) {
    subsets.push(subset.clone());

    for i in idx..nums.len() {
        if i > idx && nums[i] == nums[i - 1] {
            continue;
        }

        subset.push(nums[i]);
        backtrack(nums, subsets, subset, i + 1);
        subset.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_1() {
        assert_eq!(
            subsets(&mut [1, 2, 2]),
            [
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        )
    }

    #[test]
    fn test_subsets_2() {
        assert_eq!(subsets(&mut [0]), [vec![], vec![0],])
    }
}
