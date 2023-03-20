//! 44. Wildcard Matching
//!
//! Hard
//!
//! Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*' where:
//! '?' Matches any single character.
//! '*' Matches any sequence of characters (including the empty sequence).
//! The matching should cover the entire input string (not partial).
//!
//! Example 1:
//! Input: s = "aa", p = "a"
//! Output: false
//! Explanation: "a" does not match the entire string "aa".
//!
//! Example 2:
//! Input: s = "aa", p = "*"
//! Output: true
//! Explanation: '*' matches any sequence.
//!
//! Example 3:
//! Input: s = "cb", p = "?a"
//! Output: false
//! Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
//!
//! Constraints:
//! 0 <= s.length, p.length <= 2000
//! s contains only lowercase English letters.
//! p contains only lowercase English letters, '?' or '*'.

use std::collections::HashMap;

pub fn is_wildcard_match(s: &str, p: &str) -> bool {
    let p = wildcard_compress(p);
    let mut cache = HashMap::new();
    is_wildcard_match_memoized(s.as_bytes(), 0, p.as_bytes(), 0, &mut cache)
}

fn is_wildcard_match_memoized(
    s: &[u8],
    i: usize,
    p: &[u8],
    j: usize,
    cache: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if let Some(&res) = cache.get(&(i, j)) {
        return res;
    }

    let res = if j >= p.len() {
        i >= s.len() // If p has been exhausted, return if s has been exhausted
    } else if i >= s.len() {
        // If s has been exhausted but p has not, p[p_idx..p.len()] must all be b'*'
        if p[j] == b'*' {
            is_wildcard_match_memoized(s, i, p, j + 1, cache)
        } else {
            false
        }
    } else if s[i] == p[j] || p[j] == b'?' {
        // If sequences match so far, check further
        is_wildcard_match_memoized(s, i + 1, p, j + 1, cache)
    } else if p[j] == b'*' {
        // If p[j] is *
        // Check possibility of:
        //  '*' does not contain current character,
        //  '*' contains current character and no more,
        //  '*' contains current character and more
        // Right-hand operand is only evaluated when left-hand returns false
        is_wildcard_match_memoized(s, i, p, j + 1, cache)
            || is_wildcard_match_memoized(s, i + 1, p, j + 1, cache)
            || is_wildcard_match_memoized(s, i + 1, p, j, cache)
    } else {
        false
    };

    cache.insert((i, j), res);

    *cache.get(&(i, j)).unwrap()
}

fn wildcard_compress(pattern: &str) -> String {
    let mut compressed = "".to_string();

    let mut last_star = false;
    for c in pattern.chars() {
        if c == '*' {
            if !last_star {
                last_star = true;
                compressed.push(c);
            }
        } else {
            last_star = false;
            compressed.push(c);
        }
    }

    compressed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_wildcard_match() {
        assert!(is_wildcard_match("aa", "*"));
        assert!(!is_wildcard_match("aa", "a"));
        assert!(!is_wildcard_match("cb", "?a"));
    }

    #[test]
    fn test_wildcard_compress() {
        assert_eq!(wildcard_compress("abc"), "abc".to_string());
        assert_eq!(wildcard_compress("a*b*c*"), "a*b*c*".to_string());
        assert_eq!(wildcard_compress("a****"), "a*".to_string());
        assert_eq!(wildcard_compress("a**b*c"), "a*b*c".to_string());
    }
}
