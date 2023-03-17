//! 32. Longest Valid Parentheses
//!
//! Hard
//!
//! Given a string containing just the characters '(' and ')',
//! return the length of the longest valid (well-formed) parentheses substring
//!
//! Example 1:
//! Input: s = "(()"
//! Output: 2
//! Explanation: The longest valid parentheses substring is "()".
//!
//! Example 2:
//! Input: s = ")()())"
//! Output: 4
//! Explanation: The longest valid parentheses substring is "()()".
//!
//! Example 3:
//! Input: s = ""
//! Output: 0
//!
//! Constraints:
//! 0 <= s.length <= 3 * 104
//! s[i] is '(', or ')'.

pub fn longest_valid_parentheses(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }

    let mut max_len = 0;
    let mut stack = vec![-1];

    for (i, c) in s.char_indices() {
        if c == '(' {
            stack.push(i as i32);
        } else {
            stack.pop();

            if stack.is_empty() {
                stack.push(i as i32);
            } else {
                max_len = max_len.max(i as i32 - stack.last().unwrap());
            }
        }
    }

    max_len as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses_1() {
        assert_eq!(longest_valid_parentheses("(()"), 2);
    }

    #[test]
    fn test_longest_valid_parentheses_2() {
        assert_eq!(longest_valid_parentheses(")()())"), 4);
    }

    #[test]
    fn test_longest_valid_parentheses_3() {
        assert_eq!(longest_valid_parentheses(""), 0);
    }
}
