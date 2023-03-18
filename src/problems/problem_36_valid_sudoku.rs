//! 36. Valid Sudoku
//!
//! Medium
//!
//! Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
//! Each row must contain the digits 1-9 without repetition.
//! Each column must contain the digits 1-9 without repetition.
//! Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
//! Note:
//! A Sudoku board (partially filled) could be valid but is not necessarily solvable.
//! Only the filled cells need to be validated according to the mentioned rules.
//!
//! Example 1:
//! Input: board =
//! [["5","3",".",".","7",".",".",".","."]
//! ,["6",".",".","1","9","5",".",".","."]
//! ,[".","9","8",".",".",".",".","6","."]
//! ,["8",".",".",".","6",".",".",".","3"]
//! ,["4",".",".","8",".","3",".",".","1"]
//! ,["7",".",".",".","2",".",".",".","6"]
//! ,[".","6",".",".",".",".","2","8","."]
//! ,[".",".",".","4","1","9",".",".","5"]
//! ,[".",".",".",".","8",".",".","7","9"]]
//! Output: true
//!
//! Example 2:
//! Input: board =
//! [["8","3",".",".","7",".",".",".","."]
//! ,["6",".",".","1","9","5",".",".","."]
//! ,[".","9","8",".",".",".",".","6","."]
//! ,["8",".",".",".","6",".",".",".","3"]
//! ,["4",".",".","8",".","3",".",".","1"]
//! ,["7",".",".",".","2",".",".",".","6"]
//! ,[".","6",".",".",".",".","2","8","."]
//! ,[".",".",".","4","1","9",".",".","5"]
//! ,[".",".",".",".","8",".",".","7","9"]]
//! Output: false
//! Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
//!
//! Constraints:
//! board.length == 9
//! board[i].length == 9
//! board[i][j] is a digit 1-9 or '.'.

use std::collections::HashSet;

pub struct Sudoku {
    board: Vec<Vec<Option<u8>>>,
}

impl From<Vec<Vec<char>>> for Sudoku {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self {
            board: value
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|c| match c {
                            '.' => None,
                            x => Some(x as u8),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl Sudoku {
    pub fn is_valid(&self) -> bool {
        let mut row_sets = vec![HashSet::new(); 9];
        let mut col_sets = vec![HashSet::new(); 9];
        let mut grid_sets = vec![HashSet::new(); 9];

        for (i, row) in self.board.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                match val {
                    None => continue,
                    Some(val) => {
                        if row_sets[i].contains(&val) {
                            return false;
                        }

                        if col_sets[j].contains(&val) {
                            return false;
                        }

                        let grid_id = (i / 3) * 3 + (j / 3);
                        if grid_sets[grid_id].contains(&val) {
                            return false;
                        }

                        row_sets[i].insert(val);
                        col_sets[j].insert(val);
                        grid_sets[grid_id].insert(val);
                    }
                }
            }
        }

        true
    }
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let sudoku: Sudoku = board.into();
    sudoku.is_valid()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_is_valid_1() {
        let input = vec![
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

        let sudoku: Sudoku = input.into();

        assert!(sudoku.is_valid());
    }

    #[test]
    fn test_sudoku_is_valid_2() {
        let input = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let sudoku: Sudoku = input.into();

        assert!(!sudoku.is_valid());
    }
}
