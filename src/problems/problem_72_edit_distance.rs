//! 72. Edit Distance
//!
//! Hard
//!
//! Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
//! You have the following three operations permitted on a word:
//!     - Insert a character
//!     - Delete a character
//!     - Replace a character
//!
//! Example 1:
//! Input: word1 = "horse", word2 = "ros"
//! Output: 3
//! Explanation:
//! horse -> rorse (replace 'h' with 'r')
//! rorse -> rose (remove 'r')
//! rose -> ros (remove 'e')
//!
//! Example 2:
//! Input: word1 = "intention", word2 = "execution"
//! Output: 5
//! Explanation:
//! intention -> inention (remove 't')
//! inention -> enention (replace 'i' with 'e')
//! enention -> exention (replace 'n' with 'x')
//! exention -> exection (replace 'n' with 'c')
//! exection -> execution (insert 'u')
//!
//! Constraints:
//! 0 <= word1.length, word2.length <= 500
//! word1 and word2 consist of lowercase English letters.

pub enum Algorithm {
    Recursion,
    Memoization,
    Tabulation,
}

pub fn edit_distance(word1: &str, word2: &str, alg: Algorithm) -> usize {
    match alg {
        Algorithm::Recursion => edit_distance_rec(word1.as_bytes(), word2.as_bytes()),
        Algorithm::Memoization => edit_distance_mem(word1.as_bytes(), word2.as_bytes()),
        Algorithm::Tabulation => edit_distance_tab(word1.as_bytes(), word2.as_bytes()),
    }
}

fn edit_distance_tab(word1: &[u8], word2: &[u8]) -> usize {
    let mut cache = vec![vec![0; word2.len() + 1]; word1.len() + 1];
    edit_distance_tab_helper(word1, word2, &mut cache)
}

fn edit_distance_mem(word1: &[u8], word2: &[u8]) -> usize {
    let mut cache = vec![vec![None; word2.len() + 1]; word1.len() + 1];
    edit_distance_mem_helper(word1, 0, word2, 0, &mut cache)
}

fn edit_distance_rec(word1: &[u8], word2: &[u8]) -> usize {
    edit_distance_rec_helper(word1, 0, word2, 0)
}

fn edit_distance_tab_helper(word1: &[u8], word2: &[u8], cache: &mut [Vec<usize>]) -> usize {
    (0..=word1.len()).for_each(|i| {
        cache[i][word2.len()] = word1.len() - i;
    });

    (0..=word2.len()).for_each(|i| {
        cache[word1.len()][i] = word2.len() - i;
    });

    for i in (0..word1.len()).rev() {
        for j in (0..word2.len()).rev() {
            if word1[i] == word2[j] {
                cache[i][j] = cache[i + 1][j + 1];
            } else {
                let insert_steps = cache[i][j + 1];
                let delete_steps = cache[i + 1][j];
                let replace_steps = cache[i + 1][j + 1];

                cache[i][j] = 1 + insert_steps.min(delete_steps.min(replace_steps));
            }
        }
    }

    cache[0][0]
}

fn edit_distance_mem_helper(
    word1: &[u8],
    i: usize,
    word2: &[u8],
    j: usize,
    cache: &mut [Vec<Option<usize>>],
) -> usize {
    if i >= word1.len() && j >= word2.len() {
        return 0;
    }

    if i >= word1.len() {
        return word2.len() - j;
    }

    if j >= word2.len() {
        return word1.len() - i;
    }

    if let Some(cached_steps) = cache[i][j] {
        return cached_steps;
    }

    if word1[i] == word2[j] {
        let steps = edit_distance_mem_helper(word1, i + 1, word2, j + 1, cache);
        cache[i][j] = Some(steps);
        return steps;
    }

    let insert_steps = edit_distance_mem_helper(word1, i, word2, j + 1, cache);
    let delete_steps = edit_distance_mem_helper(word1, i + 1, word2, j, cache);
    let replace_steps = edit_distance_mem_helper(word1, i + 1, word2, j + 1, cache);

    cache[i][j] = Some(1 + insert_steps.min(delete_steps.min(replace_steps)));

    cache[i][j].unwrap()
}

fn edit_distance_rec_helper(word1: &[u8], i: usize, word2: &[u8], j: usize) -> usize {
    if i >= word1.len() && j >= word2.len() {
        return 0;
    }

    if i >= word1.len() {
        return word2.len() - j;
    }

    if j >= word2.len() {
        return word1.len() - i;
    }

    if word1[i] == word2[j] {
        return edit_distance_rec_helper(word1, i + 1, word2, j + 1);
    }

    let insert_steps = edit_distance_rec_helper(word1, i, word2, j + 1);
    let delete_steps = edit_distance_rec_helper(word1, i + 1, word2, j);
    let replace_steps = edit_distance_rec_helper(word1, i + 1, word2, j + 1);

    1 + insert_steps.min(delete_steps.min(replace_steps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance_rec() {
        assert_eq!(edit_distance_rec("horse".as_bytes(), "ros".as_bytes()), 3);
        assert_eq!(
            edit_distance_rec("intention".as_bytes(), "execution".as_bytes()),
            5
        );
        assert_eq!(edit_distance_rec("horse".as_bytes(), "".as_bytes()), 5);
        assert_eq!(edit_distance_rec("".as_bytes(), "horse".as_bytes()), 5);
    }

    #[test]
    fn test_edit_distance_mem() {
        assert_eq!(edit_distance_mem("horse".as_bytes(), "ros".as_bytes()), 3);
        assert_eq!(
            edit_distance_mem("intention".as_bytes(), "execution".as_bytes()),
            5
        );
        assert_eq!(edit_distance_mem("horse".as_bytes(), "".as_bytes()), 5);
        assert_eq!(edit_distance_mem("".as_bytes(), "horse".as_bytes()), 5);
    }

    #[test]
    fn test_edit_distance_tab() {
        assert_eq!(edit_distance_tab("horse".as_bytes(), "ros".as_bytes()), 3);
        assert_eq!(
            edit_distance_tab("intention".as_bytes(), "execution".as_bytes()),
            5
        );
        assert_eq!(edit_distance_tab("horse".as_bytes(), "".as_bytes()), 5);
        assert_eq!(edit_distance_tab("".as_bytes(), "horse".as_bytes()), 5);
    }
}
