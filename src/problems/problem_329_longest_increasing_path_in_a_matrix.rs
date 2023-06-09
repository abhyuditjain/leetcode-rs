//! 329. Longest Increasing Path in a Matrix
//!
//! Hard
//!
//! Given an m x n integers matrix, return the length of the longest increasing path in matrix.
//! From each cell, you can either move in four directions: left, right, up, or down. You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).
//!
//! Example 1:
//! Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
//! Output: 4
//! Explanation: The longest increasing path is [1, 2, 6, 9].
//!
//! Example 2:
//! Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
//! Output: 4
//! Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
//!
//! Example 3:
//! Input: matrix = [[1]]
//! Output: 1
//!
//! Constraints:
//! m == matrix.length
//! n == matrix[i].length
//! 1 <= m, n <= 200
//! 0 <= matrix[i][j] <= 2^31 - 1

use std::collections::VecDeque;

pub enum Algorithm {
    Dfs,
    TopologySort,
}

pub fn longest_increasing_path(matrix: &[Vec<i32>], alg: Algorithm) -> usize {
    match alg {
        Algorithm::Dfs => longest_increasing_path_dfs(matrix),
        Algorithm::TopologySort => longest_increasing_path_topology_sort(matrix),
    }
}

fn longest_increasing_path_dfs(matrix: &[Vec<i32>]) -> usize {
    fn dfs(matrix: &[Vec<i32>], i: usize, j: usize, cache: &mut [Vec<usize>]) -> usize {
        if cache[i][j] > 0 {
            return cache[i][j];
        }

        let mut ans = 0;
        if i > 0 && matrix[i - 1][j] > matrix[i][j] {
            ans = ans.max(dfs(matrix, i - 1, j, cache));
        }

        if j > 0 && matrix[i][j - 1] > matrix[i][j] {
            ans = ans.max(dfs(matrix, i, j - 1, cache));
        }

        if i < matrix.len() - 1 && matrix[i + 1][j] > matrix[i][j] {
            ans = ans.max(dfs(matrix, i + 1, j, cache));
        }

        if j < matrix[0].len() - 1 && matrix[i][j + 1] > matrix[i][j] {
            ans = ans.max(dfs(matrix, i, j + 1, cache));
        }

        cache[i][j] = ans + 1;

        cache[i][j]
    }

    if matrix.is_empty() {
        return 0;
    }
    let r = matrix.len();
    let c = matrix[0].len();

    let mut cache = vec![vec![0; c]; r];

    let mut ans = 0;

    for i in 0..r {
        for j in 0..c {
            ans = ans.max(dfs(matrix, i, j, &mut cache));
        }
    }

    ans
}

fn longest_increasing_path_topology_sort(matrix: &[Vec<i32>]) -> usize {
    if matrix.is_empty() {
        return 0;
    }

    let r = matrix.len();
    let c = matrix[0].len();

    let mut outdegrees = vec![vec![0; c]; r];

    let mut q = VecDeque::new();

    for i in 0..r {
        for j in 0..c {
            if i > 0 && matrix[i - 1][j] > matrix[i][j] {
                outdegrees[i][j] += 1;
            }

            if j > 0 && matrix[i][j - 1] > matrix[i][j] {
                outdegrees[i][j] += 1;
            }

            if i < r - 1 && matrix[i + 1][j] > matrix[i][j] {
                outdegrees[i][j] += 1;
            }

            if j < c - 1 && matrix[i][j + 1] > matrix[i][j] {
                outdegrees[i][j] += 1;
            }

            if outdegrees[i][j] == 0 {
                q.push_back((i, j));
            }
        }
    }

    let mut levels = 0;

    while !q.is_empty() {
        for _ in 0..q.len() {
            if let Some((i, j)) = q.pop_front() {
                if i > 0 && matrix[i - 1][j] < matrix[i][j] && outdegrees[i - 1][j] > 0 {
                    outdegrees[i - 1][j] -= 1;

                    if outdegrees[i - 1][j] == 0 {
                        q.push_back((i - 1, j));
                    }
                }

                if j > 0 && matrix[i][j - 1] < matrix[i][j] && outdegrees[i][j - 1] > 0 {
                    outdegrees[i][j - 1] -= 1;

                    if outdegrees[i][j - 1] == 0 {
                        q.push_back((i, j - 1));
                    }
                }

                if i < r - 1 && matrix[i + 1][j] < matrix[i][j] && outdegrees[i + 1][j] > 0 {
                    outdegrees[i + 1][j] -= 1;

                    if outdegrees[i + 1][j] == 0 {
                        q.push_back((i + 1, j));
                    }
                }

                if j < c - 1 && matrix[i][j + 1] < matrix[i][j] && outdegrees[i][j + 1] > 0 {
                    outdegrees[i][j + 1] -= 1;

                    if outdegrees[i][j + 1] == 0 {
                        q.push_back((i, j + 1));
                    }
                }
            }
        }
        levels += 1;
    }

    levels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_increasing_path_dfs_1() {
        assert_eq!(
            longest_increasing_path_dfs(&[vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
    }

    #[test]
    fn test_longest_increasing_path_dfs_2() {
        assert_eq!(
            longest_increasing_path_dfs(&[vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
            4
        );
    }

    #[test]
    fn test_longest_increasing_path_dfs_3() {
        assert_eq!(longest_increasing_path_dfs(&[vec![1]]), 1);
    }

    #[test]
    fn test_longest_increasing_path_topology_sort_1() {
        assert_eq!(
            longest_increasing_path_topology_sort(&[vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
    }

    #[test]
    fn test_longest_increasing_path_topology_sort_2() {
        assert_eq!(
            longest_increasing_path_topology_sort(&[vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
            4
        );
    }

    #[test]
    fn test_longest_increasing_path_topology_sort_3() {
        assert_eq!(longest_increasing_path_topology_sort(&[vec![1]]), 1);
    }
}
