//! 14. Longest Common Prefix
//!
//! Easy
//!
//! Write a function to find the longest common prefix string amongst an array of strings.
//! If there is no common prefix, return an empty string "".
//!
//! Example 1:
//! Input: strs = ["flower","flow","flight"]
//! Output: "fl"
//!
//! Example 2:
//! Input: strs = ["dog","racecar","car"]
//! Output: ""
//! Explanation: There is no common prefix among the input strings.
//!
//! Constraints:
//! 1 <= strs.length <= 200
//! 0 <= strs[i].length <= 200
//! strs[i] consists of only lowercase English letters.

pub fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return "".to_owned();
    }

    let strs = strs.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let min_len = strs.iter().map(|s| s.len()).min().unwrap();

    let mut prefix = "".to_string();

    for i in 0..min_len {
        let c = strs[0][i];

        for s in &strs {
            if s[i] != c {
                return prefix;
            }
        }
        prefix.push(c as char);
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix_1() {
        assert_eq!(
            longest_common_prefix(&["flower".to_owned(), "flow".to_owned(), "flight".to_owned()]),
            "fl".to_owned()
        );
    }

    #[test]
    fn test_longest_common_prefix_2() {
        assert_eq!(
            longest_common_prefix(&["dog".to_owned(), "racecar".to_owned(), "car".to_owned()]),
            "".to_owned()
        );
    }
}
