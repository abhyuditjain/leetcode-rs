//! 68. Text Justification
//!
//! Hard
//!
//! Given an array of strings words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.
//! You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.
//! Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line does not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
//! For the last line of text, it should be left-justified, and no extra space is inserted between words.
//! Note:
//! A word is defined as a character sequence consisting of non-space characters only.
//! Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
//! The input array words contains at least one word.
//!
//! Example 1:
//! Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
//! Output:
//! [
//!    "This    is    an",
//!    "example  of text",
//!    "justification.  "
//! ]
//!
//! Example 2:
//! Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
//! Output:
//! [
//!   "What   must   be",
//!   "acknowledgment  ",
//!   "shall be        "
//! ]
//! Explanation: Note that the last line is "shall be    " instead of "shall     be", because the last line must be left-justified instead of fully-justified.
//! Note that the second line is also left-justified because it contains only one word.
//!
//! Example 3:
//! Input: words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
//! Output:
//! [
//!   "Science  is  what we",
//!   "understand      well",
//!   "enough to explain to",
//!   "a  computer.  Art is",
//!   "everything  else  we",
//!   "do                  "
//! ]
//!
//! Constraints:
//! 1 <= words.length <= 300
//! 1 <= words[i].length <= 20
//! words[i] consists of only English letters and symbols.
//! 1 <= maxWidth <= 100
//! words[i].length <= maxWidth

pub fn justify_words(words: Vec<String>, max_width: usize) -> Vec<String> {
    let mut justified = vec![];
    let mut start = 0;

    while start < words.len() {
        let end = find_end(&words, start, max_width);
        justified.push(justify(&words, max_width, start, end));
        start = end + 1;
    }

    justified
}

fn justify(words: &[String], max_width: usize, start: usize, end: usize) -> String {
    if start == end {
        return pad_with_spaces(&words[start], max_width);
    }

    let is_last_line = end == words.len() - 1;

    let num_spaces_present_in_line = end - start;
    let total_spaces = max_width - len_words(words, start, end);

    let space_str_to_add = if is_last_line {
        " ".to_string()
    } else {
        " ".repeat(total_spaces / num_spaces_present_in_line)
    };

    let mut remainder = if is_last_line {
        0
    } else {
        total_spaces % num_spaces_present_in_line
    };

    let mut result = String::new();

    (start..=end).for_each(|i| {
        result.push_str(&words[i]);
        result.push_str(&space_str_to_add);

        if remainder > 0 {
            result.push(' ');
            remainder -= 1;
        }
    });

    pad_with_spaces(result.trim(), max_width)
}

/// This function returns the index of the last word which can fit in the current line's max width
/// Basically, it counts all the words that can fit into current line (with spaces included)
fn find_end(words: &[String], start: usize, max_width: usize) -> usize {
    assert!(start < words.len());

    let mut total_len = words[start].len();

    let mut end = start + 1;

    while end < words.len() {
        let word_len = words[end].len();
        total_len += 1 + word_len;

        if total_len > max_width {
            break;
        }

        end += 1;
    }

    end - 1
}

fn len_words(words: &[String], start: usize, end: usize) -> usize {
    assert!(start <= end);

    words
        .iter()
        .skip(start)
        .take(end - start + 1)
        .map(|x| x.len())
        .sum()
}

fn pad_with_spaces(line: &str, width: usize) -> String {
    // assert!(width >= line.len());

    let mut line = line.to_string();
    line.push_str(" ".repeat(width.saturating_sub(line.len())).as_str());
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_justify_words_1() {
        assert_eq!(
            justify_words(
                vec![
                    "This".to_string(),
                    "is".to_string(),
                    "an".to_string(),
                    "example".to_string(),
                    "of".to_string(),
                    "text".to_string(),
                    "justification.".to_string(),
                ],
                16
            ),
            vec![
                "This    is    an".to_string(),
                "example  of text".to_string(),
                "justification.  ".to_string(),
            ]
        )
    }

    #[test]
    fn test_justify_words_2() {
        assert_eq!(
            justify_words(
                vec![
                    "What".to_string(),
                    "must".to_string(),
                    "be".to_string(),
                    "acknowledgment".to_string(),
                    "shall".to_string(),
                    "be".to_string(),
                ],
                16
            ),
            vec![
                "What   must   be".to_string(),
                "acknowledgment  ".to_string(),
                "shall be        ".to_string(),
            ]
        )
    }

    #[test]
    fn test_justify_words_3() {
        assert_eq!(
            justify_words(
                vec![
                    "Science".to_string(),
                    "is".to_string(),
                    "what".to_string(),
                    "we".to_string(),
                    "understand".to_string(),
                    "well".to_string(),
                    "enough".to_string(),
                    "to".to_string(),
                    "explain".to_string(),
                    "to".to_string(),
                    "a".to_string(),
                    "computer.".to_string(),
                    "Art".to_string(),
                    "is".to_string(),
                    "everything".to_string(),
                    "else".to_string(),
                    "we".to_string(),
                    "do".to_string(),
                ],
                20
            ),
            vec![
                "Science  is  what we".to_string(),
                "understand      well".to_string(),
                "enough to explain to".to_string(),
                "a  computer.  Art is".to_string(),
                "everything  else  we".to_string(),
                "do                  ".to_string(),
            ]
        )
    }

    #[test]
    fn test_find_end() {
        let words = &[
            "a".to_string(),
            "bb".to_string(),
            "ccc".to_string(),
            "dddd".to_string(),
            "eeeee".to_string(),
            "ffffff".to_string(),
        ];

        assert_eq!(find_end(words, 0, 1), 0); // "a"
        assert_eq!(find_end(words, 0, 3), 0); // "a"
        assert_eq!(find_end(words, 0, 4), 1); // "a" + " " + "bb"
        assert_eq!(find_end(words, 1, 4), 1); // "bb"
        assert_eq!(find_end(words, 1, 5), 1); // "bb"
        assert_eq!(find_end(words, 1, 6), 2); // "bb" + " " + "ccc"
        assert_eq!(find_end(words, 1, 11), 3); // "bb" + " " + "ccc" + " " + "dddd"
    }

    #[test]
    fn test_len_words() {
        let words = &[
            "a".to_string(),
            "bb".to_string(),
            "ccc".to_string(),
            "dddd".to_string(),
            "eeeee".to_string(),
            "ffffff".to_string(),
        ];

        assert_eq!(len_words(words, 0, 0), 1);
        assert_eq!(len_words(words, 0, 1), 3);
        assert_eq!(len_words(words, 1, 1), 2);
        assert_eq!(len_words(words, 0, 5), 21);
    }

    #[test]
    fn test_pad_with_spaces() {
        assert_eq!(pad_with_spaces("a", 1), "a");
        assert_eq!(pad_with_spaces("a", 2), "a ");
        assert_eq!(pad_with_spaces("a", 3), "a  ");
    }
}
