//! 1312. Minimum Insertion Steps to Make a String Palindrome
//!
//! Hard
//!
//! Given a string s. In one step you can insert any character at any index of the string.
//! Return the minimum number of steps to make s palindrome.
//! A Palindrome String is one that reads the same backward as well as forward.
//!
//! Example 1:
//! Input: s = "zzazz"
//! Output: 0
//! Explanation: The string "zzazz" is already palindrome we do not need any insertions.
//!
//! Example 2:
//! Input: s = "mbadm"
//! Output: 2
//! Explanation: String can be "mbdadbm" or "mdbabdm".
//!
//! Example 3:
//! Input: s = "leetcode"
//! Output: 5
//! Explanation: Inserting 5 characters the string becomes "leetcodocteel".
//!
//! Constraints:
//! 1 <= s.length <= 500
//! s consists of lowercase English letters.

use super::problem_516_longest_palindromic_subsequence::{
    longest_palindrome_subsequence, Algorithm as A,
};

pub enum Algorithm {
    Memoization,
    Tabulation,
}

pub fn min_insertions(s: String, alg: Algorithm) -> i32 {
    match alg {
        Algorithm::Memoization => min_insertions_memoization(&s) as i32,
        Algorithm::Tabulation => min_insertions_tabulation(&s) as i32,
    }
}

fn min_insertions_memoization(s: &str) -> usize {
    s.len() - longest_palindrome_subsequence(s, A::Memoization)
}

fn min_insertions_tabulation(s: &str) -> usize {
    s.len() - longest_palindrome_subsequence(s, A::Tabulation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_insertions_memoization() {
        assert_eq!(min_insertions_memoization("zzazz"), 0);
        assert_eq!(min_insertions_memoization("mbadm"), 2);
        assert_eq!(min_insertions_memoization("leetcode"), 5);
    }

    #[test]
    fn test_min_insertions_tabulation() {
        assert_eq!(min_insertions_tabulation("zzazz"), 0);
        assert_eq!(min_insertions_tabulation("mbadm"), 2);
        assert_eq!(min_insertions_tabulation("leetcode"), 5);
    }
}
