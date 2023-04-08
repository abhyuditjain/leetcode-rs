//! 269. Alien Dictionary
//!
//! Hard
//!
//! There is a new alien language that uses the English alphabet. However, the order among the letters is unknown to you.
//! You are given a list of strings words from the alien language's dictionary, where the strings in words are sorted lexicographically by the rules of this new language.
//! Return a string of the unique letters in the new alien language sorted in lexicographically increasing order by the new language's rules. If there is no solution, return "". If there are multiple solutions, return any of them.
//!
//! Example 1:
//! Input: words = ["wrt","wrf","er","ett","rftt"]
//! Output: "wertf"
//!
//! Example 2:
//! Input: words = ["z","x"]
//! Output: "zx"
//!
//! Example 3:
//! Input: words = ["z","x","z"]
//! Output: ""
//! Explanation: The order is invalid, so return "".
//!
//! Constraints:
//! 1 <= words.length <= 100
//! 1 <= words[i].length <= 100
//! words[i] consists of only lowercase English letters.

use std::collections::{HashMap, VecDeque};

type Graph = HashMap<char, Vec<char>>;
type Indegrees = HashMap<char, usize>;

pub enum Algorithm {
    TopologicalSort,
    Dfs,
}

pub fn alien_order(words: &[String], alg: Algorithm) -> Option<String> {
    match alg {
        Algorithm::TopologicalSort => alien_order_topological_sort(words),
        Algorithm::Dfs => alien_order_dfs(words),
    }
}

fn alien_order_dfs(words: &[String]) -> Option<String> {
    fn dfs(
        start: char,
        graph: &HashMap<char, Vec<char>>,
        visited: &mut HashMap<char, bool>,
        order: &mut String,
    ) -> bool {
        match visited.get(&start) {
            Some(&result) => result,
            None => {
                visited.insert(start, false);
                for next in graph.get(&start).unwrap() {
                    if !dfs(*next, graph, visited, order) {
                        return false;
                    }
                }

                visited.insert(start, true);
                order.push(start);
                true
            }
        }
    }

    let graph = match build_reverse_graph(words) {
        Some(g) => g,
        None => return None,
    };
    let mut visited = HashMap::new();
    let mut order = "".to_string();

    for k in graph.keys() {
        if !dfs(*k, &graph, &mut visited, &mut order) {
            return None;
        }
    }

    Some(order)
}

fn alien_order_topological_sort(words: &[String]) -> Option<String> {
    let (graph, mut indegrees) = match build_graph_and_indegrees(words) {
        Some((g, i)) => (g, i),
        None => return None,
    };

    let mut q = indegrees
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some(*k) } else { None })
        .collect::<VecDeque<_>>();

    let mut order = String::with_capacity(indegrees.len());

    while let Some(c) = q.pop_front() {
        order.push(c);

        for next in graph.get(&c).unwrap() {
            indegrees.entry(*next).and_modify(|x| {
                if *x == 1 {
                    q.push_back(*next);
                }
                *x -= 1
            });
        }
    }

    if order.len() != indegrees.len() {
        return None;
    }

    Some(order)
}

fn build_graph_and_indegrees(words: &[String]) -> Option<(Graph, Indegrees)> {
    let mut adj = HashMap::new();
    let mut indegrees = HashMap::new();

    for word in words {
        for c in word.chars() {
            adj.insert(c, vec![]);
            indegrees.insert(c, 0);
        }
    }

    for i in 0..words.len() - 1 {
        let (w1, w2) = (&words[i], &words[i + 1]);

        if w1.len() > w2.len() && w1.starts_with(w2) {
            return None;
        }

        if let Some((c1, c2)) = w1.chars().zip(w2.chars()).find(|(c1, c2)| c1 != c2) {
            adj.get_mut(&c1).unwrap().push(c2);
            indegrees.entry(c2).and_modify(|x| *x += 1);
        }
    }

    Some((adj, indegrees))
}

fn build_reverse_graph(words: &[String]) -> Option<Graph> {
    let mut adj = HashMap::new();

    for word in words {
        for c in word.chars() {
            adj.insert(c, vec![]);
        }
    }

    for i in 0..words.len() - 1 {
        let (w1, w2) = (&words[i], &words[i + 1]);

        if w1.len() > w2.len() && w1.starts_with(w2) {
            return None;
        }

        if let Some((c1, c2)) = w1.chars().zip(w2.chars()).find(|(c1, c2)| c1 != c2) {
            adj.get_mut(&c2).unwrap().push(c1);
        }
    }

    Some(adj)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alien_order_topological_sort_1() {
        let words = &[
            "wrt".to_string(),
            "wrf".to_string(),
            "er".to_string(),
            "ett".to_string(),
            "rftt".to_string(),
        ];

        assert_eq!(
            alien_order_topological_sort(words),
            Some("wertf".to_string())
        );
    }

    #[test]
    fn test_alien_order_topological_sort_2() {
        let words = &["z".to_string(), "x".to_string()];

        assert_eq!(alien_order_topological_sort(words), Some("zx".to_string()));
    }

    #[test]
    fn test_alien_order_topological_sort_3() {
        let words = &["z".to_string(), "x".to_string(), "z".to_string()];

        assert_eq!(alien_order_topological_sort(words), None);
    }

    #[test]
    fn test_alien_order_topological_sort_4() {
        let words = &["zz".to_string(), "z".to_string()];

        assert_eq!(alien_order_topological_sort(words), None);
    }

    #[test]
    fn test_alien_order_dfs_1() {
        let words = &[
            "wrt".to_string(),
            "wrf".to_string(),
            "er".to_string(),
            "ett".to_string(),
            "rftt".to_string(),
        ];

        assert_eq!(alien_order_dfs(words), Some("wertf".to_string()));
    }

    #[test]
    fn test_alien_order_dfs_2() {
        let words = &["z".to_string(), "x".to_string()];

        assert_eq!(alien_order_dfs(words), Some("zx".to_string()));
    }

    #[test]
    fn test_alien_order_dfs_3() {
        let words = &["z".to_string(), "x".to_string(), "z".to_string()];

        assert_eq!(alien_order_dfs(words), None);
    }

    #[test]
    fn test_alien_order_dfs_4() {
        let words = &["zz".to_string(), "z".to_string()];

        assert_eq!(alien_order_dfs(words), None);
    }
}
