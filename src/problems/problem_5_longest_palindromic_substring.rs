//! 5. Longest Palindromic Substring
//!
//! Medium
//!
//! Given a string s, return the longest palindromic substring in s.
//!
//! Example 1:
//! Input: s = "babad"
//! Output: "bab"
//! Explanation: "aba" is also a valid answer.
//!
//! Example 2:
//! Input: s = "cbbd"
//! Output: "bb"
//!
//! Constraints:
//! 1 <= s.length <= 1000
//! s consist of only digits and English letters.

pub fn longest_palindrome(s: &str) -> String {
    let mut start = 0;
    let mut end = 0;

    for i in 0..s.len() {
        let len1 = max_palindrome_length_with_center(s.as_bytes(), (i, i));
        let len2 = max_palindrome_length_with_center(s.as_bytes(), (i, i + 1));

        let max_len = len1.max(len2);

        if max_len > end - start {
            start = i - (max_len - 1) / 2;
            end = i + max_len / 2;
        }
    }

    s[start..end + 1].to_owned()
}

fn max_palindrome_length_with_center(text: &[u8], (left, right): (usize, usize)) -> usize {
    let mut i = left as isize;
    let mut j = right as isize;

    while i >= 0 && j < text.len() as isize && text[i as usize] == text[j as usize] {
        i -= 1;
        j += 1;
    }

    (j - i - 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("babad"), "aba".to_owned());
        assert_eq!(longest_palindrome("cbbd"), "bb".to_owned());
    }

    #[test]
    fn test_max_palindrome_length_with_center() {
        assert_eq!(
            max_palindrome_length_with_center("bab".as_bytes(), (1, 1)),
            3
        );

        assert_eq!(
            max_palindrome_length_with_center("baab".as_bytes(), (1, 2)),
            4
        );

        assert_eq!(
            max_palindrome_length_with_center("babab".as_bytes(), (1, 2)),
            0
        );
    }
}
