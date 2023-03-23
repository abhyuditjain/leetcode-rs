//! 69. Sqrt(x)
//! Easy
//!
//! Given a non-negative integer x, return the square root of x rounded down to the nearest integer.
//! The returned integer should be non-negative as well.
//! You must not use any built-in exponent function or operator.
//! For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
//!
//! Example 1:
//! Input: x = 4
//! Output: 2
//! Explanation: The square root of 4 is 2, so we return 2.
//!
//! Example 2:
//! Input: x = 8
//! Output: 2
//! Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
//!
//! Constraints:
//! 0 <= x <= 2^31 - 1

use std::cmp::Ordering;

pub fn sqrt(x: i32) -> i32 {
    if x <= 1 {
        return x;
    }

    let (mut low, mut high) = (2, x / 2);

    while low <= high {
        let mid = low + (high - low) / 2;

        if mid.checked_mul(mid).is_none() {
            high = mid - 1;
            continue;
        }

        let square = mid * mid;

        match square.cmp(&x) {
            Ordering::Equal => return mid,
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
        }
    }

    high
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_base_cases() {
        assert_eq!(sqrt(0), 0);
        assert_eq!(sqrt(1), 1);
    }

    #[test]
    fn test_sqrt_normal() {
        assert_eq!(sqrt(2), 1);
        assert_eq!(sqrt(4), 2);
        assert_eq!(sqrt(8), 2);
        assert_eq!(sqrt(9), 3);
        assert_eq!(sqrt(400), 20);
    }

    #[test]
    fn test_sqrt_large() {
        assert_eq!(sqrt(i32::MAX), 46340);
    }
}
