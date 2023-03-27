//! 91. Decode Ways
//!
//! Medium
//!
//! A message containing letters from A-Z can be encoded into numbers using the following mapping:
//!     'A' -> "1"
//!     'B' -> "2"
//!     ...
//!     'Z' -> "26"
//! To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways).
//!
//! For example, "11106" can be mapped into:
//! "AAJF" with the grouping (1 1 10 6)
//! "KJF" with the grouping (11 10 6)
//! Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".
//!
//! Given a string s containing only digits, return the number of ways to decode it.
//! The test cases are generated so that the answer fits in a 32-bit integer.
//!
//! Example 1:
//! Input: s = "12"
//! Output: 2
//! Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
//!
//! Example 2:
//! Input: s = "226"
//! Output: 3
//! Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
//!
//! Example 3:
//! Input: s = "06"
//! Output: 0
//! Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
//!
//! Constraints:
//! 1 <= s.length <= 100
//! s contains only digits and may contain leading zero(s).

pub fn num_ways_to_decode(text: &str) -> usize {
    let bytes = text.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>();

    let mut cache = vec![None; bytes.len() + 1];

    num_ways_to_decode_memoized(&bytes, 0, &mut cache)
}

fn num_ways_to_decode_memoized(bytes: &[u8], i: usize, cache: &mut [Option<usize>]) -> usize {
    if let Some(count) = cache[i] {
        return count;
    }

    if i == bytes.len() {
        return 1;
    }

    if bytes[i] == 0 {
        return 0;
    }

    if i == bytes.len() - 1 {
        return 1;
    }

    let mut count = num_ways_to_decode_memoized(bytes, i + 1, cache);
    if bytes[i] * 10 + bytes[i + 1] <= 26 {
        count += num_ways_to_decode_memoized(bytes, i + 2, cache);
    }

    cache[i] = Some(count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_ways_to_decode_1() {
        assert_eq!(num_ways_to_decode("12"), 2);
    }

    #[test]
    fn test_num_ways_to_decode_2() {
        assert_eq!(num_ways_to_decode("226"), 3);
    }

    #[test]
    fn test_num_ways_to_decode_3() {
        assert_eq!(num_ways_to_decode("06"), 0);
    }
}
