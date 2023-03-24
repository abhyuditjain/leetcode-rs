//! 73. Set Matrix Zeroes
//!
//! Medium
//!
//! Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
//! You must do it in place.
//!
//! Example 1:
//! Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
//! Output: [[1,0,1],[0,0,0],[1,0,1]]
//!
//! Example 2:
//! Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
//! Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
//!
//! Constraints:
//! m == matrix.length
//! n == matrix[0].length
//! 1 <= m, n <= 200
//! -2^31 <= matrix[i][j] <= 2^31 - 1
//!
//! Follow up:
//! A straightforward solution using O(mn) space is probably a bad idea.
//! A simple improvement uses O(m + n) space, but still not the best solution.
//! Could you devise a constant space solution?

pub fn set_matrix_zeroes(matrix: &mut [Vec<i32>]) {
    assert!(!matrix.is_empty() && !matrix[0].is_empty());

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut is_first_col_zero = false;

    for i in 0..rows {
        if matrix[i][0] == 0 {
            is_first_col_zero = true;
        }

        for j in 1..cols {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    for i in 1..rows {
        for j in 1..cols {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    if matrix[0][0] == 0 {
        for j in 0..cols {
            matrix[0][j] = 0;
        }
    }

    if is_first_col_zero {
        for row in matrix.iter_mut() {
            row[0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_matrix_zeroes_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let correct = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];

        set_matrix_zeroes(&mut matrix);

        assert_eq!(matrix, correct);
    }

    #[test]
    fn test_set_matrix_zeroes_2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let correct = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];

        set_matrix_zeroes(&mut matrix);

        assert_eq!(matrix, correct);
    }
}
