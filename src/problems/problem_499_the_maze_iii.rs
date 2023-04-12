//! 499. The Maze III
//!
//! Hard
//!
//! There is a ball in a maze with empty spaces (represented as 0) and walls (represented as 1).
//! The ball can go through the empty spaces by rolling up, down, left or right, but it won't stop rolling until hitting a wall.
//! When the ball stops, it could choose the next direction.
//! There is also a hole in this maze.
//! The ball will drop into the hole if it rolls onto the hole.
//! Given the m x n maze, the ball's position ball and the hole's position hole, where ball = [ballrow, ballcol] and hole = [holerow, holecol], return a string instructions of all the instructions that the ball should follow to drop in the hole with the shortest distance possible.
//! If there are multiple valid instructions, return the lexicographically minimum one. If the ball can't drop in the hole, return "impossible".
//! If there is a way for the ball to drop in the hole, the answer instructions should contain the characters 'u' (i.e., up), 'd' (i.e., down), 'l' (i.e., left), and 'x' (i.e., right).
//! The distance is the number of empty spaces traveled by the ball from the start position (excluded) to the destination (included).
//! You may assume that the borders of the maze are all walls (see examples).
//!
//! Example 1:
//! Input: maze = [[0,0,0,0,0],[1,1,0,0,1],[0,0,0,0,0],[0,1,0,0,1],[0,1,0,0,0]], ball = [4,3], hole = [0,1]
//! Output: "lul"
//! Explanation: There are two shortest ways for the ball to drop into the hole.
//! The first way is left -> up -> left, represented by "lul".
//! The second way is up -> left, represented by 'ul'.
//! Both ways have shortest distance 6, but the first way is lexicographically smaller because 'l' < 'u'. So the output is "lul".
//!
//! Example 2:
//! Input: maze = [[0,0,0,0,0],[1,1,0,0,1],[0,0,0,0,0],[0,1,0,0,1],[0,1,0,0,0]], ball = [4,3], hole = [3,0]
//! Output: "impossible"
//! Explanation: The ball cannot reach the hole.
//!
//! Example 3:
//! Input: maze = [[0,0,0,0,0,0,0],[0,0,1,0,0,1,0],[0,0,0,0,1,0,0],[0,0,0,0,0,0,1]], ball = [0,4], hole = [3,5]
//! Output: "dldr"
//!
//! Constraints:
//! m == maze.length
//! n == maze[i].length
//! 1 <= m, n <= 100
//! maze[i][j] is 0 or 1.
//! ball.length == 2
//! hole.length == 2
//! 0 <= ballrow, holerow <= m
//! 0 <= ballcol, holecol <= n
//! Both the ball and the hole exist in an empty space, and they will not be in the same position initially.
//! The maze contains at least 2 empty spaces.

use std::{cmp::Ordering, collections::BinaryHeap};

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

#[derive(PartialEq, Eq)]
struct State {
    distance: usize,
    pos: (isize, isize),
    path: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| other.path.cmp(&self.path))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRS: [(isize, isize, char); 4] = [(-1, 0, 'u'), (1, 0, 'd'), (0, -1, 'l'), (0, 1, 'r')];

pub fn find_shortest_way(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String {
    let maze: Vec<Vec<Cell>> = maze
        .into_iter()
        .map(|row| row.into_iter().map(|cell| cell.into()).collect())
        .collect();

    let ball = (ball[0] as isize, ball[1] as isize);
    let hole = (hole[0] as isize, hole[1] as isize);

    shortest_way(&maze, ball, hole)
}

fn shortest_way(maze: &[Vec<Cell>], ball: (isize, isize), hole: (isize, isize)) -> String {
    use Cell::*;

    let rows = maze.len();
    let cols = maze[0].len();

    let mut visited = vec![vec![false; cols]; rows];

    let mut pq = BinaryHeap::new();

    pq.push(State {
        distance: 0,
        pos: ball,
        path: String::new(),
    });

    while let Some(State {
        distance,
        pos,
        path,
    }) = pq.pop()
    {
        if pos == hole {
            return path;
        }

        if visited[pos.0 as usize][pos.1 as usize] {
            continue;
        }
        visited[pos.0 as usize][pos.1 as usize] = true;

        for (dx, dy, dc) in DIRS {
            let (mut x, mut y) = pos;
            let mut d = distance;

            while x >= 0
                && y >= 0
                && x < rows as isize
                && y < cols as isize
                && maze[x as usize][y as usize] == Empty
                && (x, y) != hole
            {
                x += dx;
                y += dy;
                d += 1;
            }

            // We hit a wall, so take 1 step back
            if (x, y) != hole {
                x -= dx;
                y -= dy;
                d -= 1;
            }

            if !visited[x as usize][y as usize] {
                pq.push(State {
                    distance: d,
                    pos: (x, y),
                    path: format!("{path}{dc}"),
                })
            }
        }
    }

    "impossible".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_way_1() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty],
        ];

        assert_eq!(&shortest_way(&maze, (4, 3), (0, 1)), "lul");
    }

    #[test]
    fn test_shortest_way_2() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty],
        ];

        assert_eq!(&shortest_way(&maze, (4, 3), (3, 0)), "impossible");
    }

    #[test]
    fn test_shortest_way_3() {
        use Cell::*;

        let maze = vec![
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Wall, Empty, Empty, Wall, Empty],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Wall],
        ];

        assert_eq!(&shortest_way(&maze, (0, 4), (3, 5)), "dldr");
    }
}
