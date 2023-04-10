//! 490. The Maze
//!
//! Medium
//!
//! There is a ball in a maze with empty spaces (represented as 0) and walls (represented as 1). The ball can go through the empty spaces by rolling up, down, left or right, but it won't stop rolling until hitting a wall. When the ball stops, it could choose the next direction.
//! Given the m x n maze, the ball's start position and the destination, where start = [startrow, startcol] and destination = [destinationrow, destinationcol], return true if the ball can stop at the destination, otherwise return false.
//! You may assume that the borders of the maze are all walls (see examples).
//!
//! Example 1:
//! Input: maze = [[0,0,1,0,0],[0,0,0,0,0],[0,0,0,1,0],[1,1,0,1,1],[0,0,0,0,0]], start = [0,4], destination = [4,4]
//! Output: true
//! Explanation: One possible way is : left -> down -> left -> down -> right -> down -> right.
//!
//! Example 2:
//! Input: maze = [[0,0,1,0,0],[0,0,0,0,0],[0,0,0,1,0],[1,1,0,1,1],[0,0,0,0,0]], start = [0,4], destination = [3,2]
//! Output: false
//! Explanation: There is no way for the ball to stop at the destination. Notice that you can pass through the destination but you cannot stop there.
//!
//! Example 3:
//! Input: maze = [[0,0,0,0,0],[1,1,0,0,1],[0,0,0,0,0],[0,1,0,0,1],[0,1,0,0,0]], start = [4,3], destination = [0,1]
//! Output: false
//!
//! Constraints:
//! m == maze.length
//! n == maze[i].length
//! 1 <= m, n <= 100
//! maze[i][j] is 0 or 1.
//! start.length == 2
//! destination.length == 2
//! 0 <= startrow, destinationrow <= m
//! 0 <= startcol, destinationcol <= n
//! Both the ball and the destination exist in an empty space, and they will not be in the same position initially.
//! The maze contains at least 2 empty spaces.

use std::collections::VecDeque;

pub enum Algorithm {
    Dfs,
    Bfs,
}

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

impl From<i32> for Cell {
    fn from(value: i32) -> Self {
        match value {
            0 => Cell::Empty,
            _ => Cell::Wall,
        }
    }
}

pub fn has_path(
    maze: Vec<Vec<i32>>,
    start: Vec<i32>,
    destination: Vec<i32>,
    alg: Algorithm,
) -> bool {
    let maze: Vec<Vec<Cell>> = maze
        .into_iter()
        .map(|row| row.into_iter().map(|cell| cell.into()).collect())
        .collect();

    let start = (start[0] as usize, start[1] as usize);
    let destination = (destination[0] as usize, destination[1] as usize);

    does_path_exist(&maze, start, destination, alg)
}

fn does_path_exist(
    maze: &[Vec<Cell>],
    start: (usize, usize),
    destination: (usize, usize),
    alg: Algorithm,
) -> bool {
    match alg {
        Algorithm::Dfs => does_path_exist_dfs(maze, start, destination),
        Algorithm::Bfs => does_path_exist_bfs(maze, start, destination),
    }
}

fn does_path_exist_bfs(
    maze: &[Vec<Cell>],
    start: (usize, usize),
    destination: (usize, usize),
) -> bool {
    use Cell::*;

    let r = maze.len();
    let c = maze[0].len();

    let mut visited = vec![vec![false; c]; r];

    let mut q = VecDeque::new();
    q.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((i, j)) = q.pop_front() {
        if (i, j) == destination {
            return true;
        }

        let i = i as isize;
        let j = j as isize;

        for (di, dj) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let mut x = i + di;
            let mut y = j + dj;

            while x >= 0
                && x < r as isize
                && y >= 0
                && y < c as isize
                && maze[x as usize][y as usize] == Empty
            {
                x += di;
                y += dj;
            }

            if !visited[(x - di) as usize][(y - dj) as usize] {
                q.push_back(((x - di) as usize, (y - dj) as usize));
                visited[(x - di) as usize][(y - dj) as usize] = true;
            }
        }
    }

    false
}

fn does_path_exist_dfs(
    maze: &[Vec<Cell>],
    start: (usize, usize),
    destination: (usize, usize),
) -> bool {
    fn dfs(
        maze: &[Vec<Cell>],
        start: (usize, usize),
        destination: (usize, usize),
        visited: &mut [Vec<bool>],
    ) -> bool {
        use Cell::*;

        if visited[start.0][start.1] {
            return false;
        }

        if start == destination {
            return true;
        }

        visited[start.0][start.1] = true;

        let mut r = start.1 as isize + 1;
        let mut l = start.1 as isize - 1;
        let mut u = start.0 as isize - 1;
        let mut d = start.0 as isize + 1;

        while r < maze[0].len() as isize && maze[start.0][r as usize] == Empty {
            r += 1;
        }

        if dfs(maze, (start.0, r as usize - 1), destination, visited) {
            return true;
        }

        while l >= 0 && maze[start.0][l as usize] == Empty {
            l -= 1;
        }

        if dfs(maze, (start.0, (l + 1) as usize), destination, visited) {
            return true;
        }

        while u >= 0 && maze[u as usize][start.1] == Empty {
            u -= 1;
        }

        if dfs(maze, ((u + 1) as usize, start.1), destination, visited) {
            return true;
        }

        while d < maze.len() as isize && maze[d as usize][start.1] == Empty {
            d += 1;
        }

        if dfs(maze, ((d - 1) as usize, start.1), destination, visited) {
            return true;
        }

        false
    }

    let r = maze.len();
    let c = maze[0].len();

    let mut visited = vec![vec![false; c]; r];

    dfs(maze, start, destination, &mut visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_does_path_exist_dfs_1() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert!(does_path_exist_dfs(&maze, (0, 4), (4, 4)));
    }

    #[test]
    fn test_does_path_exist_dfs_2() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert!(!does_path_exist_dfs(&maze, (0, 4), (3, 2)));
    }

    #[test]
    fn test_does_path_exist_dfs_3() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty],
        ];

        assert!(!does_path_exist_dfs(&maze, (4, 3), (0, 1)));
    }

    #[test]
    fn test_does_path_exist_bfs_1() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert!(does_path_exist_bfs(&maze, (0, 4), (4, 4)));
    }

    #[test]
    fn test_does_path_exist_bfs_2() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert!(!does_path_exist_bfs(&maze, (0, 4), (3, 2)));
    }

    #[test]
    fn test_does_path_exist_bfs_3() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty],
        ];

        assert!(!does_path_exist_bfs(&maze, (4, 3), (0, 1)));
    }
}
