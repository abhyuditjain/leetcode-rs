//! 39. Combination Sum
//!
//! Medium
//!
//! Given an array of distinct integers candidates and a target integer target,
//! return a list of all unique combinations of candidates where the chosen numbers sum to target.
//! You may return the combinations in any order.
//! The same number may be chosen from candidates an unlimited number of times.
//! Two combinations are unique if the frequency of at least one of the chosen numbers is different.
//! The test cases are generated such that the number of unique combinations
//! that sum up to target is less than 150 combinations for the given input.
//!
//! Example 1:
//! Input: candidates = [2,3,6,7], target = 7
//! Output: [[2,2,3],[7]]
//! Explanation:
//! 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
//! 7 is a candidate, and 7 = 7.
//! These are the only two combinations.
//!
//! Example 2:
//! Input: candidates = [2,3,5], target = 8
//! Output: [[2,2,2,2],[2,3,3],[3,5]]
//!
//! Example 3:
//! Input: candidates = [2], target = 1
//! Output: []
//!
//! Constraints:
//! 1 <= candidates.length <= 30
//! 2 <= candidates[i] <= 40
//! All elements of candidates are distinct.
//! 1 <= target <= 40

pub fn combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut combinations = vec![];

    backtrack(candidates, &mut combinations, &mut vec![], target, 0);

    combinations
}

fn backtrack(
    candidates: &[i32],
    combinations: &mut Vec<Vec<i32>>,
    combination: &mut Vec<i32>,
    target: i32,
    start: usize,
) {
    if target < 0 {
        return;
    }

    if target == 0 {
        combinations.push(combination.clone());
        return;
    }

    for i in start..candidates.len() {
        combination.push(candidates[i]);
        backtrack(
            candidates,
            combinations,
            combination,
            target - candidates[i],
            i,
        );
        combination.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum_1() {
        assert_eq!(
            combination_sum(&[2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        )
    }

    #[test]
    fn test_combination_sum_2() {
        assert_eq!(
            combination_sum(&[2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        )
    }
    #[test]
    fn test_combination_sum_3() {
        assert_eq!(combination_sum(&[2], 1), vec![vec![]; 0]);
    }
}
