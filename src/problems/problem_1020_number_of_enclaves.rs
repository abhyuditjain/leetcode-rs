//! 1020. Number of Enclaves
//!
//! Medium
//!
//! You are given an m x n binary matrix grid, where 0 represents a sea cell and 1 represents a land cell.
//! A move consists of walking from one land cell to another adjacent (4-directionally) land cell or walking off the boundary of the grid.
//! Return the number of land cells in grid for which we cannot walk off the boundary of the grid in any number of moves.
//!
//! Example 1:
//! Input: grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
//! Output: 3
//! Explanation: There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.
//!
//! Example 2:
//! Input: grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
//! Output: 0
//! Explanation: All 1s are either on the boundary or can reach the boundary.
//!
//! Constraints:
//! m == grid.length
//! n == grid[i].length
//! 1 <= m, n <= 500
//! grid[i][j] is either 0 or 1.

use std::collections::VecDeque;

pub enum Algorithm {
    Dfs,
    Bfs,
}

pub fn num_enclaves(grid: &[Vec<i32>], alg: Algorithm) -> usize {
    match alg {
        Algorithm::Dfs => num_enclaves_dfs(grid),
        Algorithm::Bfs => num_enclaves_bfs(grid),
    }
}

fn num_enclaves_bfs(grid: &[Vec<i32>]) -> usize {
    let r = grid.len();
    let c = grid[0].len();

    let mut visited = vec![vec![false; c]; r];

    for i in 0..r {
        if grid[i][0] == 1 && !visited[i][0] {
            bfs(grid, i, 0, &mut visited);
        }

        if grid[i][c - 1] == 1 && !visited[i][c - 1] {
            bfs(grid, i, c - 1, &mut visited);
        }
    }

    for j in 0..c {
        if grid[0][j] == 1 && !visited[0][j] {
            bfs(grid, 0, j, &mut visited);
        }

        if grid[r - 1][j] == 1 && !visited[r - 1][j] {
            bfs(grid, r - 1, j, &mut visited);
        }
    }

    get_enclave_size(grid, &visited)
}

fn bfs(grid: &[Vec<i32>], i: usize, j: usize, visited: &mut [Vec<bool>]) {
    let r = grid.len();
    let c = grid[0].len();

    let mut q = VecDeque::new();
    q.push_back((i, j));
    visited[i][j] = true;

    while !q.is_empty() {
        if let Some((i, j)) = q.pop_front() {
            if let Some(i) = i.checked_sub(1) {
                if !visited[i][j] && grid[i][j] == 1 {
                    q.push_back((i, j));
                    visited[i][j] = true;
                }
            }

            if let Some(j) = j.checked_sub(1) {
                if !visited[i][j] && grid[i][j] == 1 {
                    q.push_back((i, j));
                    visited[i][j] = true;
                }
            }

            if i + 1 < r && !visited[i + 1][j] && grid[i + 1][j] == 1 {
                q.push_back((i + 1, j));
                visited[i + 1][j] = true;
            }

            if j + 1 < c && !visited[i][j + 1] && grid[i][j + 1] == 1 {
                q.push_back((i, j + 1));
                visited[i][j + 1] = true;
            }
        }
    }
}

fn num_enclaves_dfs(grid: &[Vec<i32>]) -> usize {
    let r = grid.len();
    let c = grid[0].len();

    let mut visited = vec![vec![false; c]; r];

    for i in 0..r {
        if grid[i][0] == 1 {
            dfs(grid, i, 0, &mut visited);
        }

        if grid[i][c - 1] == 1 {
            dfs(grid, i, c - 1, &mut visited);
        }
    }

    for j in 0..c {
        if grid[0][j] == 1 {
            dfs(grid, 0, j, &mut visited);
        }

        if grid[r - 1][j] == 1 {
            dfs(grid, r - 1, j, &mut visited);
        }
    }

    get_enclave_size(grid, &visited)
}

fn get_enclave_size(grid: &[Vec<i32>], visited: &[Vec<bool>]) -> usize {
    let r = grid.len();
    let c = grid[0].len();

    let mut count = 0;

    for i in 0..r {
        for j in 0..c {
            if !visited[i][j] && grid[i][j] == 1 {
                count += 1;
            }
        }
    }

    count
}

fn dfs(grid: &[Vec<i32>], i: usize, j: usize, visited: &mut [Vec<bool>]) {
    if grid[i][j] == 0 || visited[i][j] {
        return;
    }

    let r = grid.len();
    let c = grid[0].len();

    visited[i][j] = true;

    if i > 0 {
        dfs(grid, i - 1, j, visited);
    }

    if j > 0 {
        dfs(grid, i, j - 1, visited);
    }

    if i < r - 1 {
        dfs(grid, i + 1, j, visited);
    }

    if j < c - 1 {
        dfs(grid, i, j + 1, visited);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_enclaves_dfs_1() {
        assert_eq!(
            num_enclaves_dfs(&[
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            3
        );
    }

    #[test]
    fn test_num_enclaves_dfs_2() {
        assert_eq!(
            num_enclaves_dfs(&[
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            0
        );
    }

    #[test]
    fn test_num_enclaves_bfs_1() {
        assert_eq!(
            num_enclaves_bfs(&[
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            3
        );
    }

    #[test]
    fn test_num_enclaves_bfs_2() {
        assert_eq!(
            num_enclaves_bfs(&[
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            0
        );
    }
}
