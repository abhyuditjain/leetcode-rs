//! 246. Strobogrammatic Number
//!
//! Easy
//!
//! Given a string num which represents an integer, return true if num is a strobogrammatic number.
//! A strobogrammatic number is a number that looks the same when rotated 180 degrees (looked at upside down).
//!
//! Example 1:
//! Input: num = "69"
//! Output: true
//!
//! Example 2:
//! Input: num = "88"
//! Output: true
//!
//! Example 3:
//! Input: num = "962"
//! Output: false
//!
//! Constraints:
//! 1 <= num.length <= 50
//! num consists of only digits.
//! num does not contain any leading zeros except for zero itself.

use std::collections::HashMap;

const CHAR_MAP: [(char, char); 5] = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];

pub fn is_strobogrammatic(num: &str) -> bool {
    let map = CHAR_MAP.into_iter().collect();
    let n = num.len();

    let mut first_half = num.chars().take(n - n / 2);
    let mut second_half = num.chars().rev().take(n / 2);

    loop {
        match (first_half.next(), second_half.next()) {
            (Some(a), Some(b)) => {
                if !is_match(a, b, &map) {
                    return false;
                }
            }
            (Some(a), None) => {
                if !is_match(a, a, &map) {
                    return false;
                }
            }
            _ => break,
        }
    }

    true
}

fn is_match(a: char, b: char, map: &HashMap<char, char>) -> bool {
    if let Some(x) = map.get(&a) {
        return x == &b;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_strobogrammatic() {
        assert!(is_strobogrammatic("69"));
        assert!(is_strobogrammatic("88"));
        assert!(!is_strobogrammatic("962"));
    }
}
