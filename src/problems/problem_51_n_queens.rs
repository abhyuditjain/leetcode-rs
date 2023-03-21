//! 51. N-Queens
//!
//! Hard
//!
//! The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
//! Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.
//! Each solution contains a distinct board configuration of the n-queens' placement,
//! where 'Q' and '.' both indicate a queen and an empty space, respectively.
//!
//! Example 1:
//! Input: n = 4
//! Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
//! Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
//!
//! Example 2:
//! Input: n = 1
//! Output: [["Q"]]
//!
//! Constraints:
//! 1 <= n <= 9

use std::collections::HashSet;

#[derive(Default)]
pub struct Board {
    n: usize,
    board: Vec<Vec<bool>>,
    row_queens: HashSet<usize>,
    diagonal_queens: HashSet<isize>,
    anti_diagonal_queens: HashSet<usize>,
}

pub fn n_queens(n: usize) -> Vec<Vec<String>> {
    let mut board = Board::new(n);
    board.n_queens()
}

impl Board {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            board: vec![vec![false; n]; n],
            ..Default::default()
        }
    }

    pub fn n_queens(&mut self) -> Vec<Vec<String>> {
        let mut solved_boards = vec![];
        self.n_queens_backtrack(0, &mut solved_boards);
        solved_boards
    }

    pub fn n_queens_backtrack(&mut self, col: usize, solved_boards: &mut Vec<Vec<String>>) {
        if col == self.n {
            solved_boards.push(self.get_board());
            return;
        }

        for row in 0..self.n {
            if self.is_move_valid(row, col) {
                self.place_queen(row, col);
                self.n_queens_backtrack(col + 1, solved_boards);
                self.remove_queen(row, col);
            }
        }
    }

    pub fn get_board(&self) -> Vec<String> {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| match x {
                        true => "Q".to_string(),
                        false => ".".to_string(),
                    })
                    .collect::<String>()
            })
            .collect()
    }

    fn is_move_valid(&self, row: usize, col: usize) -> bool {
        assert!(row < self.n && col < self.n);

        if self.row_queens.contains(&row) {
            return false;
        }

        if self
            .diagonal_queens
            .contains(&(row as isize - col as isize))
        {
            return false;
        }

        if self.anti_diagonal_queens.contains(&(row + col)) {
            return false;
        }

        true
    }

    fn place_queen(&mut self, row: usize, col: usize) {
        assert!(row < self.n && col < self.n);

        self.board[row][col] = true;
        self.row_queens.insert(row);
        self.diagonal_queens.insert(row as isize - col as isize);
        self.anti_diagonal_queens.insert(row + col);
    }

    fn remove_queen(&mut self, row: usize, col: usize) {
        assert!(row < self.n && col < self.n);

        self.board[row][col] = false;
        self.row_queens.remove(&row);
        self.diagonal_queens.remove(&(row as isize - col as isize));
        self.anti_diagonal_queens.remove(&(row + col));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_board() {
        let board = Board {
            n: 3,
            board: vec![
                vec![true, false, false],
                vec![false, true, false],
                vec![false, false, true],
            ],
            ..Default::default()
        };

        assert_eq!(
            board.get_board(),
            vec!["Q..".to_string(), ".Q.".to_string(), "..Q".to_string(),]
        );
    }

    #[test]
    fn test_n_queens_1() {
        assert_eq!(
            n_queens(4),
            vec![
                vec!["..Q.", "Q...", "...Q", ".Q.."],
                vec![".Q..", "...Q", "Q...", "..Q."],
            ]
        )
    }

    #[test]
    fn test_n_queens_2() {
        assert_eq!(n_queens(1), vec![vec!["Q"]])
    }
}
