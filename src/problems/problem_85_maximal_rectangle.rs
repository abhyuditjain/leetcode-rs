//! 85. Maximal Rectangle
//!
//! Hard
//!
//! Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
//!
//! Example 1:
//! Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
//! Output: 6
//! Explanation: The maximal rectangle is shown in the above picture.
//!
//! Example 2:
//! Input: matrix = [["0"]]
//! Output: 0
//!
//! Example 3:
//! Input: matrix = [["1"]]
//! Output: 1
//!
//! Constraints:
//! rows == matrix.length
//! cols == matrix[i].length
//! 1 <= row, cols <= 200
//! matrix[i][j] is '0' or '1'.

use crate::problems::problem_84_largest_rectangle_in_histogram::area_linear;

enum Cell {
    Occupied,
    Vacant,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '1' => Cell::Occupied,
            '0' => Cell::Vacant,
            _ => unimplemented!(),
        }
    }
}

pub fn maximal_rectangle_area(matrix: Vec<Vec<char>>) -> usize {
    let matrix = matrix
        .into_iter()
        .map(|row| row.into_iter().map(|val| val.into()).collect())
        .collect::<Vec<_>>();

    maximal_area_matrix(&matrix)
}

fn maximal_area_matrix(matrix: &[Vec<Cell>]) -> usize {
    assert!(!matrix.is_empty());

    let cols = matrix[0].len();

    let mut max_area = 0;
    let mut histogram = vec![0; cols];

    for row in matrix.iter() {
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Cell::Occupied => histogram[j] += 1,
                Cell::Vacant => histogram[j] = 0,
            }
        }

        max_area = max_area.max(area_linear(&histogram));
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::Cell::*;
    use super::*;

    #[test]
    fn test_maximal_area_matrix_1() {
        let matrix = vec![
            vec![Occupied, Vacant, Occupied, Vacant, Vacant],
            vec![Occupied, Vacant, Occupied, Occupied, Occupied],
            vec![Occupied, Occupied, Occupied, Occupied, Occupied],
            vec![Occupied, Vacant, Vacant, Occupied, Vacant],
        ];

        assert_eq!(maximal_area_matrix(&matrix), 6);
    }

    #[test]
    fn test_maximal_area_matrix_2() {
        let matrix = vec![vec![Vacant]];

        assert_eq!(maximal_area_matrix(&matrix), 0);
    }

    #[test]
    fn test_maximal_area_matrix_3() {
        let matrix = vec![vec![Occupied]];

        assert_eq!(maximal_area_matrix(&matrix), 1);
    }
}
