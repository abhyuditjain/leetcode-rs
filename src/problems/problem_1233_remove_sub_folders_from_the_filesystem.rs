//! 1233. Remove Sub-Folders from the Filesystem
//!
//! Medium
//!
//! Given a list of folders folder, return the folders after removing all sub-folders in those folders. You may return the answer in any order.
//! If a folder[i] is located within another folder[j], it is called a sub-folder of it.
//! The format of a path is one or more concatenated strings of the form: '/' followed by one or more lowercase English letters.
//! For example, "/leetcode" and "/leetcode/problems" are valid paths while an empty string and "/" are not.
//!
//! Example 1:
//! Input: folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
//! Output: ["/a","/c/d","/c/f"]
//! Explanation: Folders "/a/b" is a subfolder of "/a" and "/c/d/e" is inside of folder "/c/d" in our filesystem.
//!
//! Example 2:
//! Input: folder = ["/a","/a/b/c","/a/b/d"]
//! Output: ["/a"]
//! Explanation: Folders "/a/b/c" and "/a/b/d" will be removed because they are subfolders of "/a".
//!
//! Example 3:
//! Input: folder = ["/a/b/c","/a/b/ca","/a/b/d"]
//! Output: ["/a/b/c","/a/b/ca","/a/b/d"]
//!
//! Constraints:
//! 1 <= folder.length <= 4 * 10^4
//! 2 <= folder[i].length <= 100
//! folder[i] contains only lowercase letters and '/'.
//! folder[i] always starts with the character '/'.
//! Each folder name is unique.

use std::collections::HashMap;

#[derive(Debug, Default)]
struct Trie {
    children: HashMap<String, Trie>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, path: String) {
        let mut curr = self;

        for folder in path.split('/').filter(|x| !x.is_empty()) {
            curr = curr.children.entry(folder.to_string()).or_default();
        }

        curr.is_end = true;
    }

    fn get_parent_folders(&self) -> Vec<String> {
        let mut folders = vec![];
        let mut path = String::new();

        Self::dfs(self, &mut folders, &mut path);

        folders
    }

    fn dfs(node: &Self, folders: &mut Vec<String>, path: &mut String) {
        if node.is_end {
            folders.push(path.clone());
            return;
        }

        for (k, v) in &node.children {
            let folder = format!("/{}", k);
            let size = path.len();
            path.push_str(&folder);
            Self::dfs(v, folders, path);
            path.truncate(size);
        }
    }
}

pub fn remove_subfolders(folders: Vec<String>) -> Vec<String> {
    let mut trie = Trie::new();

    for folder in folders {
        trie.insert(folder);
    }

    trie.get_parent_folders()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(mut output: Vec<String>, mut correct: Vec<String>) {
        output.sort();
        correct.sort();

        assert_eq!(output, correct);
    }

    #[test]
    fn test_remove_subfolders_1() {
        check(
            remove_subfolders(vec![
                "/a".to_string(),
                "/a/b".to_string(),
                "/c/d".to_string(),
                "/c/d/e".to_string(),
                "/c/f".to_string(),
            ]),
            vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()],
        );
    }

    #[test]
    fn test_remove_subfolders_2() {
        check(
            remove_subfolders(vec![
                "/a".to_string(),
                "/a/b/c".to_string(),
                "/a/b/d".to_string(),
            ]),
            vec!["/a".to_string()],
        );
    }

    #[test]
    fn test_remove_subfolders_3() {
        check(
            remove_subfolders(vec![
                "/a/b/c".to_string(),
                "/a/b/ca".to_string(),
                "/a/b/d".to_string(),
            ]),
            vec![
                "/a/b/c".to_string(),
                "/a/b/ca".to_string(),
                "/a/b/d".to_string(),
            ],
        );
    }
}
