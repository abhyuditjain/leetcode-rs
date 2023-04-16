//! 1639. Number of Ways to Form a Target String Given a Dictionary
//!
//! Hard
//!
//! You are given a list of strings of the same length words and a string target.
//! Your task is to form target using the given words under the following rules:
//! target should be formed from left to right.
//! To form the ith character (0-indexed) of target, you can choose the kth character of the jth string in words if target[i] = words[j][k].
//! Once you use the kth character of the jth string of words, you can no longer use the xth character of any string in words where x <= k. In other words, all characters to the left of or at index k become unusuable for every string.
//! Repeat the process until you form the string target.
//! Notice that you can use multiple characters from the same string in words provided the conditions above are met.
//! Return the number of ways to form target from words. Since the answer may be too large, return it modulo 109 + 7.
//!
//! Example 1:
//! Input: words = ["acca","bbbb","caca"], target = "aba"
//! Output: 6
//! Explanation: There are 6 ways to form target.
//! "aba" -> index 0 ("acca"), index 1 ("bbbb"), index 3 ("caca")
//! "aba" -> index 0 ("acca"), index 2 ("bbbb"), index 3 ("caca")
//! "aba" -> index 0 ("acca"), index 1 ("bbbb"), index 3 ("acca")
//! "aba" -> index 0 ("acca"), index 2 ("bbbb"), index 3 ("acca")
//! "aba" -> index 1 ("caca"), index 2 ("bbbb"), index 3 ("acca")
//! "aba" -> index 1 ("caca"), index 2 ("bbbb"), index 3 ("caca")
//!
//! Example 2:
//! Input: words = ["abba","baab"], target = "bab"
//! Output: 4
//! Explanation: There are 4 ways to form target.
//! "bab" -> index 0 ("baab"), index 1 ("baab"), index 2 ("abba")
//! "bab" -> index 0 ("baab"), index 1 ("baab"), index 3 ("baab")
//! "bab" -> index 0 ("baab"), index 2 ("baab"), index 3 ("baab")
//! "bab" -> index 1 ("abba"), index 2 ("baab"), index 3 ("baab")
//!
//! Constraints:
//! 1 <= words.length <= 1000
//! 1 <= words[i].length <= 1000
//! All strings in words have the same length.
//! 1 <= target.length <= 1000
//! words[i] and target contain only lowercase English letters.

pub enum Algorithm {
    Memoization,
    Tabulation,
}

const MOD: usize = 1000000007;

pub fn num_ways(words: Vec<String>, target: String, alg: Algorithm) -> usize {
    let words = words
        .into_iter()
        .map(|word| word.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let target = target.chars().collect::<Vec<_>>();

    match alg {
        Algorithm::Memoization => num_ways_memoization(&words, &target),
        Algorithm::Tabulation => num_ways_tabulation(&words, &target),
    }
}

fn num_ways_memoization(words: &[Vec<char>], target: &[char]) -> usize {
    fn memoize(
        words: &[Vec<char>],
        target: &[char],
        i: usize,
        j: usize,
        freq: &[Vec<usize>],
        memo: &mut [Vec<Option<usize>>],
    ) -> usize {
        if j == target.len() {
            return 1;
        }

        if i == words[0].len() || words[0].len() - i < target.len() - j {
            return 0;
        }

        if let Some(count) = memo[i][j] {
            return count;
        }

        let idx = target[j] as usize - 'a' as usize;
        let mut count = memoize(words, target, i + 1, j, freq, memo);
        count += freq[i][idx] * memoize(words, target, i + 1, j + 1, freq, memo);
        count %= MOD;

        memo[i][j] = Some(count);

        count
    }

    let freq = get_char_frequency(words);
    let mut memo = vec![vec![None; target.len()]; words[0].len()];
    memoize(words, target, 0, 0, &freq, &mut memo)
}

fn num_ways_tabulation(words: &[Vec<char>], target: &[char]) -> usize {
    let (m, n) = (words[0].len(), target.len());

    let freq = get_char_frequency(words);
    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = freq[0][target[0] as usize - 'a' as usize];

    for i in 1..m {
        dp[i][0] = (dp[i - 1][0] + freq[i][target[0] as usize - 'a' as usize]) % MOD;

        (1..n).for_each(|j| {
            dp[i][j] = (dp[i - 1][j]
                + dp[i - 1][j - 1] * freq[i][target[j] as usize - 'a' as usize])
                % MOD;
        });
    }

    dp[m - 1][n - 1]
}

fn get_char_frequency(words: &[Vec<char>]) -> Vec<Vec<usize>> {
    let word_len = words[0].len();
    let mut freq = vec![vec![0; 26]; word_len];

    for word in words {
        for j in 0..word.len() {
            let idx = word[j] as usize - 'a' as usize;
            freq[j][idx] += 1;
        }
    }

    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_ways_memoization() {
        assert_eq!(
            num_ways_memoization(
                &[
                    vec!['a', 'c', 'c', 'a'],
                    vec!['b', 'b', 'b', 'b'],
                    vec!['c', 'a', 'c', 'a']
                ],
                &['a', 'b', 'a',],
            ),
            6
        );

        assert_eq!(
            num_ways_memoization(
                &[vec!['a', 'b', 'b', 'a'], vec!['b', 'a', 'a', 'b'],],
                &['b', 'a', 'b',],
            ),
            4
        );
    }

    #[test]
    fn test_num_ways_tabulation() {
        assert_eq!(
            num_ways_tabulation(
                &[
                    vec!['a', 'c', 'c', 'a'],
                    vec!['b', 'b', 'b', 'b'],
                    vec!['c', 'a', 'c', 'a']
                ],
                &['a', 'b', 'a',],
            ),
            6
        );

        assert_eq!(
            num_ways_tabulation(
                &[vec!['a', 'b', 'b', 'a'], vec!['b', 'a', 'a', 'b'],],
                &['b', 'a', 'b',],
            ),
            4
        );
    }
}
