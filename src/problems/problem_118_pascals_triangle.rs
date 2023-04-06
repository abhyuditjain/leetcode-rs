//! 118. Pascal's Triangle
//!
//! Easy
//!
//! Given an integer numRows, return the first numRows of Pascal's triangle.
//! In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
//!
//! Example 1:
//! Input: numRows = 5
//! Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
//!
//! Example 2:
//! Input: numRows = 1
//! Output: [[1]]
//!
//! Constraints:
//! 1 <= numRows <= 30

pub fn pascals_triangle(num_rows: usize) -> Vec<Vec<i32>> {
    let mut triangle = vec![vec![1]];

    if num_rows == 1 {
        return triangle;
    }

    for i in 1..num_rows {
        let prev_row = &triangle[i - 1];
        let mut row = Vec::with_capacity(i + 1);

        row.push(1);

        for j in 1..i {
            row.push(prev_row[j - 1] + prev_row[j]);
        }

        row.push(1);

        triangle.push(row);
    }

    triangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascals_triangle_1() {
        assert_eq!(pascals_triangle(1), vec![vec![1]]);
    }

    #[test]
    fn test_pascals_triangle_2() {
        assert_eq!(pascals_triangle(2), vec![vec![1], vec![1, 1]]);
    }

    #[test]
    fn test_pascals_triangle_3() {
        assert_eq!(
            pascals_triangle(3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1]]
        );
    }

    #[test]
    fn test_pascals_triangle_4() {
        assert_eq!(
            pascals_triangle(4),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
        );
    }

    #[test]
    fn test_pascals_triangle_5() {
        assert_eq!(
            pascals_triangle(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
