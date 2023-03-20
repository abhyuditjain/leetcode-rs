//! 47. Permutations II
//!
//! Medium
//!
//! Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.
//!
//! Example 1:
//! Input: nums = [1,1,2]
//! Output:
//! [[1,1,2],
//!  [1,2,1],
//!  [2,1,1]]
//!
//! Example 2:
//! Input: nums = [1,2,3]
//! Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//!
//! Constraints:
//! 1 <= nums.length <= 8
//! -10 <= nums[i] <= 10

pub fn permutations(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut permutations = vec![];
    nums.sort_unstable();

    let mut used = vec![false; nums.len()];
    let mut permutation = vec![];
    backtrack(&nums, &mut permutations, &mut permutation, &mut used);
    permutations
}

fn backtrack(
    nums: &[i32],
    permutations: &mut Vec<Vec<i32>>,
    permutation: &mut Vec<i32>,
    used: &mut Vec<bool>,
) {
    if permutation.len() == nums.len() {
        permutations.push(permutation.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
            continue;
        }

        used[i] = true;
        permutation.push(nums[i]);
        backtrack(nums, permutations, permutation, used);
        used[i] = false;
        permutation.pop();
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
                vec![3, 1, 2],
                vec![3, 2, 1],
            ]
        );
    }

    #[test]
    fn test_permutations_2() {
        assert_eq!(
            permutations(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],]
        );
    }
}
