//! 78. Subsets
//!
//! Medium
//!
//! Given an integer array nums of unique elements, return all possible subsets (the power set).
//! The solution set must not contain duplicate subsets. Return the solution in any order.
//!
//! Example 1:
//! Input: nums = [1,2,3]
//! Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//!
//! Example 2:
//! Input: nums = [0]
//! Output: [[],[0]]
//!
//! Constraints:
//! 1 <= nums.length <= 10
//! -10 <= nums[i] <= 10
//! All the numbers of nums are unique.

pub fn generate_subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut subsets = vec![];
    let mut subset = vec![];

    backtrack(nums, &mut subsets, &mut subset, 0);

    subsets
}

fn backtrack(nums: &[i32], subsets: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, start: usize) {
    subsets.push(subset.clone());

    for i in start..nums.len() {
        subset.push(nums[i]);
        backtrack(nums, subsets, subset, i + 1);
        subset.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(mut output: Vec<Vec<i32>>, mut correct: Vec<Vec<i32>>) {
        output.iter_mut().for_each(|x| x.sort());
        output.sort();

        correct.iter_mut().for_each(|x| x.sort());
        correct.sort();

        assert_eq!(output, correct);
    }

    #[test]
    fn test_combinations_1() {
        check(
            generate_subsets(&[1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ],
        )
    }

    #[test]
    fn test_combinations_2() {
        check(generate_subsets(&[0]), vec![vec![], vec![0]])
    }
}
