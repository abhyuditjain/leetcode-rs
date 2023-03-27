//! 89. Gray Code
//!
//! Medium
//!
//! An n-bit gray code sequence is a sequence of 2n integers where:
//!     - Every integer is in the inclusive range [0, 2n - 1],
//!     - The first integer is 0,
//!     - An integer appears no more than once in the sequence,
//!     - The binary representation of every pair of adjacent integers differs by exactly one bit, and
//!     - The binary representation of the first and last integers differs by exactly one bit.
//!
//! Given an integer n, return any valid n-bit gray code sequence.
//!
//! Example 1:
//! Input: n = 2
//! Output: [0,1,3,2]
//! Explanation:
//! The binary representation of [0,1,3,2] is [00,01,11,10].
//! - 00 and 01 differ by one bit
//! - 01 and 11 differ by one bit
//! - 11 and 10 differ by one bit
//! - 10 and 00 differ by one bit
//! [0,2,3,1] is also a valid gray code sequence, whose binary representation is [00,10,11,01].
//! - 00 and 10 differ by one bit
//! - 10 and 11 differ by one bit
//! - 11 and 01 differ by one bit
//! - 01 and 00 differ by one bit
//!
//! Example 2:
//! Input: n = 1
//! Output: [0,1]
//!
//! Constraints:
//! 1 <= n <= 16

use std::ops::BitOr;

pub fn gray_code_vec(n: usize) -> Vec<i32> {
    gray_code_recursion(n)
}

fn gray_code_recursion(n: usize) -> Vec<i32> {
    if n == 1 {
        return vec![0, 1];
    }

    let gray_codes_n_minus_1 = gray_code_recursion(n - 1);

    // Add 0 bit in front of all (from start to end), basically do nothing
    // Add 1 bit in front of all (from end to start), reverse the order

    gray_codes_n_minus_1
        .clone()
        .into_iter()
        .chain(
            gray_codes_n_minus_1
                .into_iter()
                .rev()
                .map(|x| x.bitor(1 << (n - 1)))
                .collect::<Vec<_>>(),
        )
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code_recursion_1() {
        assert_eq!(gray_code_recursion(1), vec![0, 1]);
    }

    #[test]
    fn test_gray_code_recursion_2() {
        assert_eq!(gray_code_recursion(2), vec![0, 1, 3, 2]);
    }

    #[test]
    fn test_gray_code_recursion_3() {
        assert_eq!(gray_code_recursion(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    }
}
