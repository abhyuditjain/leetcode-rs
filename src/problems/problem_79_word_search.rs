//! 79. Word Search
//!
//! Medium
//!
//! Given an m x n grid of characters board and a string word, return true if word exists in the grid.
//! The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring.
//! The same letter cell may not be used more than once.
//!
//! Example 1:
//! Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
//! Output: true
//!
//! Example 2:
//! Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
//! Output: true
//!
//! Example 3:
//! Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
//! Output: false
//!
//! Constraints:
//! m == board.length
//! n = board[i].length
//! 1 <= m, n <= 6
//! 1 <= word.length <= 15
//! board and word consists of only lowercase and uppercase English letters.
//!
//! Follow up: Could you use search pruning to make your solution faster with a larger board?

pub fn does_word_exist(board: Vec<Vec<char>>, word: &str) -> bool {
    assert!(!board.is_empty());

    let mut board = board;

    let rows = board.len();
    let cols = board[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if dfs(&mut board, word.as_bytes(), i, j, 0) {
                return true;
            }
        }
    }

    false
}

fn dfs(board: &mut Vec<Vec<char>>, word: &[u8], i: usize, j: usize, k: usize) -> bool {
    if board[i][j] != word[k] as char {
        return false;
    }

    if k == word.len() - 1 {
        return true;
    }

    let ch = board[i][j];

    board[i][j] = '#';

    let mut found = false;

    if i > 0 {
        found = found || dfs(board, word, i - 1, j, k + 1);
    }
    if j > 0 {
        found = found || dfs(board, word, i, j - 1, k + 1);
    }
    if i < board.len() - 1 {
        found = found || dfs(board, word, i + 1, j, k + 1);
    }
    if j < board[0].len() - 1 {
        found = found || dfs(board, word, i, j + 1, k + 1);
    }

    board[i][j] = ch;

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_does_word_exist_1() {
        assert!(does_word_exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED"
        ));
    }

    #[test]
    fn test_does_word_exist_2() {
        assert!(does_word_exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE"
        ));
    }

    #[test]
    fn test_does_word_exist_3() {
        assert!(!does_word_exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB"
        ));
    }

    #[test]
    fn test_does_word_exist_4() {
        assert!(does_word_exist(vec![vec!['A']], "A"));
    }
}
