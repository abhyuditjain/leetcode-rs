//! 9. Palindrome Number
//!
//! Easy
//!
//! Given an integer x, return true if x is a palindrome, and false otherwise.
//!
//! Example 1:
//! Input: x = 121
//! Output: true
//! Explanation: 121 reads as 121 from left to right and from right to left.
//!
//! Example 2:
//! Input: x = -121
//! Output: false
//! Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//!
//! Example 3:
//! Input: x = 10
//! Output: false
//! Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//!
//! Constraints:
//! -231 <= x <= 231 - 1
//!
//! Follow up: Could you solve it without converting the integer to a string?

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    x == reverse(x)
}

fn reverse(x: i32) -> i32 {
    let sign = if x < 0 { -1 } else { 1 };
    let mut x = x.abs();

    let mut rev = 0;

    while x > 0 {
        let digit = x % 10;
        rev = rev * 10 + digit;
        x /= 10;
    }

    sign * rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(10));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(121), 121);
        assert_eq!(reverse(-121), -121);
        assert_eq!(reverse(10), 1);
    }
}
