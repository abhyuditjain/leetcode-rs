//! 97. Interleaving String
//!
//! Medium
//!
//! Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.
//! An interleaving of two strings s and t is a configuration where s and t are divided into n and m substrings respectively, such that:
//! s = s1 + s2 + ... + sn
//! t = t1 + t2 + ... + tm
//! |n - m| <= 1
//! The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
//! Note: a + b is the concatenation of strings a and b.
//!
//! Example 1:
//! Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
//! Output: true
//! Explanation: One way to obtain s3 is:
//! Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
//! Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
//! Since s3 can be obtained by interleaving s1 and s2, we return true.
//!
//! Example 2:
//! Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
//! Output: false
//! Explanation: Notice how it is impossible to interleave s2 with any other string to obtain s3.
//!
//! Example 3:
//! Input: s1 = "", s2 = "", s3 = ""
//! Output: true
//!
//! Constraints:
//! 0 <= s1.length, s2.length <= 100
//! 0 <= s3.length <= 200
//! s1, s2, and s3 consist of lowercase English letters.
//!
//! Follow up: Could you solve it using only O(s2.length) additional memory space?

use std::collections::HashMap;

pub enum Algorithm {
    Memoization,
    Tabulation,
}

pub fn is_interleaving(s1: &str, s2: &str, s3: &str, alg: Algorithm) -> bool {
    match alg {
        Algorithm::Memoization => {
            is_interleaving_memoized(s1.as_bytes(), s2.as_bytes(), s3.as_bytes())
        }
        Algorithm::Tabulation => {
            is_interleaving_tabulated(s1.as_bytes(), s2.as_bytes(), s3.as_bytes())
        }
    }
}

fn is_interleaving_tabulated(s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
    dp[s1.len()][s2.len()] = true;

    for i in (0..=s1.len()).rev() {
        for j in (0..=s2.len()).rev() {
            if i < s1.len() && s1[i] == s3[i + j] && dp[i + 1][j] {
                dp[i][j] = true;
            }

            if j < s2.len() && s2[j] == s3[i + j] && dp[i][j + 1] {
                dp[i][j] = true;
            }
        }
    }

    dp[0][0]
}

fn is_interleaving_memoized(s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    let mut cache = HashMap::new();

    is_interleaving_memoized_helper(s1, s2, s3, 0, 0, &mut cache)
}

fn is_interleaving_memoized_helper(
    s1: &[u8],
    s2: &[u8],
    s3: &[u8],
    i: usize,
    j: usize,
    cache: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if i == s1.len() && j == s2.len() {
        return true;
    }

    if let Some(result) = cache.get(&(i, j)) {
        return *result;
    }

    let mut result = false;

    if i < s1.len() && s1[i] == s3[i + j] {
        result = result || is_interleaving_memoized_helper(s1, s2, s3, i + 1, j, cache);
        if result {
            return result;
        }
    }

    if j < s2.len() && s2[j] == s3[i + j] {
        result = result || is_interleaving_memoized_helper(s1, s2, s3, i, j + 1, cache);
        if result {
            return result;
        }
    }

    cache.insert((i, j), result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interleaving_memoized_1() {
        assert!(is_interleaving_memoized(
            "aabcc".as_bytes(),
            "dbbca".as_bytes(),
            "aadbbcbcac".as_bytes()
        ));
    }

    #[test]
    fn test_is_interleaving_memoized_2() {
        assert!(!is_interleaving_memoized(
            "aabcc".as_bytes(),
            "dbbca".as_bytes(),
            "aadbbbaccc".as_bytes()
        ));
    }

    #[test]
    fn test_is_interleaving_memoized_3() {
        assert!(is_interleaving_memoized(
            "".as_bytes(),
            "".as_bytes(),
            "".as_bytes()
        ));
    }

    #[test]
    fn test_is_interleaving_tabulated_1() {
        assert!(is_interleaving_tabulated(
            "aabcc".as_bytes(),
            "dbbca".as_bytes(),
            "aadbbcbcac".as_bytes()
        ));
    }

    #[test]
    fn test_is_interleaving_tabulated_2() {
        assert!(!is_interleaving_tabulated(
            "aabcc".as_bytes(),
            "dbbca".as_bytes(),
            "aadbbbaccc".as_bytes()
        ));
    }

    #[test]
    fn test_is_interleaving_tabulated_3() {
        assert!(is_interleaving_tabulated(
            "".as_bytes(),
            "".as_bytes(),
            "".as_bytes()
        ));
    }
}
