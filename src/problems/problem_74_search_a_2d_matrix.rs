//! 74. Search a 2D Matrix
//!
//! Medium
//!
//! You are given an m x n integer matrix matrix with the following two properties:
//! Each row is sorted in non-decreasing order.
//! The first integer of each row is greater than the last integer of the previous row.
//! Given an integer target, return true if target is in matrix or false otherwise.
//! You must write a solution in O(log(m * n)) time complexity.
//!
//! Example 1:
//! Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
//! Output: true
//!
//! Example 2:
//! Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
//! Output: false
//!
//! Constraints:
//! m == matrix.length
//! n == matrix[i].length
//! 1 <= m, n <= 100
//! -10^4 <= matrix[i][j], target <= 10^4

use std::cmp::Ordering;

pub fn search_sorted_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    assert!(!matrix.is_empty());

    let rows = matrix.len();
    let cols = matrix[0].len();

    if cols == 0 {
        return false;
    }

    let mut low = 0;
    let mut high = rows * cols - 1;

    while low <= high {
        let mid = low + ((high - low) >> 2);

        match matrix[mid / cols][mid % cols].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => low = mid + 1,
            Ordering::Greater => {
                high = match mid.checked_sub(1) {
                    Some(high) => high,
                    None => break,
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_sorted_matrix() {
        assert!(search_sorted_matrix(
            &[vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));

        assert!(!search_sorted_matrix(
            &[vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }

    #[test]
    fn test_search_sorted_matrix_edge_cases() {
        assert!(!search_sorted_matrix(&[vec![2, 3, 4], vec![5, 6, 7]], 1));
        assert!(!search_sorted_matrix(&[vec![2, 3, 4], vec![5, 6, 7]], 8));
        assert!(!search_sorted_matrix(&[vec![]], 1));
    }
}
