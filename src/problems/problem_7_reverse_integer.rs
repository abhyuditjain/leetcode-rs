//! 7. Reverse Integer
//!
//! Medium
//!
//! Given a signed 32-bit integer x, return x with its digits reversed.
//! If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
//! Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
//!
//! Example 1:
//! Input: x = 123
//! Output: 321
//!
//! Example 2:
//! Input: x = -123
//! Output: -321
//!
//! Example 3:
//! Input: x = 120
//! Output: 21
//!
//! Constraints:
//! -231 <= x <= 231 - 1

pub fn reverse(x: i32) -> i32 {
    let mut res = 0;
    let mut x = x;

    while x != 0 {
        let digit = x % 10;
        x /= 10;

        if res > i32::MAX / 10 || (res == i32::MAX / 10 && digit >= i32::MAX % 10) {
            return 0;
        }

        if res < i32::MIN / 10 || (res == i32::MIN / 10 && digit <= i32::MIN % 10) {
            return 0;
        }

        res = res * 10 + digit
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_negative() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_with_trailing_zero() {
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn test_max_value() {
        assert_eq!(reverse(2147483647), 0);
    }

    #[test]
    fn test_min_value() {
        assert_eq!(reverse(-2147483648), 0);
    }
}
