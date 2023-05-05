//! 1456. Maximum Number of Vowels in a Substring of Given Length
//!
//! Medium
//!
//! Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.
//! Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.
//!
//! Example 1:
//! Input: s = "abciiidef", k = 3
//! Output: 3
//! Explanation: The substring "iii" contains 3 vowel letters.
//!
//! Example 2:
//! Input: s = "aeiou", k = 2
//! Output: 2
//! Explanation: Any substring of length 2 contains 2 vowels.
//!
//! Example 3:
//! Input: s = "leetcode", k = 3
//! Output: 2
//! Explanation: "lee", "eet" and "ode" contain 2 vowels.
//!
//! Constraints:
//! 1 <= s.length <= 10^5
//! s consists of lowercase English letters.
//! 1 <= k <= s.length

pub fn max_vowels(s: &str, k: usize) -> usize {
    let chars = s.chars().collect::<Vec<_>>();

    let mut max_len = 0;
    let mut window_vowel_count = 0;

    for right in 0..chars.len() {
        if right >= k {
            let left_char = chars[right - k];
            if is_vowel(left_char) {
                window_vowel_count -= 1;
            }
        }

        if is_vowel(chars[right]) {
            window_vowel_count += 1;
        }

        max_len = max_len.max(window_vowel_count);
    }

    max_len
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_vowels() {
        assert_eq!(max_vowels("abciiidef", 3), 3);
        assert_eq!(max_vowels("aeiou", 2), 2);
        assert_eq!(max_vowels("leetcode", 3), 2);
    }
}
