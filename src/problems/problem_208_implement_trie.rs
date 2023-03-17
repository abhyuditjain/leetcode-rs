//! 208. Implement Trie (Prefix Tree)
//!
//! Medium
//!
//! A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings.
//! There are various applications of this data structure, such as autocomplete and spellchecker.
//!
//! Implement the Trie class:
//! Trie() Initializes the trie object.
//! void insert(String word) Inserts the string word into the trie.
//! boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
//! boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.
//!
//! Example 1:
//! Input:
//! ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
//! [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
//! Output:
//! [null, null, true, false, true, null, true]
//! Explanation:
//! Trie trie = new Trie();
//! trie.insert("apple");
//! trie.search("apple");   // return True
//! trie.search("app");     // return False
//! trie.startsWith("app"); // return True
//! trie.insert("app");
//! trie.search("app");     // return True
//!
//! Constraints:
//! 1 <= word.length, prefix.length <= 2000
//! word and prefix consist only of lowercase English letters.
//! At most 3 * 104 calls in total will be made to insert, search, and startsWith.

#[derive(Debug, Default)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr = self;
        for c in word.chars().map(|c| c as usize - 'a' as usize) {
            curr = curr.children[c].get_or_insert_with(|| Box::new(Trie::new()));
        }

        curr.is_word_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut curr = self;
        for c in word.chars().map(|c| c as usize - 'a' as usize) {
            match curr.children[c].as_ref() {
                Some(child) => curr = child,
                None => return false,
            }
        }

        curr.is_word_end
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut curr = self;
        for c in prefix.chars().map(|c| c as usize - 'a' as usize) {
            match curr.children[c].as_ref() {
                Some(child) => curr = child,
                None => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();

        trie.insert("apple");
        assert!(trie.search("apple"));
        assert!(!trie.search("app"));
        assert!(trie.starts_with("app"));

        trie.insert("app");
        assert!(trie.search("app"));
    }
}
