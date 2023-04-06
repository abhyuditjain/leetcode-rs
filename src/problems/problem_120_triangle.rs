//! 120. Triangle
//!
//! Medium
//!
//! Given a triangle array, return the minimum path sum from top to bottom.
//! For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.
//!
//! Example 1:
//! Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
//! Output: 11
//! Explanation: The triangle looks like:
//!    2
//!   3 4
//!  6 5 7
//! 4 1 8 3
//! The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
//!
//! Example 2:
//! Input: triangle = [[-10]]
//! Output: -10
//!
//! Constraints:
//! 1 <= triangle.length <= 200
//! triangle[0].length == 1
//! triangle[i].length == triangle[i - 1].length + 1
//! -10^4 <= triangle[i][j] <= 10^4
//!
//! Follow up: Could you do this using only O(n) extra space, where n is the total number of rows in the triangle?

use std::collections::HashMap;

pub enum Algorithm {
    Memoized,
    Tabulated,
}

pub fn minimum_total(triangle: Vec<Vec<i32>>, alg: Algorithm) -> i32 {
    match alg {
        Algorithm::Memoized => minimum_total_memoized(triangle),
        Algorithm::Tabulated => minimum_total_tabulated(triangle),
    }
}

fn minimum_total_tabulated(mut triangle: Vec<Vec<i32>>) -> i32 {
    for i in 1..triangle.len() {
        for j in 0..=i {
            let mut smallest_above = i32::MAX;

            if j > 0 {
                smallest_above = triangle[i - 1][j - 1];
            }

            if j < i {
                smallest_above = smallest_above.min(triangle[i - 1][j]);
            }

            let path = smallest_above + triangle[i][j];

            triangle[i][j] = path;
        }
    }

    *triangle.last().unwrap().iter().min().unwrap()
}

fn minimum_total_memoized(triangle: Vec<Vec<i32>>) -> i32 {
    let mut cache = HashMap::new();
    minimum_total_memoized_helper(&triangle, 0, 0, &mut cache)
}

fn minimum_total_memoized_helper(
    triangle: &[Vec<i32>],
    i: usize,
    j: usize,
    cache: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if i >= triangle.len() {
        return 0;
    }

    if let Some(sum) = cache.get(&(i, j)) {
        return *sum;
    }

    let sum = triangle[i][j]
        + std::cmp::min(
            minimum_total_memoized_helper(triangle, i + 1, j, cache),
            minimum_total_memoized_helper(triangle, i + 1, j + 1, cache),
        );

    cache.insert((i, j), sum);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_total_memoized_1() {
        assert_eq!(
            minimum_total_memoized(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }

    #[test]
    fn test_minimum_total_memoized_2() {
        assert_eq!(minimum_total_memoized(vec![vec![-10]]), -10);
    }

    #[test]
    fn test_minimum_total_tabulated_1() {
        assert_eq!(
            minimum_total_tabulated(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }

    #[test]
    fn test_minimum_total_tabulated_2() {
        assert_eq!(minimum_total_tabulated(vec![vec![-10]]), -10);
    }
}
