//! 839. Similar String Groups
//!
//! Hard
//!
//! Two strings X and Y are similar if we can swap two letters (in different positions) of X, so that it equals Y.
//! Also two strings X and Y are similar if they are equal.
//! For example, "tars" and "rats" are similar (swapping at positions 0 and 2), and "rats" and "arts" are similar,
//! but "star" is not similar to "tars", "rats", or "arts".
//! Together, these form two connected groups by similarity: {"tars", "rats", "arts"} and {"star"}.
//! Notice that "tars" and "arts" are in the same group even though they are not similar.
//! Formally, each group is such that a word is in the group if and only if it is similar to at least one other word in the group.
//! We are given a list strs of strings where every string in strs is an anagram of every other string in strs.
//!
//! How many groups are there?
//!  
//! Example 1:
//! Input: strs = ["tars","rats","arts","star"]
//! Output: 2
//!
//! Example 2:
//! Input: strs = ["omv","ovm"]
//! Output: 1
//!  
//! Constraints:
//! 1 <= strs.length <= 300
//! 1 <= strs[i].length <= 300
//! strs[i] consists of lowercase letters only.
//! All words in strs have the same length and are anagrams of each other.

use std::collections::VecDeque;

use crate::utils::union_find::UnionFind;

pub enum Algorithm {
    Dfs,
    Bfs,
    DSU,
}

pub fn similar_groups(strs: &[String], alg: Algorithm) -> usize {
    match alg {
        Algorithm::Dfs => similar_groups_dfs(strs),
        Algorithm::Bfs => similar_groups_bfs(strs),
        Algorithm::DSU => similar_groups_dsu(strs),
    }
}

fn similar_groups_dfs(strs: &[String]) -> usize {
    fn dfs(node: usize, graph: &[Vec<usize>], visited: &mut [bool]) {
        visited[node] = true;

        for &neighbour in &graph[node] {
            if !visited[neighbour] {
                visited[neighbour] = true;
                dfs(neighbour, graph, visited);
            }
        }
    }

    let graph = build_graph(strs);
    let mut visited = vec![false; strs.len()];

    let mut count = 0;

    for i in 0..strs.len() {
        if !visited[i] {
            count += 1;
            dfs(i, &graph, &mut visited);
        }
    }

    count
}

fn similar_groups_bfs(strs: &[String]) -> usize {
    fn bfs(node: usize, graph: &[Vec<usize>], visited: &mut [bool]) {
        let mut q = VecDeque::new();
        q.push_back(node);
        visited[node] = true;

        while let Some(node) = q.pop_front() {
            for &neighbour in &graph[node] {
                if !visited[neighbour] {
                    visited[neighbour] = true;
                    q.push_back(neighbour);
                }
            }
        }
    }

    let graph = build_graph(strs);
    let mut visited = vec![false; strs.len()];

    let mut count = 0;

    for i in 0..strs.len() {
        if !visited[i] {
            count += 1;
            bfs(i, &graph, &mut visited);
        }
    }

    count
}

fn similar_groups_dsu(strs: &[String]) -> usize {
    let mut uf = UnionFind::new(strs.len());

    let mut count = strs.len();

    for i in 0..strs.len() {
        for j in i + 1..strs.len() {
            if is_similar(&strs[i], &strs[j]) && uf.union(i, j) {
                count -= 1;
            }
        }
    }

    count
}

/// Builds an adjacency list of indices for words which are similar
fn build_graph(words: &[String]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; words.len()];

    for i in 0..words.len() {
        for j in i + 1..words.len() {
            if is_similar(&words[i], &words[j]) {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    graph
}

fn is_similar(w1: &str, w2: &str) -> bool {
    let diff = w1
        .chars()
        .zip(w2.chars())
        .fold(0, |diff, (c1, c2)| match c1 == c2 {
            true => diff,
            false => diff + 1,
        });

    diff == 0 || diff == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similar_groups_dfs() {
        assert_eq!(
            similar_groups_dfs(&[
                "tars".to_string(),
                "rats".to_string(),
                "arts".to_string(),
                "star".to_string(),
            ]),
            2
        );

        assert_eq!(
            similar_groups_dfs(&["omv".to_string(), "ovm".to_string(),]),
            1
        );
    }

    #[test]
    fn test_similar_groups_bfs() {
        assert_eq!(
            similar_groups_bfs(&[
                "tars".to_string(),
                "rats".to_string(),
                "arts".to_string(),
                "star".to_string(),
            ]),
            2
        );

        assert_eq!(
            similar_groups_bfs(&["omv".to_string(), "ovm".to_string(),]),
            1
        );
    }

    #[test]
    fn test_similar_groups_dsu() {
        assert_eq!(
            similar_groups_dsu(&[
                "tars".to_string(),
                "rats".to_string(),
                "arts".to_string(),
                "star".to_string(),
            ]),
            2
        );

        assert_eq!(
            similar_groups_dsu(&["omv".to_string(), "ovm".to_string(),]),
            1
        );
    }
}
