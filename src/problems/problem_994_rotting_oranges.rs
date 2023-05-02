//! 994. Rotting Oranges
//!
//! Medium
//!
//! You are given an m x n grid where each cell can have one of three values:
//! 0 representing an empty cell,
//! 1 representing a fresh orange, or
//! 2 representing a rotten orange.
//! Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
//! Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
//!
//! Example 1:
//! Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
//! Output: 4
//!
//! Example 2:
//! Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
//! Output: -1
//! Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
//!
//! Example 3:
//! Input: grid = [[0,2]]
//! Output: 0
//! Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
//!
//! Constraints:
//! m == grid.length
//! n == grid[i].length
//! 1 <= m, n <= 10
//! grid[i][j] is 0, 1, or 2.

use std::collections::VecDeque;

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    match time_to_rot(&mut grid) {
        Some(x) => x as i32,
        None => -1,
    }
}

pub fn time_to_rot(grid: &mut [Vec<i32>]) -> Option<usize> {
    assert!(!grid.is_empty() && !grid[0].is_empty());

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut fresh_oranges: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&x| x == &1).count())
        .sum();

    if fresh_oranges == 0 {
        return Some(0);
    }

    let mut q = VecDeque::new();

    (0..rows).for_each(|i| {
        for j in 0..cols {
            if grid[i][j] == 2 {
                q.push_back((i, j));
            }
        }
    });

    let mut time = 0;

    while !q.is_empty() {
        for _ in 0..q.len() {
            if let Some((i, j)) = q.pop_front() {
                if i > 0 && grid[i - 1][j] == 1 {
                    grid[i - 1][j] = 2;
                    q.push_back((i - 1, j));
                    fresh_oranges -= 1;
                }

                if j > 0 && grid[i][j - 1] == 1 {
                    grid[i][j - 1] = 2;
                    q.push_back((i, j - 1));
                    fresh_oranges -= 1;
                }

                if i < rows - 1 && grid[i + 1][j] == 1 {
                    grid[i + 1][j] = 2;
                    q.push_back((i + 1, j));
                    fresh_oranges -= 1;
                }

                if j < cols - 1 && grid[i][j + 1] == 1 {
                    grid[i][j + 1] = 2;
                    q.push_back((i, j + 1));
                    fresh_oranges -= 1;
                }
            }
        }

        if !q.is_empty() {
            time += 1;
        }
    }

    if fresh_oranges > 0 {
        return None;
    }

    Some(time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oranges_rotting() {
        assert_eq!(
            time_to_rot(&mut [vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1],]),
            Some(4)
        );

        assert_eq!(
            time_to_rot(&mut [vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            None
        );

        assert_eq!(time_to_rot(&mut [vec![0, 2]]), Some(0));
    }
}
