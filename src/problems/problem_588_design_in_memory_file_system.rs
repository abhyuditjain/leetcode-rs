//! 588. Design In-Memory File System
//!
//! Hard
//!
//! Design a data structure that simulates an in-memory file system.
//! Implement the FileSystem class:
//! FileSystem() Initializes the object of the system.
//! List<String> ls(String path)
//! If path is a file path, returns a list that only contains this file's name.
//! If path is a directory path, returns the list of file and directory names in this directory.
//! The answer should in lexicographic order.
//! void mkdir(String path) Makes a new directory according to the given path. The given directory path does not exist. If the middle directories in the path do not exist, you should create them as well.
//! void addContentToFile(String filePath, String content)
//! If filePath does not exist, creates that file containing given content.
//! If filePath already exists, appends the given content to original content.
//! String readContentFromFile(String filePath) Returns the content in the file at filePath.
//!
//! Example 1:
//! Input
//! ["FileSystem", "ls", "mkdir", "addContentToFile", "ls", "readContentFromFile"]
//! [[], ["/"], ["/a/b/c"], ["/a/b/c/d", "hello"], ["/"], ["/a/b/c/d"]]
//! Output
//! [null, [], null, null, ["a"], "hello"]
//! Explanation
//! FileSystem fileSystem = new FileSystem();
//! fileSystem.ls("/");                         // return []
//! fileSystem.mkdir("/a/b/c");
//! fileSystem.addContentToFile("/a/b/c/d", "hello");
//! fileSystem.ls("/");                         // return ["a"]
//! fileSystem.readContentFromFile("/a/b/c/d"); // return "hello"
//!
//! Constraints:
//! 1 <= path.length, filePath.length <= 100
//! path and filePath are absolute paths which begin with '/' and do not end with '/' except that the path is just "/".
//! You can assume that all directory names and file names only contain lowercase letters, and the same names will not exist in the same directory.
//! You can assume that all operations will be passed valid parameters, and users will not attempt to retrieve file content or list a directory or file that does not exist.
//! 1 <= content.length <= 50
//! At most 300 calls will be made to ls, mkdir, addContentToFile, and readContentFromFile.

use std::collections::BTreeMap;

#[derive(Default)]
struct Trie {
    value: Option<String>,
    content: String,
    children: BTreeMap<String, Trie>,
    is_file: bool,
}

impl Trie {
    fn new(value: Option<String>, is_file: bool) -> Self {
        Self {
            value,
            content: String::new(),
            children: BTreeMap::new(),
            is_file,
        }
    }
}

#[derive(Default)]
pub struct FileSystem {
    data: Trie,
}

impl FileSystem {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ls(&self, path: String) -> Vec<String> {
        let tokens = path.split('/').filter(|x| !x.is_empty());
        let mut current_node = &self.data;

        for token in tokens {
            current_node = current_node.children.get(token).unwrap();
        }

        if current_node.is_file {
            return vec![current_node.value.clone().unwrap()];
        }

        if current_node.children.is_empty() {
            return vec![];
        }

        current_node.children.keys().cloned().collect()
    }

    pub fn mkdir(&mut self, path: String) {
        let tokens = path.split('/').filter(|x| !x.is_empty());
        let mut current_node = &mut self.data;

        for token in tokens {
            current_node = current_node
                .children
                .entry(token.to_string())
                .or_insert(Trie::new(Some(token.to_string()), false));
        }
    }

    pub fn add_content_to_file(&mut self, file_path: String, content: String) {
        let tokens = file_path.split('/').filter(|x| !x.is_empty());
        let mut current_node = &mut self.data;

        for token in tokens {
            current_node = current_node
                .children
                .entry(token.to_string())
                .or_insert(Trie::new(Some(token.to_string()), true));
        }

        current_node.content.push_str(&content);
    }

    pub fn read_content_from_file(&self, file_path: String) -> String {
        let tokens = file_path.split('/').filter(|x| !x.is_empty());
        let mut current_node = &self.data;

        for token in tokens {
            current_node = current_node.children.get(token).unwrap();
        }

        current_node.content.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_system() {
        let mut fs = FileSystem::new();

        assert_eq!(fs.ls("/".to_string()), vec!["".to_string(); 0]);

        fs.mkdir("/a/b/c".to_string());
        fs.add_content_to_file("/a/b/c/d".to_string(), "hello".to_string());

        assert_eq!(fs.ls("/".to_string()), vec!["a".to_string()]);

        assert_eq!(
            fs.read_content_from_file("/a/b/c/d".to_string()),
            "hello".to_string()
        );
    }
}
