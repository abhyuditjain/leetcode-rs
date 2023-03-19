//! 43. Multiply Strings
//!
//! Medium
//!
//! Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
//! Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
//!
//! Example 1:
//! Input: num1 = "2", num2 = "3"
//! Output: "6"
//!
//! Example 2:
//! Input: num1 = "123", num2 = "456"
//! Output: "56088"
//!
//! Constraints:
//! 1 <= num1.length, num2.length <= 200
//! num1 and num2 consist of digits only.
//! Both num1 and num2 do not contain any leading zero, except the number 0 itself.

pub fn multiply_strings(num1: &str, num2: &str) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }

    let first = num1
        .chars()
        .rev()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let second = num2
        .chars()
        .rev()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut answer = vec![];

    for (i, digit) in second.into_iter().enumerate() {
        answer = add(&multiply_one_digit(&first, digit, i), &answer);
    }

    answer.into_iter().rev().map(|x| x.to_string()).collect()
}

fn multiply_one_digit(num: &[u32], digit: u32, pad_zeros: usize) -> Vec<u32> {
    let mut res = vec![0; pad_zeros];

    let mut carry = 0;

    for d in num {
        let mul = d * digit + carry;
        res.push(mul % 10);
        carry = mul / 10;
    }

    if carry != 0 {
        res.push(carry);
    }

    res
}

fn add(num1: &[u32], num2: &[u32]) -> Vec<u32> {
    let mut carry = 0;
    let mut answer = vec![];

    let mut num1_iter = num1.iter();
    let mut num2_iter = num2.iter();

    let mut zipped = vec![];

    loop {
        match (num1_iter.next(), num2_iter.next()) {
            (Some(&x), Some(&y)) => zipped.push((x, y)),
            (Some(&x), None) => zipped.push((x, 0)),
            (None, Some(&y)) => zipped.push((0, y)),
            (None, None) => break,
        }
    }

    for (digit1, digit2) in zipped.into_iter() {
        let addition = digit1 + digit2 + carry;
        answer.push(addition % 10);
        carry = addition / 10;
    }

    if carry != 0 {
        answer.push(carry);
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_strings_base_case() {
        assert_eq!(multiply_strings("0", "0"), "0");
        assert_eq!(multiply_strings("0", "12312312"), "0");
        assert_eq!(multiply_strings("12312312", "0"), "0");
    }

    #[test]
    fn test_multiply_strings_1() {
        assert_eq!(multiply_strings("2", "3"), "6");
    }

    #[test]
    fn test_multiply_strings_2() {
        assert_eq!(multiply_strings("123", "456"), "56088");
    }

    #[test]
    fn test_multiply_strings_3() {
        assert_eq!(
            multiply_strings("123456789", "987654321"),
            "121932631112635269"
        );
    }

    #[test]
    fn test_add_same_length() {
        assert_eq!(add(&[3, 2, 1], &[3, 2, 1]), vec![6, 4, 2]);
    }

    #[test]
    fn test_add_different_length() {
        assert_eq!(add(&[3, 2, 1], &[3]), vec![6, 2, 1]);
        assert_eq!(add(&[3, 2, 1], &[]), vec![3, 2, 1]);
    }

    #[test]
    fn test_multiply_one_digit() {
        assert_eq!(multiply_one_digit(&[3, 2, 1], 1, 0), vec![3, 2, 1]);
        assert_eq!(multiply_one_digit(&[3, 2, 1], 2, 0), vec![6, 4, 2]);
        assert_eq!(multiply_one_digit(&[3, 2, 1], 2, 2), vec![0, 0, 6, 4, 2]);
        assert_eq!(multiply_one_digit(&[3, 2, 1], 9, 2), vec![0, 0, 7, 0, 1, 1]);
    }
}
