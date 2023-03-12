//! 3. Longest Substring Without Repeating Characters
//!
//! Medium
//!
//! Given a string s, find the length of the longest substring without repeating characters.
//!
//! Example 1:
//! Input: s = "abcabcbb"
//! Output: 3
//! Explanation: The answer is "abc", with the length of 3.
//!
//! Example 2:
//! Input: s = "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//!
//!  Example 3:
//! Input: s = "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3.
//! Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//!
//! Constraints:
//! 0 <= s.length <= 5 * 104
//! s consists of English letters, digits, symbols and spaces.

pub fn length_of_longest_substring(s: &str) -> usize {
    let mut max_len = 0;

    let mut window_freq = [0; 128];
    let mut left = 0;

    let s = s.as_bytes();

    for right in 0..s.len() {
        let right_char = s[right] as usize;
        window_freq[right_char] += 1;

        while window_freq[right_char] > 1 {
            let left_char = s[left] as usize;
            window_freq[left_char] -= 1;
            left += 1;
        }

        max_len = max_len.max(right - left + 1);
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test3() {
        assert_eq!(length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(length_of_longest_substring("bbbbb"), 1);
        assert_eq!(length_of_longest_substring("pwwkew"), 3);
    }
}
