//! 1254. Number of Closed Islands
//!
//! Medium
//!
//! Given a 2D grid consists of 0s (land) and 1s (water).  An island is a maximal 4-directionally connected group of 0s and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.
//! Return the number of closed islands.
//!
//! Example 1:
//! Input: grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
//! Output: 2
//! Explanation:
//! Islands in gray are closed because they are completely surrounded by water (group of 1s).
//!
//! Example 2:
//! Input: grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
//! Output: 1
//!
//! Example 3:
//! Input: grid = [[1,1,1,1,1,1,1],
//!                [1,0,0,0,0,0,1],
//!                [1,0,1,1,1,0,1],
//!                [1,0,1,0,1,0,1],
//!                [1,0,1,1,1,0,1],
//!                [1,0,0,0,0,0,1],
//!                [1,1,1,1,1,1,1]]
//! Output: 2
//!
//! Constraints:
//! 1 <= grid.length, grid[0].length <= 100
//! 0 <= grid[i][j] <=1

pub fn num_closed_islands(mut grid: Vec<Vec<i32>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        if grid[i][0] == 0 {
            flood_fill(&mut grid, i, 0);
        }

        if grid[i][cols - 1] == 0 {
            flood_fill(&mut grid, i, cols - 1);
        }
    }

    for j in 0..cols {
        if grid[0][j] == 0 {
            flood_fill(&mut grid, 0, j);
        }

        if grid[rows - 1][j] == 0 {
            flood_fill(&mut grid, rows - 1, j);
        }
    }

    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                count += 1;
                flood_fill(&mut grid, i, j);
            }
        }
    }

    count
}

/// Set all connected 0 to 1
fn flood_fill(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    if grid[i][j] == 1 {
        return;
    }

    grid[i][j] = 1;

    if i > 0 {
        flood_fill(grid, i - 1, j);
    }

    if j > 0 {
        flood_fill(grid, i, j - 1);
    }

    if i < grid.len() - 1 {
        flood_fill(grid, i + 1, j);
    }

    if j < grid[0].len() - 1 {
        flood_fill(grid, i, j + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_closed_islands_1() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        assert_eq!(num_closed_islands(grid), 2)
    }

    #[test]
    fn test_num_closed_islands_2() {
        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
        ];

        assert_eq!(num_closed_islands(grid), 1)
    }

    #[test]
    fn test_num_closed_islands_3() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];

        assert_eq!(num_closed_islands(grid), 2)
    }
}
