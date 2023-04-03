//! 676. Implement Magic Dictionary
//!
//! Medium
//!
//! Design a data structure that is initialized with a list of different words. Provided a string, you should determine if you can change exactly one character in this string to match any word in the data structure.
//!
//! Implement the MagicDictionary class:
//! MagicDictionary() Initializes the object.
//! void buildDict(String[] dictionary) Sets the data structure with an array of distinct strings dictionary.
//! bool search(String searchWord) Returns true if you can change exactly one character in searchWord to match any string in the data structure, otherwise returns false.
//!
//! Example 1:
//! Input
//! ["MagicDictionary", "buildDict", "search", "search", "search", "search"]
//! [[], [["hello", "leetcode"]], ["hello"], ["hhllo"], ["hell"], ["leetcoded"]]
//! Output
//! [null, null, false, true, false, false]
//! Explanation
//! MagicDictionary magicDictionary = new MagicDictionary();
//! magicDictionary.buildDict(["hello", "leetcode"]);
//! magicDictionary.search("hello"); // return False
//! magicDictionary.search("hhllo"); // We can change the second 'h' to 'e' to match "hello" so we return True
//! magicDictionary.search("hell"); // return False
//! magicDictionary.search("leetcoded"); // return False
//!
//! Constraints:
//! 1 <= dictionary.length <= 100
//! 1 <= dictionary[i].length <= 100
//! dictionary[i] consists of only lower-case English letters.
//! All the strings in dictionary are distinct.
//! 1 <= searchWord.length <= 100
//! searchWord consists of only lower-case English letters.
//! buildDict will be called only once before search.
//! At most 100 calls will be made to search.

use crate::utils::trie::Trie;

#[derive(Default)]
pub struct MagicDictionary {
    trie: Trie<char>,
}

impl MagicDictionary {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_dict(&mut self, dictionary: &[String]) {
        self.trie = Trie::from_iter(dictionary.iter().map(|s| s.chars()));
    }

    pub fn search(&self, search_word: &str) -> bool {
        self.trie
            .contains_with_deviations(&search_word.chars().collect::<Vec<_>>(), 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_dictionary_1() {
        let mut dict = MagicDictionary::new();

        dict.build_dict(&["hello".to_string(), "leetcode".to_string()]);

        assert!(!dict.search("hello"));
        assert!(!dict.search("hell"));
        assert!(!dict.search("leetcoded"));
        assert!(dict.search("hhllo"));
    }

    #[test]
    fn test_magic_dictionary_2() {
        let mut dict = MagicDictionary::new();

        dict.build_dict(&[
            "hello".to_string(),
            "hallo".to_string(),
            "leetcode".to_string(),
        ]);

        assert!(dict.search("hello"));
        assert!(dict.search("hhllo"));
        assert!(!dict.search("hell"));
        assert!(!dict.search("leetcoded"));
    }
}
