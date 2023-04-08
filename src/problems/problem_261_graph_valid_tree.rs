//! 261. Graph Valid Tree
//!
//! Medium
//!
//! You have a graph of n nodes labeled from 0 to n - 1. You are given an integer n and a list of edges where edges[i] = [ai, bi] indicates that there is an undirected edge between nodes ai and bi in the graph.
//! Return true if the edges of the given graph make up a valid tree, and false otherwise.
//!
//! Example 1:
//! Input: n = 5, edges = [[0,1],[0,2],[0,3],[1,4]]
//! Output: true
//!
//! Example 2:
//! Input: n = 5, edges = [[0,1],[1,2],[2,3],[1,3],[1,4]]
//! Output: false
//!
//! Constraints:
//! 1 <= n <= 2000
//! 0 <= edges.length <= 5000
//! edges[i].length == 2
//! 0 <= ai, bi < n
//! ai != bi
//! There are no self-loops or repeated edges.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::union_find::UnionFind;

pub enum Algorithm {
    IterativeDfsCycleDetection,
    RecursiveDfsCycleDetection,
    IterativeBfsCycleDetection,
    IterativeDfsCheckFullyConnected,
    RecursiveDfsCheckFullyConnected,
    IterativeBfsCheckFullyConnected,
    DSU,
}

pub fn valid_tree(n: usize, edges: Vec<Vec<i32>>, alg: Algorithm) -> bool {
    let edges = edges
        .into_iter()
        .map(|edge| (edge[0] as usize, edge[1] as usize))
        .collect::<Vec<_>>();

    match alg {
        Algorithm::IterativeDfsCycleDetection => {
            is_valid_tree_iterative_dfs_cycle_detection(n, &edges)
        }
        Algorithm::RecursiveDfsCycleDetection => {
            is_valid_tree_recursive_dfs_cycle_detection(n, &edges)
        }
        Algorithm::IterativeBfsCycleDetection => {
            is_valid_tree_iterative_bfs_cycle_detection(n, &edges)
        }
        Algorithm::IterativeDfsCheckFullyConnected => {
            is_valid_tree_iterative_dfs_check_fully_connected(n, &edges)
        }
        Algorithm::RecursiveDfsCheckFullyConnected => {
            is_valid_tree_recursive_dfs_check_fully_connected(n, &edges)
        }
        Algorithm::IterativeBfsCheckFullyConnected => {
            is_valid_tree_iterative_bfs_check_fully_connected(n, &edges)
        }
        Algorithm::DSU => is_valid_tree_dsu(n, &edges),
    }
}

fn is_valid_tree_iterative_dfs_cycle_detection(n: usize, edges: &[(usize, usize)]) -> bool {
    let graph = build_graph(n, edges);

    let mut parent = HashMap::new();
    parent.insert(0, usize::MAX);

    let mut stack = Vec::with_capacity(n);
    stack.push(0);

    while let Some(node) = stack.pop() {
        for &neighbour in &graph[node] {
            if let Some(p) = parent.get(&node) {
                if p == &neighbour {
                    continue;
                }
            }

            if parent.contains_key(&neighbour) {
                return false;
            }

            stack.push(neighbour);
            parent.insert(neighbour, node);
        }
    }

    parent.len() == n
}

fn is_valid_tree_recursive_dfs_cycle_detection(n: usize, edges: &[(usize, usize)]) -> bool {
    fn dfs(graph: &[Vec<usize>], node: usize, parent: usize, visited: &mut HashSet<usize>) -> bool {
        if visited.contains(&node) {
            return false;
        }

        visited.insert(node);

        for &neighbour in &graph[node] {
            if parent != neighbour && !dfs(graph, neighbour, node, visited) {
                return false;
            }
        }

        true
    }

    let graph = build_graph(n, edges);

    let mut visited = HashSet::new();

    dfs(&graph, 0, usize::MAX, &mut visited) && visited.len() == n
}

fn is_valid_tree_iterative_bfs_cycle_detection(n: usize, edges: &[(usize, usize)]) -> bool {
    let graph = build_graph(n, edges);

    let mut parent = HashMap::new();
    parent.insert(0, usize::MAX);

    let mut q = VecDeque::with_capacity(n);
    q.push_back(0);

    while let Some(node) = q.pop_front() {
        for &neighbour in &graph[node] {
            if let Some(p) = parent.get(&node) {
                if p == &neighbour {
                    continue;
                }
            }

            if parent.contains_key(&neighbour) {
                return false;
            }

            q.push_back(neighbour);
            parent.insert(neighbour, node);
        }
    }

    parent.len() == n
}

fn is_valid_tree_iterative_dfs_check_fully_connected(n: usize, edges: &[(usize, usize)]) -> bool {
    if edges.len() != n - 1 {
        return false;
    }

    let graph = build_graph(n, edges);

    let mut stack = Vec::with_capacity(n);
    let mut visited = HashSet::new();

    stack.push(0);
    visited.insert(0);

    while let Some(node) = stack.pop() {
        for &neighbour in &graph[node] {
            if visited.contains(&neighbour) {
                continue;
            }

            stack.push(neighbour);
            visited.insert(neighbour);
        }
    }

    visited.len() == n
}

fn is_valid_tree_recursive_dfs_check_fully_connected(n: usize, edges: &[(usize, usize)]) -> bool {
    fn dfs(graph: &[Vec<usize>], node: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&node) {
            return;
        }

        visited.insert(node);

        for &neighbour in &graph[node] {
            dfs(graph, neighbour, visited);
        }
    }

    if edges.len() != n - 1 {
        return false;
    }

    let graph = build_graph(n, edges);
    let mut visited = HashSet::new();

    dfs(&graph, 0, &mut visited);

    visited.len() == n
}

fn is_valid_tree_iterative_bfs_check_fully_connected(n: usize, edges: &[(usize, usize)]) -> bool {
    if edges.len() != n - 1 {
        return false;
    }

    let graph = build_graph(n, edges);

    let mut q = VecDeque::with_capacity(n);
    let mut visited = HashSet::new();

    q.push_back(0);
    visited.insert(0);

    while let Some(node) = q.pop_front() {
        for &neighbour in &graph[node] {
            if visited.contains(&neighbour) {
                continue;
            }

            q.push_back(neighbour);
            visited.insert(neighbour);
        }
    }

    visited.len() == n
}

fn is_valid_tree_dsu(n: usize, edges: &[(usize, usize)]) -> bool {
    if edges.len() != n - 1 {
        return false;
    }

    let mut dsu = UnionFind::new(n);

    for &(a, b) in edges {
        if !dsu.union(a, b) {
            return false;
        }
    }

    true
}

fn build_graph(vertices: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; vertices];

    for &(a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod test_iterative_dfs_cycle_detection {
        use super::*;

        #[test]
        fn test_1() {
            assert!(is_valid_tree_iterative_dfs_cycle_detection(
                5,
                &[(0, 1), (0, 2), (0, 3), (1, 4)]
            ));
        }

        #[test]
        fn test_2() {
            assert!(!is_valid_tree_iterative_dfs_cycle_detection(
                5,
                &[(0, 1), (1, 2), (2, 3), (1, 3), (1, 4)]
            ));
        }
    }

    #[cfg(test)]
    mod test_recursive_dfs_cycle_detection {
        use super::*;

        #[test]
        fn test_1() {
            assert!(is_valid_tree_recursive_dfs_cycle_detection(
                5,
                &[(0, 1), (0, 2), (0, 3), (1, 4)]
            ));
        }

        #[test]
        fn test_2() {
            assert!(!is_valid_tree_recursive_dfs_cycle_detection(
                5,
                &[(0, 1), (1, 2), (2, 3), (1, 3), (1, 4)]
            ));
        }
    }

    #[cfg(test)]
    mod test_iterative_bfs_cycle_detection {
        use super::*;

        #[test]
        fn test_1() {
            assert!(is_valid_tree_iterative_bfs_cycle_detection(
                5,
                &[(0, 1), (0, 2), (0, 3), (1, 4)]
            ));
        }

        #[test]
        fn test_2() {
            assert!(!is_valid_tree_iterative_bfs_cycle_detection(
                5,
                &[(0, 1), (1, 2), (2, 3), (1, 3), (1, 4)]
            ));
        }
    }

    #[cfg(test)]
    mod test_iterative_dfs_check_fully_connected {
        use super::*;

        #[test]
        fn test_1() {
            assert!(is_valid_tree_iterative_dfs_check_fully_connected(
                5,
                &[(0, 1), (0, 2), (0, 3), (1, 4)]
            ));
        }

        #[test]
        fn test_2() {
            assert!(!is_valid_tree_iterative_dfs_check_fully_connected(
                5,
                &[(0, 1), (1, 2), (2, 3), (1, 3), (1, 4)]
            ));
        }
    }

    #[cfg(test)]
    mod test_recursive_dfs_check_fully_connected {
        use super::*;

        #[test]
        fn test_1() {
            assert!(is_valid_tree_recursive_dfs_check_fully_connected(
                5,
                &[(0, 1), (0, 2), (0, 3), (1, 4)]
            ));
        }

        #[test]
        fn test_2() {
            assert!(!is_valid_tree_recursive_dfs_check_fully_connected(
                5,
                &[(0, 1), (1, 2), (2, 3), (1, 3), (1, 4)]
            ));
        }
    }

    #[cfg(test)]
    mod test_iterative_bfs_check_fully_connected {
        use super::*;

        #[test]
        fn test_1() {
            assert!(is_valid_tree_iterative_bfs_check_fully_connected(
                5,
                &[(0, 1), (0, 2), (0, 3), (1, 4)]
            ));
        }

        #[test]
        fn test_2() {
            assert!(!is_valid_tree_iterative_bfs_check_fully_connected(
                5,
                &[(0, 1), (1, 2), (2, 3), (1, 3), (1, 4)]
            ));
        }
    }

    #[cfg(test)]
    mod test_dsu {
        use super::*;

        #[test]
        fn test_1() {
            assert!(is_valid_tree_dsu(5, &[(0, 1), (0, 2), (0, 3), (1, 4)]));
        }

        #[test]
        fn test_2() {
            assert!(!is_valid_tree_dsu(
                5,
                &[(0, 1), (1, 2), (2, 3), (1, 3), (1, 4)]
            ));
        }
    }
}
