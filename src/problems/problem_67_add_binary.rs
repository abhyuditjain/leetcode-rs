//! 67. Add Binary
//!
//! Easy
//!
//! Given two binary strings a and b, return their sum as a binary string.
//!
//! Example 1:
//! Input: a = "11", b = "1"
//! Output: "100"
//!
//! Example 2:
//! Input: a = "1010", b = "1011"
//! Output: "10101"
//!
//! Constraints:
//! 1 <= a.length, b.length <= 10^4
//! a and b consist only of '0' or '1' characters.
//! Each string does not contain leading zeros except for the zero itself.

pub fn add_binary_strs<S: AsRef<str>>(a: S, b: S) -> String {
    let mut result = "".to_string();

    let mut max_len = a.as_ref().len().max(b.as_ref().len());

    let mut a_it = a.as_ref().chars().rev();
    let mut b_it = b.as_ref().chars().rev();

    let mut carry = '0';

    loop {
        if max_len.checked_sub(1).is_none() {
            break;
        }

        max_len -= 1;

        let a_bit = a_it.next().unwrap_or('0');
        let b_bit = b_it.next().unwrap_or('0');

        let sum_bit = match (a_bit, b_bit, carry) {
            ('0', '0', '0') => '0',
            ('0', '0', '1') => {
                carry = '0';
                '1'
            }
            ('1', '0', '0') | ('0', '1', '0') | ('1', '1', '1') => '1',
            ('1', '0', '1') | ('0', '1', '1') | ('1', '1', '0') => {
                carry = '1';
                '0'
            }
            _ => unreachable!(),
        };

        result.push(sum_bit);
    }

    if carry == '1' {
        result.push(carry);
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary_strs() {
        assert_eq!(add_binary_strs("11", "1"), "100".to_string());
        assert_eq!(add_binary_strs("1010", "1011"), "10101".to_string());
        assert_eq!(add_binary_strs("1111", "1111"), "11110".to_string());
    }
}
