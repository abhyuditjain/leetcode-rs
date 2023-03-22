//! 63. Unique Paths II
//!
//! Medium
//!
//! You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]).
//! The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]).
//! The robot can only move either down or right at any point in time.
//! An obstacle and space are marked as 1 or 0 respectively in grid.
//! A path that the robot takes cannot include any square that is an obstacle.
//! Return the number of possible unique paths that the robot can take to reach the bottom-right corner.
//!
//! The testcases are generated so that the answer will be less than or equal to 2 * 10^9.
//!
//! Example 1:
//! Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
//! Output: 2
//! Explanation: There is one obstacle in the middle of the 3x3 grid above.
//! There are two ways to reach the bottom-right corner:
//! 1. Right -> Right -> Down -> Down
//! 2. Down -> Down -> Right -> Right
//!
//! Example 2:
//! Input: obstacleGrid = [[0,1],[0,0]]
//! Output: 1
//!
//! Constraints:
//! m == obstacleGrid.length
//! n == obstacleGrid[i].length
//! 1 <= m, n <= 100
//! obstacleGrid[i][j] is 0 or 1.

pub fn num_unique_paths_with_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
    assert!(!grid.is_empty() && !grid[0].is_empty());

    if grid[0][0] == 1 {
        // obstacle at start
        return 0;
    }

    let r = grid.len();
    let c = grid[0].len();

    grid[0][0] = 1;

    for j in 1..c {
        if grid[0][j] == 0 && grid[0][j - 1] == 1 {
            grid[0][j] = 1;
        } else {
            grid[0][j] = 0;
        }
    }

    for i in 1..r {
        if grid[i][0] == 0 && grid[i - 1][0] == 1 {
            grid[i][0] = 1;
        } else {
            grid[i][0] = 0;
        }
    }

    for i in 1..r {
        for j in 1..c {
            if grid[i][j] == 0 {
                grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
            } else {
                grid[i][j] = 0;
            }
        }
    }

    grid[r - 1][c - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_unique_paths_with_obstacles() {
        assert_eq!(
            num_unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            2
        );

        assert_eq!(
            num_unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );

        assert_eq!(num_unique_paths_with_obstacles(vec![vec![1, 0]]), 0);
    }
}
