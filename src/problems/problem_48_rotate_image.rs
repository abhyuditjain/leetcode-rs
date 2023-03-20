//! 48. Rotate Image
//!
//! Medium
//!
//! You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).
//! You have to rotate the image in-place, which means you have to modify the input 2D matrix directly.
//! DO NOT allocate another 2D matrix and do the rotation.
//!
//! Example 1:
//! Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//! Output: [[7,4,1],[8,5,2],[9,6,3]]
//!
//! Example 2:
//! Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
//! Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
//!
//! Constraints:
//! n == matrix.length == matrix[i].length
//! 1 <= n <= 20
//! -1000 <= matrix[i][j] <= 1000

pub fn rotate_matrix(matrix: &mut [Vec<i32>], clockwise: bool) {
    assert!(!matrix.is_empty());
    assert!(matrix.len() == matrix[0].len());

    transpose(matrix);

    if clockwise {
        mirror_along_vertical_axis(matrix);
    } else {
        mirror_along_horizontal_axis(matrix);
    }
}

fn transpose(matrix: &mut [Vec<i32>]) {
    assert!(!matrix.is_empty());
    assert!(matrix.len() == matrix[0].len());

    for i in 0..matrix.len() {
        for j in 0..i {
            swap(matrix, i, j, j, i);
        }
    }
}

fn mirror_along_vertical_axis(matrix: &mut [Vec<i32>]) {
    assert!(!matrix.is_empty());
    assert!(matrix.len() == matrix[0].len());

    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        for j in 0..cols / 2 {
            swap(matrix, i, j, i, cols - j - 1);
        }
    }
}

fn mirror_along_horizontal_axis(matrix: &mut [Vec<i32>]) {
    assert!(!matrix.is_empty());
    assert!(matrix.len() == matrix[0].len());

    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows / 2 {
        for j in 0..cols {
            swap(matrix, i, j, rows - i - 1, j);
        }
    }
}

fn swap(matrix: &mut [Vec<i32>], r1: usize, c1: usize, r2: usize, c2: usize) {
    let temp = matrix[r1][c1];
    matrix[r1][c1] = matrix[r2][c2];
    matrix[r2][c2] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_clockwise_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_matrix(&mut matrix, true);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn test_rotate_matrix_clockwise_2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate_matrix(&mut matrix, true);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn test_transpose_1() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        transpose(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 3], vec![2, 4]]);
    }

    #[test]
    fn test_transpose_2() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        transpose(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9],]);
    }

    #[test]
    fn test_mirror_along_vertical_axis_1() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        mirror_along_vertical_axis(&mut matrix);
        assert_eq!(matrix, vec![vec![2, 1], vec![4, 3]]);
    }

    #[test]
    fn test_mirror_along_vertical_axis_2() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        mirror_along_vertical_axis(&mut matrix);
        assert_eq!(matrix, vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7],]);
    }

    #[test]
    fn test_mirror_along_horizontal_axis_1() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        mirror_along_horizontal_axis(&mut matrix);
        assert_eq!(matrix, vec![vec![3, 4], vec![1, 2]]);
    }

    #[test]
    fn test_mirror_along_horizontal_axis_2() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        mirror_along_horizontal_axis(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]]);
    }
}
