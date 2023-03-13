//! 10. Regular Expression Matching
//!
//! Hard
//!
//! Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
//! '.' Matches any single character.​​​​
//! '*' Matches zero or more of the preceding element.
//!
//! The matching should cover the entire input string (not partial).
//!
//! Example 1:
//! Input: s = "aa", p = "a"
//! Output: false
//! Explanation: "a" does not match the entire string "aa".
//!
//! Example 2:
//! Input: s = "aa", p = "a*"
//! Output: true
//! Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
//!
//! Example 3:
//! Input: s = "ab", p = ".*"
//! Output: true
//! Explanation: ".*" means "zero or more (*) of any character (.)".
//!
//! Constraints:
//! 1 <= s.length <= 20
//! 1 <= p.length <= 20
//! s contains only lowercase English letters.
//! p contains only lowercase English letters, '.', and '*'.
//! It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.

use std::collections::HashMap;

pub fn is_match(s: &str, p: &str) -> bool {
    let mut cache = HashMap::new();
    dfs(s.as_bytes(), 0, p.as_bytes(), 0, &mut cache)
}

fn dfs(s: &[u8], i: usize, p: &[u8], j: usize, cache: &mut HashMap<(usize, usize), bool>) -> bool {
    if cache.contains_key(&(i, j)) {
        return *cache.get(&(i, j)).unwrap();
    }

    if i >= s.len() && j >= p.len() {
        return true;
    }

    if j >= p.len() {
        return false;
    }

    let is_match = i < s.len() && (s[i] == p[j] || p[j] == b'.');

    if j + 1 < p.len() && p[j + 1] == b'*' {
        let result = dfs(s, i, p, j + 2, cache) || (is_match && dfs(s, i + 1, p, j, cache));
        cache.insert((i, j), result);
        return result;
    }

    if is_match {
        let result = dfs(s, i + 1, p, j + 1, cache);
        cache.insert((i, j), result);
        return result;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert!(!is_match("aa", "a"));
        assert!(is_match("aa", "a*"));
        assert!(is_match("ab", ".*"));
        assert!(is_match("ab", "c*a*b"));
    }
}
