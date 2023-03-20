//! 49. Group Anagrams
//!
//! Medium
//!
//! Given an array of strings strs, group the anagrams together. You can return the answer in any order.
//! An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
//! typically using all the original letters exactly once.
//!
//! Example 1:
//! Input: strs = ["eat","tea","tan","ate","nat","bat"]
//! Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//!
//! Example 2:
//! Input: strs = [""]
//! Output: [[""]]
//!
//! Example 3:
//! Input: strs = ["a"]
//! Output: [["a"]]
//!
//! Constraints:
//! 1 <= strs.length <= 104
//! 0 <= strs[i].length <= 100
//! strs[i] consists of lowercase English letters.

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for s in strs {
        let id = get_mask(&s);
        map.entry(id).or_insert(vec![]).push(s);
    }

    map.values().cloned().collect::<Vec<_>>()
}

fn get_mask(s: &str) -> [usize; 26] {
    let mut count = [0; 26];

    for c in s.chars() {
        count[c as usize - 'a' as usize] += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(mut output: Vec<Vec<String>>, mut correct: Vec<Vec<String>>) {
        output.iter_mut().for_each(|x| x.sort());
        output.sort();

        correct.iter_mut().for_each(|x| x.sort());
        correct.sort();

        assert_eq!(output, correct);
    }

    #[test]
    fn test_group_anagrams_1() {
        check(
            group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string(),
            ]),
            vec![
                vec!["tan".to_string(), "nat".to_string()],
                vec!["bat".to_string()],
                vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            ],
        )
    }

    #[test]
    fn test_group_anagrams_2() {
        check(
            group_anagrams(vec!["eat".to_string()]),
            vec![vec!["eat".to_string()]],
        )
    }

    #[test]
    fn test_group_anagrams_3() {
        check(
            group_anagrams(vec!["".to_string()]),
            vec![vec!["".to_string()]],
        )
    }
}
