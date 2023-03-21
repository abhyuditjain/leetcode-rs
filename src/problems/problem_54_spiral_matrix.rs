//! 54. Spiral Matrix
//!
//! Medium
//!
//! Given an m x n matrix, return all elements of the matrix in spiral order.
//!
//! Example 1:
//! Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//! Output: [1,2,3,6,9,8,7,4,5]
//!
//! Example 2:
//! Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
//! Output: [1,2,3,4,8,12,11,10,9,5,6,7]
//!
//! Constraints:
//! m == matrix.length
//! n == matrix[i].length
//! 1 <= m, n <= 10
//! -100 <= matrix[i][j] <= 100

enum Dir {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_formatted_values(matrix: &[Vec<i32>]) -> Vec<i32> {
    assert!(!matrix.is_empty() && !matrix[0].is_empty());

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = Vec::with_capacity(rows * cols);

    let mut top = 0;
    let mut bottom = rows - 1;
    let mut left = 0;
    let mut right = cols - 1;

    let mut dir = Dir::Right;

    while top <= bottom && left <= right {
        match dir {
            Dir::Right => {
                (left..=right).for_each(|i| {
                    result.push(matrix[top][i]);
                });
                top += 1;
                dir = Dir::Down;
            }

            Dir::Down => {
                (top..=bottom).for_each(|i| {
                    result.push(matrix[i][right]);
                });

                right = match right.checked_sub(1) {
                    Some(right) => right,
                    None => break,
                };

                dir = Dir::Left;
            }

            Dir::Left => {
                (left..=right).rev().for_each(|i| {
                    result.push(matrix[bottom][i]);
                });

                bottom = match bottom.checked_sub(1) {
                    Some(bottom) => bottom,
                    None => break,
                };

                dir = Dir::Up;
            }

            Dir::Up => {
                (top..=bottom).rev().for_each(|i| {
                    result.push(matrix[i][left]);
                });
                left += 1;
                dir = Dir::Right;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_formatted_values_1() {
        assert_eq!(
            spiral_formatted_values(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn test_spiral_formatted_values_2() {
        assert_eq!(
            spiral_formatted_values(&[vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12],]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn test_spiral_formatted_values_3() {
        assert_eq!(spiral_formatted_values(&[vec![3], vec![2]]), vec![3, 2]);
    }
}
