//! 52. N-Queens II
//!
//! Hard
//!
//! The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
//! Given an integer n, return the number of distinct solutions to the n-queens puzzle.
//!
//! Example 1:
//! Input: n = 4
//! Output: 2
//! Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
//!
//! Example 2:
//! Input: n = 1
//! Output: 1
//!
//! Constraints:
//! 1 <= n <= 9

use super::problem_51_n_queens::Board;

pub fn n_queens_count(n: usize) -> usize {
    let mut board = Board::new(n);
    let n_queens_boards = board.n_queens();
    n_queens_boards.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_queens_count() {
        assert_eq!(n_queens_count(4), 2);
        assert_eq!(n_queens_count(1), 1);
    }
}
