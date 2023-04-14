//! 516. Longest Palindromic Subsequence
//!
//! Medium
//!
//! Given a string s, find the longest palindromic subsequence's length in s.
//! A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.
//!
//! Example 1:
//! Input: s = "bbbab"
//! Output: 4
//! Explanation: One possible longest palindromic subsequence is "bbbb".
//!
//! Example 2:
//! Input: s = "cbbd"
//! Output: 2
//! Explanation: One possible longest palindromic subsequence is "bb".
//!
//! Constraints:
//! 1 <= s.length <= 1000
//! s consists only of lowercase English letters.

pub enum Algorithm {
    Memoization,
    Tabulation,
    TabulationSpaceOptimized,
}

pub fn longest_palindrome_subsequence(s: &str, alg: Algorithm) -> usize {
    match alg {
        Algorithm::Memoization => longest_palindrome_subsequence_memoization(s.as_bytes()),
        Algorithm::Tabulation => longest_palindrome_subsequence_tabulation(s.as_bytes()),
        Algorithm::TabulationSpaceOptimized => {
            longest_palindrome_subsequence_tabulation_space_optimized(s.as_bytes())
        }
    }
}

fn longest_palindrome_subsequence_tabulation_space_optimized(s: &[u8]) -> usize {
    let mut dp = vec![0; s.len()];
    let mut dp_prev = vec![0; s.len()];

    for i in (0..s.len()).rev() {
        dp[i] = 1;

        for j in i + 1..s.len() {
            if s[i] == s[j] {
                dp[j] = 2 + dp_prev[j - 1];
            } else {
                dp[j] = std::cmp::max(dp_prev[j], dp[j - 1]);
            }
        }

        dp_prev = dp.clone();
    }

    dp[s.len() - 1]
}

fn longest_palindrome_subsequence_tabulation(s: &[u8]) -> usize {
    let mut dp = vec![vec![0; s.len()]; s.len()];

    for i in (0..s.len()).rev() {
        dp[i][i] = 1;

        for j in i + 1..s.len() {
            if s[i] == s[j] {
                dp[i][j] = 2 + dp[i + 1][j - 1];
            } else {
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[0][s.len() - 1]
}

fn longest_palindrome_subsequence_memoization(s: &[u8]) -> usize {
    fn helper(s: &[u8], i: usize, j: usize, cache: &mut [Vec<usize>]) -> usize {
        if i > j {
            return 0;
        }

        if i == j {
            return 1;
        }

        if cache[i][j] > 0 {
            return cache[i][j];
        }

        if s[i] == s[j] {
            cache[i][j] = 2 + helper(s, i + 1, j - 1, cache);
        } else {
            cache[i][j] = std::cmp::max(helper(s, i + 1, j, cache), helper(s, i, j - 1, cache));
        }

        cache[i][j]
    }

    let mut cache = vec![vec![0; s.len()]; s.len()];
    helper(s, 0, s.len() - 1, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome_subsequence_memoization() {
        assert_eq!(
            longest_palindrome_subsequence_memoization("bbbab".as_bytes()),
            4
        );
        assert_eq!(
            longest_palindrome_subsequence_memoization("cbbd".as_bytes()),
            2
        );
    }

    #[test]
    fn test_longest_palindrome_subsequence_tabulation() {
        assert_eq!(
            longest_palindrome_subsequence_tabulation("bbbab".as_bytes()),
            4
        );
        assert_eq!(
            longest_palindrome_subsequence_tabulation("cbbd".as_bytes()),
            2
        );
    }

    #[test]
    fn test_longest_palindrome_subsequence_tabulation_space_optimized() {
        assert_eq!(
            longest_palindrome_subsequence_tabulation_space_optimized("bbbab".as_bytes()),
            4
        );
        assert_eq!(
            longest_palindrome_subsequence_tabulation_space_optimized("cbbd".as_bytes()),
            2
        );
    }
}
