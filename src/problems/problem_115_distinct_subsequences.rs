//! 115. Distinct Subsequences
//!
//! Hard
//!
//! Given two strings s and t, return the number of distinct subsequences of s which equals t.
//! The test cases are generated so that the answer fits on a 32-bit signed integer.
//!
//! Example 1:
//! Input: s = "rabbbit", t = "rabbit"
//! Output: 3
//! Explanation:
//! As shown below, there are 3 ways you can generate "rabbit" from s.
//! rabbbit
//! rabbbit
//! rabbbit
//!
//! Example 2:
//! Input: s = "babgbag", t = "bag"
//! Output: 5
//! Explanation:
//! As shown below, there are 5 ways you can generate "bag" from s.
//! babgbag
//! babgbag
//! babgbag
//! babgbag
//! babgbag
//!
//! Constraints:
//! 1 <= s.length, t.length <= 1000
//! s and t consist of English letters.

use std::collections::HashMap;

pub enum Algorithm {
    Recursive,
    Memoized,
    Tabulation,
    TabulationSpaceOptimized,
}

pub fn num_distinct_subsequences(text: &str, pattern: &str, alg: Algorithm) -> usize {
    match alg {
        Algorithm::Recursive => num_distinct_recursive(text.as_bytes(), pattern.as_bytes()),
        Algorithm::Memoized => num_distinct_memoized(text.as_bytes(), pattern.as_bytes()),
        Algorithm::Tabulation => num_distinct_tabulation(text.as_bytes(), pattern.as_bytes()),
        Algorithm::TabulationSpaceOptimized => {
            num_distinct_tabulation_space_optimized(text.as_bytes(), pattern.as_bytes())
        }
    }
}

fn num_distinct_recursive(text: &[u8], pattern: &[u8]) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if text.is_empty() {
        return 0;
    }

    if text[0] == pattern[0] {
        return num_distinct_recursive(&text[1..], &pattern[1..])
            + num_distinct_recursive(&text[1..], pattern);
    }

    num_distinct_recursive(&text[1..], pattern)
}

fn num_distinct_memoized(text: &[u8], pattern: &[u8]) -> usize {
    let mut cache = HashMap::new();

    num_distinct_memoized_helper(text, 0, pattern, 0, &mut cache)
}

fn num_distinct_memoized_helper(
    text: &[u8],
    i: usize,
    pattern: &[u8],
    j: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if j == pattern.len() {
        return 1;
    }

    if i == text.len() {
        return 0;
    }

    if let Some(count) = cache.get(&(i, j)) {
        return *count;
    }

    let count = match text[i] == pattern[j] {
        true => {
            num_distinct_memoized_helper(text, i + 1, pattern, j + 1, cache)
                + num_distinct_memoized_helper(text, i + 1, pattern, j, cache)
        }
        false => num_distinct_memoized_helper(text, i + 1, pattern, j, cache),
    };

    cache.insert((i, j), count);

    count
}

fn num_distinct_tabulation(text: &[u8], pattern: &[u8]) -> usize {
    let mut dp = vec![vec![0; pattern.len() + 1]; text.len() + 1];

    for row in dp.iter_mut() {
        row[pattern.len()] = 1;
    }

    for i in (0..text.len()).rev() {
        for j in (0..pattern.len()).rev() {
            dp[i][j] += dp[i + 1][j];

            if text[i] == pattern[j] {
                dp[i][j] += dp[i + 1][j + 1];
            }
        }
    }

    dp[0][0]
}

fn num_distinct_tabulation_space_optimized(text: &[u8], pattern: &[u8]) -> usize {
    let mut dp = vec![0; pattern.len()];

    for i in (0..text.len()).rev() {
        let mut prev = 1;

        for j in (0..pattern.len()).rev() {
            let old_dp = dp[j];

            if text[i] == pattern[j] {
                dp[j] += prev;
            }

            prev = old_dp;
        }
    }

    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_distinct_recursive() {
        assert_eq!(
            num_distinct_recursive("rabbbit".as_bytes(), "rabbit".as_bytes()),
            3
        );

        assert_eq!(
            num_distinct_recursive("babgbag".as_bytes(), "bag".as_bytes()),
            5
        );
    }

    #[test]
    fn test_num_distinct_memoized() {
        assert_eq!(
            num_distinct_memoized("rabbbit".as_bytes(), "rabbit".as_bytes()),
            3
        );

        assert_eq!(
            num_distinct_memoized("babgbag".as_bytes(), "bag".as_bytes()),
            5
        );
    }

    #[test]
    fn test_num_distinct_tabulation() {
        assert_eq!(
            num_distinct_tabulation("rabbbit".as_bytes(), "rabbit".as_bytes()),
            3
        );

        assert_eq!(
            num_distinct_tabulation("babgbag".as_bytes(), "bag".as_bytes()),
            5
        );
    }

    #[test]
    fn test_num_distinct_tabulation_space_optimized() {
        assert_eq!(
            num_distinct_tabulation_space_optimized("rabbbit".as_bytes(), "rabbit".as_bytes()),
            3
        );

        assert_eq!(
            num_distinct_tabulation_space_optimized("babgbag".as_bytes(), "bag".as_bytes()),
            5
        );
    }
}
