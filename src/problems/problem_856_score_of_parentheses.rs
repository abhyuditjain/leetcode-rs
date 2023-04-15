//! 856. Score of Parentheses
//!
//! Medium
//!
//! Given a balanced parentheses string s, return the score of the string.
//! The score of a balanced parentheses string is based on the following rule:
//! "()" has score 1.
//! AB has score A + B, where A and B are balanced parentheses strings.
//! (A) has score 2 * A, where A is a balanced parentheses string.
//!
//! Example 1:
//! Input: s = "()"
//! Output: 1
//!
//! Example 2:
//! Input: s = "(())"
//! Output: 2
//!
//! Example 3:
//! Input: s = "()()"
//! Output: 2
//!
//! Constraints:
//! 2 <= s.length <= 50
//! s consists of only '(' and ')'.
//! s is a balanced parentheses string.

pub fn score_of_parentheses(s: &str) -> usize {
    let mut stack = vec![0];

    for c in s.chars() {
        match c {
            '(' => stack.push(0),
            ')' => {
                if let (Some(v), Some(w)) = (stack.pop(), stack.pop()) {
                    stack.push(w + 1.max(2 * v));
                }
            }
            _ => unreachable!(),
        }
    }

    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_of_parentheses() {
        assert_eq!(score_of_parentheses("()"), 1);
        assert_eq!(score_of_parentheses("(())"), 2);
        assert_eq!(score_of_parentheses("()()"), 2);
        assert_eq!(score_of_parentheses("((()))"), 4);
        assert_eq!(score_of_parentheses("(())(())"), 4);
    }
}
