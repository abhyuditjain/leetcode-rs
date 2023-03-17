//! 30. Substring with Concatenation of All Words
//!
//! Hard
//!
//! You are given a string s and an array of strings words. All the strings of words are of the same length.
//! A concatenated substring in s is a substring that contains all the strings of any permutation of words concatenated.
//!
//! For example, if words = ["ab","cd","ef"],
//! then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are all concatenated strings.
//! "acdbef" is not a concatenated substring because it is not the concatenation of any permutation of words.
//!
//! Return the starting indices of all the concatenated substrings in s. You can return the answer in any order.
//!
//! Example 1:
//! Input: s = "barfoothefoobarman", words = ["foo","bar"]
//! Output: [0,9]
//! Explanation: Since words.length == 2 and words[i].length == 3, the concatenated substring has to be of length 6.
//! The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
//! The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.
//! The output order does not matter. Returning [9,0] is fine too.
//!
//! Example 2:
//! Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
//! Output: []
//! Explanation: Since words.length == 4 and words[i].length == 4, the concatenated substring has to be of length 16.
//! There is no substring of length 16 is s that is equal to the concatenation of any permutation of words.
//! We return an empty array.
//!
//! Example 3:
//! Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
//! Output: [6,9,12]
//! Explanation: Since words.length == 3 and words[i].length == 3, the concatenated substring has to be of length 9.
//! The substring starting at 6 is "foobarthe". It is the concatenation of ["foo","bar","the"] which is a permutation of words.
//! The substring starting at 9 is "barthefoo". It is the concatenation of ["bar","the","foo"] which is a permutation of words.
//! The substring starting at 12 is "thefoobar". It is the concatenation of ["the","foo","bar"] which is a permutation of words.
//!
//! Constraints:
//! 1 <= s.length <= 104
//! 1 <= words.length <= 5000
//! 1 <= words[i].length <= 30
//! s and words[i] consist of lowercase English letters.

use std::collections::HashMap;

pub fn find_substrings(s: &str, words: &[&str]) -> Vec<usize> {
    assert!(!words.is_empty());

    let word_length = words[0].len();
    let word_count = words.len();

    if word_length * word_count > s.len() {
        return vec![];
    }

    let mut word_freq = HashMap::new();

    for word in words {
        *word_freq.entry(word).or_insert(0) += 1;
    }

    let mut res = vec![];

    for i in 0..=(s.len() - word_count * word_length) {
        let mut words_seen = HashMap::new();

        for j in 0..word_count {
            let word_start = i + j * word_length;

            let word = &s[word_start..word_start + word_length];

            if !word_freq.contains_key(&word) {
                break;
            }

            *words_seen.entry(word).or_insert(0) += 1;

            if words_seen.get(&word).unwrap() > word_freq.get(&word).unwrap() {
                break;
            }

            if j + 1 == word_count {
                res.push(i);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_substrings_1() {
        assert_eq!(
            find_substrings("barfoothefoobarman", &["foo", "bar"]),
            vec![0, 9]
        );
    }

    #[test]
    fn test_find_substrings_2() {
        assert_eq!(
            find_substrings(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "word"]
            ),
            vec![]
        );
    }

    #[test]
    fn test_find_substrings_3() {
        assert_eq!(
            find_substrings("barfoofoobarthefoobarman", &["bar", "foo", "the"]),
            vec![6, 9, 12]
        );
    }

    #[test]
    fn test_find_substrings_4() {
        assert_eq!(
            find_substrings(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "good"]
            ),
            vec![8]
        );
    }
}
