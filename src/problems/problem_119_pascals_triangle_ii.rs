//! 119. Pascal's Triangle II
//!
//! Easy
//!
//! Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
//! In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
//!
//! Example 1:
//! Input: rowIndex = 3
//! Output: [1,3,3,1]
//!
//! Example 2:
//! Input: rowIndex = 0
//! Output: [1]
//!
//! Example 3:
//! Input: rowIndex = 1
//! Output: [1,1]
//!
//! Constraints:
//! 0 <= rowIndex <= 33
//!
//! Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?

pub fn pascals_triangle_row(row_id: usize) -> Vec<i32> {
    let mut row = vec![1];

    for _ in 0..row_id {
        for j in (1..row.len()).rev() {
            row[j] += row[j - 1];
        }
        row.push(1);
    }

    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascals_triangle_row_1() {
        assert_eq!(pascals_triangle_row(0), vec![1]);
    }

    #[test]
    fn test_pascals_triangle_row_2() {
        assert_eq!(pascals_triangle_row(1), vec![1, 1]);
    }

    #[test]
    fn test_pascals_triangle_row_3() {
        assert_eq!(pascals_triangle_row(2), vec![1, 2, 1]);
    }

    #[test]
    fn test_pascals_triangle_row_4() {
        assert_eq!(pascals_triangle_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_pascals_triangle_row_5() {
        assert_eq!(pascals_triangle_row(4), vec![1, 4, 6, 4, 1]);
    }
}
