//! 77. Combinations
//!
//! Medium
//!
//! Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].
//! You may return the answer in any order.
//!
//! Example 1:
//! Input: n = 4, k = 2
//! Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
//! Explanation: There are 4 choose 2 = 6 total combinations.
//! Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.
//!
//! Example 2:
//! Input: n = 1, k = 1
//! Output: [[1]]
//! Explanation: There is 1 choose 1 = 1 total combination.
//!
//! Constraints:
//! 1 <= n <= 20
//! 1 <= k <= n

pub fn combinations(n: usize, k: usize) -> Vec<Vec<i32>> {
    assert!(1 <= n && k <= n);
    let mut combinations = vec![];
    let mut combination = vec![];

    backtrack(n, k, &mut combinations, &mut combination, 1);

    combinations
}

fn backtrack(
    n: usize,
    k: usize,
    combinations: &mut Vec<Vec<i32>>,
    combination: &mut Vec<i32>,
    start: usize,
) {
    if combination.len() == k {
        combinations.push(combination.clone());
        return;
    }

    for i in start..=n {
        combination.push(i as i32);
        backtrack(n, k, combinations, combination, i + 1);
        combination.pop();
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
            combinations(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
        )
    }

    #[test]
    fn test_combinations_2() {
        check(combinations(1, 1), vec![vec![1]])
    }
}
