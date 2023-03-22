//! 64. Minimum Path Sum
//!
//! Medium
//!
//! Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right,
//! which minimizes the sum of all numbers along its path.
//! Note: You can only move either down or right at any point in time.
//!
//! Example 1:
//! Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
//! Output: 7
//! Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
//!
//! Example 2:
//! Input: grid = [[1,2,3],[4,5,6]]
//! Output: 12
//!
//! Constraints:
//! m == grid.length
//! n == grid[i].length
//! 1 <= m, n <= 200
//! 0 <= grid[i][j] <= 100

pub fn minimum_path_sum(mut grid: Vec<Vec<i32>>) -> usize {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let r = grid.len();
    let c = grid[0].len();

    for i in 0..r {
        for j in 0..c {
            match (i, j) {
                (0, 0) => {}
                (0, j) => grid[0][j] += grid[0][j - 1],
                (i, 0) => grid[i][0] += grid[i - 1][0],
                (i, j) => grid[i][j] += std::cmp::min(grid[i - 1][j], grid[i][j - 1]),
            }
        }
    }

    grid[r - 1][c - 1] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_path_sum_1() {
        assert_eq!(
            minimum_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn test_minimum_path_sum_2() {
        assert_eq!(minimum_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }

    #[test]
    fn test_minimum_path_sum_3() {
        assert_eq!(minimum_path_sum(vec![]), 0);
        assert_eq!(minimum_path_sum(vec![vec![]]), 0);
    }
}
