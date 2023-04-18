//! 1768. Merge Strings Alternately
//!
//! Easy
//!
//! You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1.
//! If a string is longer than the other, append the additional letters onto the end of the merged string.
//! Return the merged string.
//!
//! Example 1:
//! Input: word1 = "abc", word2 = "pqr"
//! Output: "apbqcr"
//! Explanation: The merged string will be merged as so:
//! word1:  a   b   c
//! word2:    p   q   r
//! merged: a p b q c r
//!
//! Example 2:
//! Input: word1 = "ab", word2 = "pqrs"
//! Output: "apbqrs"
//! Explanation: Notice that as word2 is longer, "rs" is appended to the end.
//! word1:  a   b
//! word2:    p   q   r   s
//! merged: a p b q   r   s
//!
//! Example 3:
//! Input: word1 = "abcd", word2 = "pq"
//! Output: "apbqcd"
//! Explanation: Notice that as word1 is longer, "cd" is appended to the end.
//! word1:  a   b   c   d
//! word2:    p   q
//! merged: a p b q c   d
//!
//! Constraints:
//! 1 <= word1.length, word2.length <= 100
//! word1 and word2 consist of lowercase English letters.

pub fn merge_alternately(word1: &str, word2: &str) -> String {
    let mut merged = String::with_capacity(word1.len() + word2.len());

    let (mut first, mut second) = (word1.chars(), word2.chars());

    loop {
        match (first.next(), second.next()) {
            (None, None) => break,
            (None, Some(y)) => merged.push(y),
            (Some(x), None) => merged.push(x),
            (Some(x), Some(y)) => {
                merged.push(x);
                merged.push(y)
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately() {
        assert_eq!(&merge_alternately("abc", "pqr"), "apbqcr");
        assert_eq!(&merge_alternately("ab", "pqrs"), "apbqrs");
        assert_eq!(&merge_alternately("abcd", "pq"), "apbqcd");
    }
}
