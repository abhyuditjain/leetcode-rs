//! 76. Minimum Window Substring
//!
//! Hard
//!
//! Given two strings s and t of lengths m and n respectively, return the minimum window substring of s
//! such that every character in t (including duplicates) is included in the window.
//! If there is no such substring, return the empty string "".
//! The testcases will be generated such that the answer is unique.
//!
//! Example 1:
//! Input: s = "ADOBECODEBANC", t = "ABC"
//! Output: "BANC"
//! Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
//!
//! Example 2:
//! Input: s = "a", t = "a"
//! Output: "a"
//! Explanation: The entire string s is the minimum window.
//!
//! Example 3:
//! Input: s = "a", t = "aa"
//! Output: ""
//! Explanation: Both 'a's from t must be included in the window.
//! Since the largest window of s only has one 'a', return empty string.
//!
//! Constraints:
//! m == s.length
//! n == t.length
//! 1 <= m, n <= 10^5
//! s and t consist of uppercase and lowercase English letters.
//!
//! Follow up: Could you find an algorithm that runs in O(m + n) time?

#[derive(Default)]
pub struct AlphabetCount([usize; 26], [usize; 26]); // lower and upper case

impl AlphabetCount {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.0
            .iter()
            .chain(self.1.iter())
            .filter(|&x| *x > 0)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, k: &u8) -> usize {
        let bucket = self.get_bucket_ref(k);
        let idx = Self::idx(k);

        bucket[idx]
    }

    pub fn set(&mut self, k: &u8, v: usize) {
        let bucket = self.get_bucket_mut(k);
        let idx = Self::idx(k);

        bucket[idx] = v;
    }

    pub fn increment(&mut self, k: &u8) {
        let bucket = self.get_bucket_mut(k);
        let idx = Self::idx(k);

        bucket[idx] += 1;
    }

    pub fn decrement(&mut self, k: &u8) {
        if self.get(k) == 0 {
            return;
        }

        let bucket = self.get_bucket_mut(k);
        let idx = Self::idx(k);

        bucket[idx] -= 1;
    }

    pub fn contains(&self, k: &u8) -> bool {
        let bucket = self.get_bucket_ref(k);
        let idx = Self::idx(k);

        bucket[idx] > 0
    }

    fn get_bucket_ref(&self, k: &u8) -> &[usize; 26] {
        match *k {
            b'a'..=b'z' => &self.0,
            b'A'..=b'Z' => &self.1,
            _ => unreachable!(),
        }
    }

    fn get_bucket_mut(&mut self, k: &u8) -> &mut [usize; 26] {
        match *k {
            b'a'..=b'z' => &mut self.0,
            b'A'..=b'Z' => &mut self.1,
            _ => unreachable!(),
        }
    }

    fn idx(k: &u8) -> usize {
        match *k {
            b'a'..=b'z' => *k as usize - 'a' as usize,
            b'A'..=b'Z' => *k as usize - 'A' as usize,
            _ => unreachable!(),
        }
    }
}

impl From<&[u8]> for AlphabetCount {
    fn from(value: &[u8]) -> Self {
        let mut counts = AlphabetCount::new();

        for k in value {
            counts.increment(k);
        }

        counts
    }
}

pub fn minimum_window_substring(text: &[u8], pattern: &[u8]) -> String {
    if pattern.is_empty() || pattern.len() > text.len() {
        return String::new();
    }

    let pattern_count: AlphabetCount = pattern.into();
    let need = pattern_count.len();

    let mut answer_start = 0;
    let mut answer_end = 0;
    let mut answer_len = text.len() + 1;

    let mut have = 0;

    let mut window_count = AlphabetCount::new();
    let mut left = 0;

    for right in 0..text.len() {
        let right_char = &text[right];
        window_count.increment(right_char);

        if pattern_count.contains(right_char)
            && window_count.get(right_char) == pattern_count.get(right_char)
        {
            have += 1;
        }

        while have == need {
            let window_len = right - left + 1;
            if window_len < answer_len {
                answer_len = window_len;
                answer_start = left;
                answer_end = right;
            }
            let left_char = &text[left];
            window_count.decrement(left_char);

            if pattern_count.contains(left_char)
                && window_count.get(left_char) < pattern_count.get(left_char)
            {
                have -= 1;
            }
            left += 1;
        }
    }

    if answer_len > text.len() {
        return "".to_string();
    }

    text[answer_start..answer_end + 1]
        .iter()
        .map(|c| *c as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_window_substring() {
        assert_eq!(
            minimum_window_substring("ADOBECODEBANC".as_bytes(), "ABC".as_bytes()),
            "BANC"
        );

        assert_eq!(
            minimum_window_substring("a".as_bytes(), "a".as_bytes()),
            "a"
        );

        assert_eq!(
            minimum_window_substring("aa".as_bytes(), "aa".as_bytes()),
            "aa"
        );

        assert_eq!(
            minimum_window_substring("a".as_bytes(), "aa".as_bytes()),
            ""
        );
    }

    #[test]
    fn test_alphabet_counts_contains() {
        let mut counts = AlphabetCount::new();
        assert!(!counts.contains(&b'a'));

        counts.set(&b'a', 1);
        assert!(counts.contains(&b'a'));
    }

    #[test]
    fn test_alphabet_counts_is_empty() {
        let mut counts = AlphabetCount::new();

        assert!(counts.is_empty());

        counts.set(&b'a', 1);
        assert!(!counts.is_empty());

        counts.set(&b'a', 0);
        assert!(counts.is_empty());
    }

    #[test]
    fn test_alphabet_counts_len() {
        let mut counts = AlphabetCount::new();

        assert_eq!(counts.len(), 0);

        counts.increment(&b'a');
        assert_eq!(counts.len(), 1);

        counts.increment(&b'a');
        assert_eq!(counts.len(), 1);

        counts.increment(&b'b');
        assert_eq!(counts.len(), 2);

        counts.increment(&b'A');
        assert_eq!(counts.len(), 3);
    }

    #[test]
    fn test_alphabet_counts_get() {
        let mut counts = AlphabetCount::new();
        assert_eq!(counts.get(&b'a'), 0);

        counts.set(&b'a', 1);
        assert_eq!(counts.get(&b'a'), 1);
    }

    #[test]
    fn test_alphabet_counts_set() {
        let mut counts = AlphabetCount::new();

        counts.set(&b'a', 26);
        assert_eq!(counts.get(&b'a'), 26);
        assert_eq!(counts.get(&b'b'), 0);

        counts.set(&b'a', 1);
        assert_eq!(counts.get(&b'a'), 1);
    }

    #[test]
    fn test_alphabet_counts_increment() {
        let mut counts = AlphabetCount::new();

        counts.increment(&b'a');
        assert_eq!(counts.get(&b'a'), 1);

        counts.increment(&b'a');
        assert_eq!(counts.get(&b'a'), 2);
    }

    #[test]
    fn test_alphabet_counts_decrement() {
        let mut counts = AlphabetCount::new();

        counts.set(&b'a', 2);
        assert_eq!(counts.get(&b'a'), 2);

        counts.decrement(&b'a');
        assert_eq!(counts.get(&b'a'), 1);

        counts.decrement(&b'a');
        assert_eq!(counts.get(&b'a'), 0);

        counts.decrement(&b'a');
        assert_eq!(counts.get(&b'a'), 0);
    }

    #[test]
    fn test_alphabet_counts_from() {
        let text = "aaabbccAAB".as_bytes();

        let counts: AlphabetCount = text.into();

        assert_eq!(counts.get(&b'a'), 3);
        assert_eq!(counts.get(&b'b'), 2);
        assert_eq!(counts.get(&b'c'), 2);
        assert_eq!(counts.get(&b'A'), 2);
        assert_eq!(counts.get(&b'B'), 1);
        assert_eq!(counts.get(&b'Z'), 0);
    }
}
