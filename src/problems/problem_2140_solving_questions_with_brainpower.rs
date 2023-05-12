//! 2140. Solving Questions With Brainpower
//!
//! Medium
//!
//! You are given a 0-indexed 2D integer array questions where questions[i] = [pointsi, brainpoweri].
//! The array describes the questions of an exam, where you have to process the questions in order (i.e., starting from question 0) and make a decision whether to solve or skip each question. Solving question i will earn you pointsi points but you will be unable to solve each of the next brainpoweri questions. If you skip question i, you get to make the decision on the next question.
//! For example, given questions = [[3, 2], [4, 3], [4, 4], [2, 5]]:
//! If question 0 is solved, you will earn 3 points but you will be unable to solve questions 1 and 2.
//! If instead, question 0 is skipped and question 1 is solved, you will earn 4 points but you will be unable to solve questions 2 and 3.
//! Return the maximum points you can earn for the exam.
//!
//! Example 1:
//! Input: questions = [[3,2],[4,3],[4,4],[2,5]]
//! Output: 5
//! Explanation: The maximum points can be earned by solving questions 0 and 3.
//! - Solve question 0: Earn 3 points, will be unable to solve the next 2 questions
//! - Unable to solve questions 1 and 2
//! - Solve question 3: Earn 2 points
//! Total points earned: 3 + 2 = 5. There is no other way to earn 5 or more points.
//!
//! Example 2:
//! Input: questions = [[1,1],[2,2],[3,3],[4,4],[5,5]]
//! Output: 7
//! Explanation: The maximum points can be earned by solving questions 1 and 4.
//! - Skip question 0
//! - Solve question 1: Earn 2 points, will be unable to solve the next 2 questions
//! - Unable to solve questions 2 and 3
//! - Solve question 4: Earn 5 points
//! Total points earned: 2 + 5 = 7. There is no other way to earn 7 or more points.
//!
//! Constraints:
//! 1 <= questions.length <= 10^5
//! questions[i].length == 2
//! 1 <= pointsi, brainpoweri <= 10^5

use std::collections::HashMap;

pub enum Algorithm {
    Memoization,
    Tabulation,
}

pub fn most_points(questions: Vec<Vec<i32>>, alg: Algorithm) -> i64 {
    match alg {
        Algorithm::Memoization => most_points_memoization(&questions),
        Algorithm::Tabulation => most_points_tabulation(&questions),
    }
}

fn most_points_memoization(questions: &[Vec<i32>]) -> i64 {
    fn memoize(questions: &[Vec<i32>], start: usize, cache: &mut HashMap<usize, i64>) -> i64 {
        if start >= questions.len() {
            return 0;
        }

        if let Some(&points) = cache.get(&start) {
            return points;
        }

        let points = questions[start][0] as i64;
        let skip = questions[start][1] as usize;

        let max_points_pick_current = points + memoize(questions, start + skip + 1, cache);
        let max_points_skip_current = memoize(questions, start + 1, cache);

        let max_points = max_points_pick_current.max(max_points_skip_current);

        cache.insert(start, max_points);

        max_points
    }

    let mut cache = HashMap::new();
    memoize(questions, 0, &mut cache)
}

fn most_points_tabulation(questions: &[Vec<i32>]) -> i64 {
    let n = questions.len();
    let mut dp = vec![0; n + 1];

    for i in (0..n).rev().step_by(1) {
        let (points, brainpower) = (questions[i][0] as i64, questions[i][1] as usize);
        let j = (i + brainpower + 1).min(n);

        dp[i] = dp[i + 1].max(dp[j] + points);
    }

    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_points_memoization() {
        assert_eq!(
            most_points_memoization(&[vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
            5
        );

        assert_eq!(
            most_points_memoization(&[vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]),
            7
        );
    }

    #[test]
    fn test_most_points_tabulation() {
        assert_eq!(
            most_points_tabulation(&[vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
            5
        );

        assert_eq!(
            most_points_tabulation(&[vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]),
            7
        );
    }
}
