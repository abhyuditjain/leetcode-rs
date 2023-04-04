//! 2405. Optimal Partition of String
//!
//! Medium
//!
//! Given a string s, partition the string into one or more substrings such that the characters in each substring are unique. That is, no letter appears in a single substring more than once.
//! Return the minimum number of substrings in such a partition.
//! Note that each character should belong to exactly one substring in a partition.
//!
//! Example 1:
//! Input: s = "abacaba"
//! Output: 4
//! Explanation:
//! Two possible partitions are ("a","ba","cab","a") and ("ab","a","ca","ba").
//! It can be shown that 4 is the minimum number of substrings needed.
//!
//! Example 2:
//! Input: s = "ssssss"
//! Output: 6
//! Explanation:
//! The only valid partition is ("s","s","s","s","s","s").
//!
//! Constraints:
//! 1 <= s.length <= 10^5
//! s consists of only English lowercase letters.

pub fn num_partitions(s: &str) -> usize {
    let mut letters = [false; 26];

    s.bytes()
        .map(|c| (c - b'a') as usize)
        .filter(|&i| {
            let flag = letters[i];
            if flag {
                letters.fill(false);
            }
            letters[i] = true;
            flag
        })
        .count()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_partitions_1() {
        assert_eq!(num_partitions("abacaba"), 4);
    }

    #[test]
    fn test_num_partitions_2() {
        assert_eq!(num_partitions("ssssss"), 6);
    }
}
