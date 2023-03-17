//! 28. Find the Index of the First Occurrence in a String
//!
//! Easy
//!
//! Given two strings needle and haystack,
//! return the index of the first occurrence of needle in haystack,
//! or -1 if needle is not part of haystack.
//!
//! Example 1:
//! Input: haystack = "sadbutsad", needle = "sad"
//! Output: 0
//! Explanation: "sad" occurs at index 0 and 6.
//! The first occurrence is at index 0, so we return 0.
//!
//! Example 2:
//! Input: haystack = "leetcode", needle = "leeto"
//! Output: -1
//! Explanation: "leeto" did not occur in "leetcode", so we return -1.
//!
//! Constraints:
//! 1 <= haystack.length, needle.length <= 104
//! haystack and needle consist of only lowercase English characters.

pub fn find_str(haystack: &str, needle: &str) -> Option<usize> {
    (0..haystack.len())
        .find(|&i| does_substring_start_at(haystack.as_bytes(), needle.as_bytes(), i))
}

fn does_substring_start_at(haystack: &[u8], needle: &[u8], start: usize) -> bool {
    let mut j = 0;
    for c in haystack.iter().skip(start) {
        if j >= needle.len() {
            return true;
        }

        if c != &needle[j] {
            return false;
        }

        j += 1;
    }

    if j != needle.len() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_str() {
        assert_eq!(find_str("sadbutsad", "sad"), Some(0));
        assert_eq!(find_str("leetcode", "leeto"), None);
        assert_eq!(find_str("aaa", "aaaa"), None);
    }
}
