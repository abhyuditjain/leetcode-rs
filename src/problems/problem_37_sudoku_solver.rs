//! 37. Sudoku Solver
//!
//! Hard
//!
//! Write a program to solve a Sudoku puzzle by filling the empty cells.
//! A sudoku solution must satisfy all of the following rules:
//!     Each of the digits 1-9 must occur exactly once in each row.
//!     Each of the digits 1-9 must occur exactly once in each column.
//!     Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
//! The '.' character indicates empty cells.
//!
//! Example 1:
//! Input: board = [
//!     ["5","3",".",".","7",".",".",".","."],
//!     ["6",".",".","1","9","5",".",".","."],
//!     [".","9","8",".",".",".",".","6","."],
//!     ["8",".",".",".","6",".",".",".","3"],
//!     ["4",".",".","8",".","3",".",".","1"],
//!     ["7",".",".",".","2",".",".",".","6"],
//!     [".","6",".",".",".",".","2","8","."],
//!     [".",".",".","4","1","9",".",".","5"],
//!     [".",".",".",".","8",".",".","7","9"]
//! ]
//! Output: [
//!     ["5","3","4","6","7","8","9","1","2"],
//!     ["6","7","2","1","9","5","3","4","8"],
//!     ["1","9","8","3","4","2","5","6","7"],
//!     ["8","5","9","7","6","1","4","2","3"],
//!     ["4","2","6","8","5","3","7","9","1"],
//!     ["7","1","3","9","2","4","8","5","6"],
//!     ["9","6","1","5","3","7","2","8","4"],
//!     ["2","8","7","4","1","9","6","3","5"],
//!     ["3","4","5","2","8","6","1","7","9"]
//! ]
//! Explanation: The input board is shown above and the only valid solution is shown below:
//!
//! Constraints:
//! board.length == 9
//! board[i].length == 9
//! board[i][j] is a digit or '.'.
//! It is guaranteed that the input board has only one solution.

pub struct Sudoku<'a> {
    board: &'a mut Vec<Vec<char>>,
    rows: [[bool; 9]; 9],
    cols: [[bool; 9]; 9],
    boxes: [[bool; 9]; 9],
}

impl<'a> Sudoku<'a> {
    pub fn new(board: &'a mut Vec<Vec<char>>) -> Self {
        let mut sudoku = Self {
            board,
            rows: [[false; 9]; 9],
            cols: [[false; 9]; 9],
            boxes: [[false; 9]; 9],
        };

        for row in 0..9 {
            for col in 0..9 {
                if sudoku.board[row][col] != '.' {
                    sudoku.place(row, col, sudoku.board[row][col]);
                }
            }
        }

        sudoku
    }

    pub fn solve(&mut self, i: usize) -> bool {
        if i >= 9 * 9 {
            return true;
        }

        let (row, col) = (i / 9, i % 9);

        if self.board[row][col] != '.' {
            return self.solve(i + 1);
        }

        for digit in '1'..='9' {
            if self.is_valid(row, col, digit) {
                self.place(row, col, digit);
                if self.solve(i + 1) {
                    return true;
                }
                self.unplace(row, col);
            }
        }

        false
    }

    pub fn is_valid(&self, row: usize, col: usize, digit: char) -> bool {
        let digit = digit as usize - '0' as usize - 1;

        !self.rows[row][digit]
            && !self.cols[col][digit]
            && !self.boxes[row / 3 * 3 + col / 3][digit]
    }

    pub fn place(&mut self, row: usize, col: usize, digit: char) {
        self.board[row][col] = digit;

        let digit = digit as usize - '0' as usize - 1;

        self.rows[row][digit] = true;
        self.cols[col][digit] = true;
        self.boxes[row / 3 * 3 + col / 3][digit] = true;
    }

    pub fn unplace(&mut self, row: usize, col: usize) {
        let digit = self.board[row][col] as usize - '0' as usize - 1;

        self.rows[row][digit] = false;
        self.cols[col][digit] = false;
        self.boxes[row / 3 * 3 + col / 3][digit] = false;

        self.board[row][col] = '.';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let mut sudoku = Sudoku::new(&mut board);

        assert!(sudoku.solve(0));

        let answer = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        assert_eq!(board, answer);
    }
}
