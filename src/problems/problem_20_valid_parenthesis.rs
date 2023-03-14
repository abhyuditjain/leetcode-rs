//! 20. Valid Parentheses
//!
//! Easy
//!
//! Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//! An input string is valid if:
//! Open brackets must be closed by the same type of brackets.
//! Open brackets must be closed in the correct order.
//! Every close bracket has a corresponding open bracket of the same type.
//!
//! Example 1:
//! Input: s = "()"
//! Output: true
//!
//! Example 2:
//! Input: s = "()[]{}"
//! Output: true
//!
//! Example 3:
//! Input: s = "(]"
//! Output: false
//!
//! Constraints:
//! 1 <= s.length <= 104
//! s consists of parentheses only '()[]{}'.

pub fn valid_parenthesis(s: &str) -> bool {
    let mut brackets = Vec::new();

    for bracket in s.chars() {
        match bracket {
            '{' => brackets.push('}'),
            '(' => brackets.push(')'),
            '[' => brackets.push(']'),
            closing => {
                if Some(closing) != brackets.pop() {
                    return false;
                }
            }
        }
    }

    brackets.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parenthesis_valid() {
        assert!(valid_parenthesis("()"));
        assert!(valid_parenthesis("(){}[]"));
        assert!(valid_parenthesis("[{()}{()}]"));
    }

    #[test]
    fn test_valid_parenthesis_invalid() {
        assert!(!valid_parenthesis("(]"));
    }
}
