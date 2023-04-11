//! 505. The Maze II
//!
//! Medium
//!
//! There is a ball in a maze with empty spaces (represented as 0) and walls (represented as 1). The ball can go through the empty spaces by rolling up, down, left or right, but it won't stop rolling until hitting a wall. When the ball stops, it could choose the next direction.
//! Given the m x n maze, the ball's start position and the destination, where start = [startrow, startcol] and destination = [destinationrow, destinationcol], return the shortest distance for the ball to stop at the destination. If the ball cannot stop at destination, return -1.
//! The distance is the number of empty spaces traveled by the ball from the start position (excluded) to the destination (included).
//! You may assume that the borders of the maze are all walls (see examples).
//!
//! Example 1:
//! Input: maze = [[0,0,1,0,0],[0,0,0,0,0],[0,0,0,1,0],[1,1,0,1,1],[0,0,0,0,0]], start = [0,4], destination = [4,4]
//! Output: 12
//! Explanation: One possible way is : left -> down -> left -> down -> right -> down -> right.
//! The length of the path is 1 + 1 + 3 + 1 + 2 + 2 + 2 = 12.
//!
//! Example 2:
//! Input: maze = [[0,0,1,0,0],[0,0,0,0,0],[0,0,0,1,0],[1,1,0,1,1],[0,0,0,0,0]], start = [0,4], destination = [3,2]
//! Output: -1
//! Explanation: There is no way for the ball to stop at the destination. Notice that you can pass through the destination but you cannot stop there.
//!
//! Example 3:
//! Input: maze = [[0,0,0,0,0],[1,1,0,0,1],[0,0,0,0,0],[0,1,0,0,1],[0,1,0,0,0]], start = [4,3], destination = [0,1]
//! Output: -1
//!
//! Constraints:
//! m == maze.length
//! n == maze[i].length
//! 1 <= m, n <= 100
//! maze[i][j] is 0 or 1.
//! start.length == 2
//! destination.length == 2
//! 0 <= startrow, destinationrow < m
//! 0 <= startcol, destinationcol < n
//! Both the ball and the destination exist in an empty space, and they will not be in the same position initially.
//! The maze contains at least 2 empty spaces.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
};

pub enum Algorithm {
    Dfs,
    Bfs,
    Dijkstra,
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

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn shortest_distance(
    maze: Vec<Vec<i32>>,
    start: Vec<i32>,
    destination: Vec<i32>,
    alg: Algorithm,
) -> isize {
    let maze: Vec<Vec<Cell>> = maze
        .into_iter()
        .map(|row| row.into_iter().map(|cell| cell.into()).collect())
        .collect();

    let start = (start[0] as isize, start[1] as isize);
    let destination = (destination[0] as isize, destination[1] as isize);

    match alg {
        Algorithm::Dfs => shortest_distance_dfs(&maze, start, destination),
        Algorithm::Bfs => shortest_distance_bfs(&maze, start, destination),
        Algorithm::Dijkstra => shortest_distance_dijkstra(&maze, start, destination),
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    pos: (isize, isize),
    distance: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_distance_dijkstra(
    maze: &[Vec<Cell>],
    start: (isize, isize),
    destination: (isize, isize),
) -> isize {
    use Cell::*;

    let r = maze.len();
    let c = maze[0].len();

    let mut distances = vec![vec![usize::MAX; c]; r];

    let mut pq = BinaryHeap::new();
    pq.push(State {
        pos: start,
        distance: 0,
    });

    distances[start.0 as usize][start.1 as usize] = 0;

    while let Some(State { pos, distance }) = pq.pop() {
        if pos == destination {
            break;
        }

        if distance > distances[pos.0 as usize][pos.1 as usize] {
            continue;
        }

        for (dx, dy) in DIRS {
            let (mut x, mut y) = (pos.0 + dx, pos.1 + dy);
            let mut distance = 0;

            while x >= 0
                && y >= 0
                && x < r as isize
                && y < c as isize
                && maze[x as usize][y as usize] == Empty
            {
                x += dx;
                y += dy;
                distance += 1;
            }

            x -= dx;
            y -= dy;

            if distances[pos.0 as usize][pos.1 as usize] + distance
                < distances[x as usize][y as usize]
            {
                distances[x as usize][y as usize] =
                    distances[pos.0 as usize][pos.1 as usize] + distance;
                pq.push(State {
                    pos: (x, y),
                    distance: distances[x as usize][y as usize],
                });
            }
        }
    }

    match distances[destination.0 as usize][destination.1 as usize] {
        usize::MAX => -1,
        x => x as isize,
    }
}

fn shortest_distance_bfs(
    maze: &[Vec<Cell>],
    start: (isize, isize),
    destination: (isize, isize),
) -> isize {
    use Cell::*;

    let r = maze.len();
    let c = maze[0].len();

    let mut distances = vec![vec![usize::MAX; c]; r];
    distances[start.0 as usize][start.1 as usize] = 0;

    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some((i, j)) = q.pop_front() {
        for (dx, dy) in DIRS {
            let (mut x, mut y) = (i + dx, j + dy);
            let mut distance = 0;

            while x >= 0
                && y >= 0
                && x < r as isize
                && y < c as isize
                && maze[x as usize][y as usize] == Empty
            {
                x += dx;
                y += dy;
                distance += 1;
            }

            x -= dx;
            y -= dy;

            if distances[i as usize][j as usize] + distance < distances[x as usize][y as usize] {
                distances[x as usize][y as usize] = distances[i as usize][j as usize] + distance;
                q.push_back((x, y));
            }
        }
    }

    match distances[destination.0 as usize][destination.1 as usize] {
        usize::MAX => -1,
        x => x as isize,
    }
}

fn shortest_distance_dfs(
    maze: &[Vec<Cell>],
    start: (isize, isize),
    destination: (isize, isize),
) -> isize {
    use Cell::*;

    fn dfs(maze: &[Vec<Cell>], start: (isize, isize), distances: &mut [Vec<usize>]) {
        for (dx, dy) in DIRS {
            let (mut x, mut y) = (start.0 + dx, start.1 + dy);

            let mut distance = 0;

            while x >= 0
                && y >= 0
                && x < maze.len() as isize
                && y < maze[0].len() as isize
                && maze[x as usize][y as usize] == Empty
            {
                x += dx;
                y += dy;
                distance += 1;
            }

            x -= dx;
            y -= dy;

            if distances[start.0 as usize][start.1 as usize] + distance
                < distances[x as usize][y as usize]
            {
                distances[x as usize][y as usize] =
                    distances[start.0 as usize][start.1 as usize] + distance;

                dfs(maze, (x, y), distances);
            }
        }
    }

    let r = maze.len();
    let c = maze[0].len();

    let mut distances = vec![vec![usize::MAX; c]; r];
    distances[start.0 as usize][start.1 as usize] = 0;

    dfs(maze, start, &mut distances);

    match distances[destination.0 as usize][destination.1 as usize] {
        usize::MAX => -1,
        x => x as isize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_distance_dfs_1() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_dfs(&maze, (0, 4), (4, 4)), 12);
    }

    #[test]
    fn test_shortest_distance_dfs_2() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_dfs(&maze, (0, 4), (3, 2)), -1);
    }

    #[test]
    fn test_shortest_distance_dfs_3() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_dfs(&maze, (4, 3), (0, 1)), -1);
    }

    #[test]
    fn test_shortest_distance_bfs_1() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_bfs(&maze, (0, 4), (4, 4)), 12);
    }

    #[test]
    fn test_shortest_distance_bfs_2() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_bfs(&maze, (0, 4), (3, 2)), -1);
    }

    #[test]
    fn test_shortest_distance_bfs_3() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_bfs(&maze, (4, 3), (0, 1)), -1);
    }

    #[test]
    fn test_shortest_distance_dijkstra_1() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_dijkstra(&maze, (0, 4), (4, 4)), 12);
    }

    #[test]
    fn test_shortest_distance_dijkstra_2() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Wall, Empty],
            vec![Wall, Wall, Empty, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_dijkstra(&maze, (0, 4), (3, 2)), -1);
    }

    #[test]
    fn test_shortest_distance_dijkstra_3() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty],
        ];

        assert_eq!(shortest_distance_dijkstra(&maze, (4, 3), (0, 1)), -1);
    }
}
