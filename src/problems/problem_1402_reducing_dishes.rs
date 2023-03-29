//! 1402. Reducing Dishes
//!
//! Hard
//!
//! A chef has collected data on the satisfaction level of his n dishes. Chef can cook any dish in 1 unit of time.
//! Like-time coefficient of a dish is defined as the time taken to cook that dish including previous dishes multiplied by its satisfaction level i.e. time[i] * satisfaction[i].
//!
//! Return the maximum sum of like-time coefficient that the chef can obtain after dishes preparation.
//!
//! Dishes can be prepared in any order and the chef can discard some dishes to get this maximum value.
//!
//! Example 1:
//! Input: satisfaction = [-1,-8,0,5,-9]
//! Output: 14
//! Explanation: After Removing the second and last dish, the maximum total like-time coefficient will be equal to (-1*1 + 0*2 + 5*3 = 14).
//! Each dish is prepared in one unit of time.
//!
//! Example 2:
//! Input: satisfaction = [4,3,2]
//! Output: 20
//! Explanation: Dishes can be prepared in any order, (2*1 + 3*2 + 4*3 = 20)
//!
//! Example 3:
//! Input: satisfaction = [-1,-4,-5]
//! Output: 0
//! Explanation: People do not like the dishes. No dish is prepared.
//!
//! Constraints:
//! n == satisfaction.length
//! 1 <= n <= 500
//! -1000 <= satisfaction[i] <= 1000

use std::collections::HashMap;

pub enum Algorithm {
    Memoization,
    Tabulation,
}

pub fn maximum_satisfaction(mut satisfaction: Vec<i32>, alg: Algorithm) -> i32 {
    match alg {
        Algorithm::Memoization => maximum_satisfaction_memoized(&mut satisfaction),
        Algorithm::Tabulation => maximum_satisfaction_tabulation(&mut satisfaction),
    }
}

fn maximum_satisfaction_tabulation(satisfaction_scores: &mut [i32]) -> i32 {
    satisfaction_scores.sort_unstable();

    let mut dp = vec![vec![0; satisfaction_scores.len() + 2]; satisfaction_scores.len() + 1];

    for i in (0..satisfaction_scores.len()).rev() {
        for j in 1..=satisfaction_scores.len() {
            let max_score = dp[i + 1][j].max(satisfaction_scores[i] * j as i32 + dp[i + 1][j + 1]);
            dp[i][j] = max_score;
        }
    }

    dp[0][1]
}

fn maximum_satisfaction_memoized(satisfaction_scores: &mut [i32]) -> i32 {
    satisfaction_scores.sort_unstable();

    let mut cache = HashMap::new();

    maximum_satisfaction_memoized_helper(satisfaction_scores, 0, 1, &mut cache)
}

fn maximum_satisfaction_memoized_helper(
    satisfaction_scores: &[i32],
    idx: usize,
    time: usize,
    cache: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if idx >= satisfaction_scores.len() {
        return 0;
    }

    if let Some(max_score) = cache.get(&(idx, time)) {
        return *max_score;
    }

    let max_score_skipped =
        maximum_satisfaction_memoized_helper(satisfaction_scores, idx + 1, time, cache);

    let max_score_picked = satisfaction_scores[idx] * time as i32
        + maximum_satisfaction_memoized_helper(satisfaction_scores, idx + 1, time + 1, cache);

    let max_score = max_score_skipped.max(max_score_picked);

    cache.insert((idx, time), max_score);

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_satisfaction_memoized_1() {
        assert_eq!(maximum_satisfaction_memoized(&mut [-1, -8, 0, 5, -9]), 14);
    }

    #[test]
    fn test_maximum_satisfaction_memoized_2() {
        assert_eq!(maximum_satisfaction_memoized(&mut [4, 3, 2]), 20);
    }

    #[test]
    fn test_maximum_satisfaction_memoized_3() {
        assert_eq!(maximum_satisfaction_memoized(&mut [-1, -4, -5]), 0);
    }

    #[test]
    fn test_maximum_satisfaction_tabulation_1() {
        assert_eq!(maximum_satisfaction_tabulation(&mut [-1, -8, 0, 5, -9]), 14);
    }

    #[test]
    fn test_maximum_satisfaction_tabulation_2() {
        assert_eq!(maximum_satisfaction_tabulation(&mut [4, 3, 2]), 20);
    }

    #[test]
    fn test_maximum_satisfaction_tabulation_3() {
        assert_eq!(maximum_satisfaction_tabulation(&mut [-1, -4, -5]), 0);
    }
}
