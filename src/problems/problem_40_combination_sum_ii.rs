//! 40. Combination Sum II
//!
//! Medium
//!
//! Given a collection of candidate numbers (candidates) and a target number (target),
//! find all unique combinations in candidates where the candidate numbers sum to target.
//! Each number in candidates may only be used once in the combination.
//! Note: The solution set must not contain duplicate combinations.
//!
//! Example 1:
//! Input: candidates = [10,1,2,7,6,1,5], target = 8
//! Output:
//! [
//!     [1,1,6],
//!     [1,2,5],
//!     [1,7],
//!     [2,6]
//! ]
//!
//! Example 2:
//! Input: candidates = [2,5,2,1,2], target = 5
//! Output:
//! [
//!     [1,2,2],
//!     [5]
//! ]
//!
//! Constraints:
//! 1 <= candidates.length <= 100
//! 1 <= candidates[i] <= 50
//! 1 <= target <= 30

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;

    candidates.sort_unstable();

    let mut combinations = vec![];

    backtrack(&candidates, &mut combinations, &mut vec![], target, 0);

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
        if i > start && candidates[i] == candidates[i - 1] {
            continue;
        }

        if target - candidates[i] < 0 {
            break;
        }

        combination.push(candidates[i]);
        backtrack(
            candidates,
            combinations,
            combination,
            target - candidates[i],
            i + 1,
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
            combination_sum(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test_combination_sum_2() {
        assert_eq!(
            combination_sum(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        )
    }
}
