//! 1444. Number of Ways of Cutting a Pizza
//!
//! Hard
//!
//! Given a rectangular pizza represented as a rows x cols matrix containing the following characters: 'A' (an apple) and '.' (empty cell) and given the integer k. You have to cut the pizza into k pieces using k-1 cuts.
//! For each cut you choose the direction: vertical or horizontal, then you choose a cut position at the cell boundary and cut the pizza into two pieces. If you cut the pizza vertically, give the left part of the pizza to a person. If you cut the pizza horizontally, give the upper part of the pizza to a person. Give the last piece of pizza to the last person.
//! Return the number of ways of cutting the pizza such that each piece contains at least one apple. Since the answer can be a huge number, return this modulo 10^9 + 7.
//!
//! Example 1:
//! Input: pizza = ["A..","AAA","..."], k = 3
//! Output: 3
//! Explanation: The figure above shows the three ways to cut the pizza. Note that pieces must contain at least one apple.
//!
//! Example 2:
//! Input: pizza = ["A..","AA.","..."], k = 3
//! Output: 1
//!
//! Example 3:
//! Input: pizza = ["A..","A..","..."], k = 1
//! Output: 1
//!
//! Constraints:
//! 1 <= rows, cols <= 50
//! rows == pizza.length
//! cols == pizza[i].length
//! 1 <= k <= 10
//! pizza consists of characters 'A' and '.' only.

const MOD: usize = 1000000007;

pub enum Cell {
    Apple,
    Other,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'A' => Cell::Apple,
            '.' => Cell::Other,
            _ => unimplemented!(),
        }
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Cell::Apple,
            b'.' => Cell::Other,
            _ => unimplemented!(),
        }
    }
}

impl From<&Cell> for usize {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Apple => 1,
            Cell::Other => 0,
        }
    }
}

pub fn num_ways_of_cutting_pizza(pizza: Vec<String>, k: i32) -> i32 {
    let pizza = pizza
        .into_iter()
        .map(|row| row.chars().map(|cell| cell.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    num_ways(&pizza, k as usize) as i32
}

pub fn num_ways(pizza: &[Vec<Cell>], k: usize) -> usize {
    let rows = pizza.len();
    let cols = pizza[0].len();

    let mut dp = vec![vec![vec![0; cols + 1]; rows + 1]; k];
    let counts = Count::new(pizza);

    for row in (0..rows).rev() {
        for col in (0..cols).rev() {
            dp[0][row][col] = if counts.get_count((row, col)) > 0 {
                1
            } else {
                0
            };
        }
    }

    for remain in 1..k {
        for row in 0..rows {
            for col in 0..cols {
                for next_row in (row + 1)..rows {
                    if counts.get_count((row, col)) > counts.get_count((next_row, col)) {
                        dp[remain][row][col] += dp[remain - 1][next_row][col];
                        dp[remain][row][col] %= MOD;
                    }
                }

                for next_col in (col + 1)..cols {
                    if counts.get_count((row, col)) > counts.get_count((row, next_col)) {
                        dp[remain][row][col] += dp[remain - 1][row][next_col];
                        dp[remain][row][col] %= MOD;
                    }
                }
            }
        }
    }

    dp[k - 1][0][0]
}

struct Count {
    rows: usize,
    cols: usize,
    counts: Vec<Vec<usize>>,
}

impl Count {
    fn new(matrix: &[Vec<Cell>]) -> Self {
        assert!(!matrix.is_empty());

        let rows = matrix.len();
        let cols = matrix[0].len();

        assert!(cols > 0);

        let mut counts = vec![vec![0; cols + 1]; rows + 1];

        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                let count: usize = (&matrix[i][j]).into();

                counts[i][j] = count + counts[i + 1][j] + counts[i][j + 1] - counts[i + 1][j + 1];
            }
        }

        Self { rows, cols, counts }
    }

    /// Get count of apples in submatrix starting at top-left corner till bottom-right end
    fn get_count(&self, (x, y): (usize, usize)) -> usize {
        self.get_count_in_range((x, y), (self.rows - 1, self.cols - 1))
    }

    fn get_count_in_range(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
        self.check_validity((x1, y1), (x2, y2));

        let x2 = x2 + 1;
        let y2 = y2 + 1;

        self.counts[x2][y2] + self.counts[x1][y1] - self.counts[x2][y1] - self.counts[x1][y2]
    }

    fn check_validity(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        assert!(x1 < self.rows);
        assert!(x2 < self.rows);
        assert!(y1 < self.cols);
        assert!(y2 < self.cols);
        assert!(x2 >= x1 && y2 >= y1);
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell::*, *};

    #[test]
    fn test_num_ways_1() {
        let pizza = &[
            vec![Apple, Other, Other],
            vec![Apple, Apple, Apple],
            vec![Other, Other, Other],
        ];

        assert_eq!(num_ways(pizza, 3), 3);
    }

    #[test]
    fn test_num_ways_2() {
        let pizza = &[
            vec![Apple, Other, Other],
            vec![Apple, Apple, Other],
            vec![Other, Other, Other],
        ];

        assert_eq!(num_ways(pizza, 3), 1);
    }

    #[test]
    fn test_num_ways_3() {
        let pizza = &[
            vec![Apple, Other, Other],
            vec![Apple, Other, Other],
            vec![Other, Other, Other],
        ];

        assert_eq!(num_ways(pizza, 1), 1);
    }

    #[test]
    fn test_get_count_in_range() {
        let matrix = vec![
            vec![Apple, Other, Other],
            vec![Other, Apple, Other],
            vec![Other, Other, Apple],
        ];

        let num_apples = Count::new(&matrix);

        assert_eq!(num_apples.get_count_in_range((0, 0), (2, 2)), 3);
        assert_eq!(num_apples.get_count_in_range((1, 1), (2, 2)), 2);
        assert_eq!(num_apples.get_count_in_range((2, 2), (2, 2)), 1);
        assert_eq!(num_apples.get_count_in_range((0, 0), (1, 1)), 2);
        assert_eq!(num_apples.get_count_in_range((0, 0), (1, 0)), 1);
        assert_eq!(num_apples.get_count_in_range((0, 1), (0, 2)), 0);
        assert_eq!(num_apples.get_count_in_range((2, 0), (2, 1)), 0);
    }

    #[test]
    fn test_get_count() {
        let matrix = vec![
            vec![Apple, Other, Other],
            vec![Other, Apple, Other],
            vec![Other, Other, Apple],
        ];

        let num_apples = Count::new(&matrix);

        assert_eq!(num_apples.get_count((0, 0)), 3);
        assert_eq!(num_apples.get_count((1, 1)), 2);
        assert_eq!(num_apples.get_count((2, 2)), 1);
        assert_eq!(num_apples.get_count((0, 1)), 2);
        assert_eq!(num_apples.get_count((2, 0)), 1);
    }
}
