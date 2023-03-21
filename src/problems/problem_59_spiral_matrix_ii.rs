//! 59. Spiral Matrix II
//!
//! Medium
//!
//! Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.
//!
//! Example 1:
//! Input: n = 3
//! Output: [[1,2,3],[8,9,4],[7,6,5]]
//!
//! Example 2:
//! Input: n = 1
//! Output: [[1]]
//!
//! Constraints:
//! 1 <= n <= 20

enum Dir {
    Right,
    Down,
    Left,
    Up,
}

pub fn generate_spiral_matrix(n: usize) -> Vec<Vec<i32>> {
    let (rows, cols) = (n, n);

    let mut matrix = vec![vec![-1; n]; n];

    let mut top = 0;
    let mut bottom = rows - 1;
    let mut left = 0;
    let mut right = cols - 1;

    let mut dir = Dir::Right;
    let mut value = 1;

    while top <= bottom && left <= right {
        match dir {
            Dir::Right => {
                (left..=right).for_each(|i| {
                    matrix[top][i] = value;
                    value += 1;
                });

                top += 1;
                dir = Dir::Down;
            }
            Dir::Down => {
                (top..=bottom).for_each(|i| {
                    matrix[i][right] = value;
                    value += 1;
                });

                right = match right.checked_sub(1) {
                    Some(right) => right,
                    None => break,
                };
                dir = Dir::Left;
            }
            Dir::Left => {
                (left..=right).rev().for_each(|i| {
                    matrix[bottom][i] = value;
                    value += 1;
                });

                bottom = match bottom.checked_sub(1) {
                    Some(bottom) => bottom,
                    None => break,
                };
                dir = Dir::Up;
            }
            Dir::Up => {
                (top..=bottom).rev().for_each(|i| {
                    matrix[i][left] = value;
                    value += 1;
                });

                left += 1;
                dir = Dir::Right;
            }
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_spiral_matrix_1() {
        assert_eq!(
            generate_spiral_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn test_generate_spiral_matrix_2() {
        assert_eq!(generate_spiral_matrix(1), vec![vec![1]]);
    }
}
