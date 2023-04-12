//! 684. Redundant Connection
//!
//! Medium
//!
//! In this problem, a tree is an undirected graph that is connected and has no cycles.
//! You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added.
//! The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed.
//! The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.
//! Return an edge that can be removed so that the resulting graph is a tree of n nodes.
//! If there are multiple answers, return the answer that occurs last in the input.
//!
//! Example 1:
//! Input: edges = [[1,2],[1,3],[2,3]]
//! Output: [2,3]
//!
//! Example 2:
//! Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
//! Output: [1,4]
//!
//! Constraints:
//! n == edges.length
//! 3 <= n <= 1000
//! edges[i].length == 2
//! 1 <= ai < bi <= edges.length
//! ai != bi
//! There are no repeated edges.
//! The given graph is connected.

use std::collections::HashSet;

use crate::utils::union_find::UnionFind;

const MAX_EDGE_VAL: usize = 1000;

type Edge = (usize, usize);

pub enum Algorithm {
    Dfs,
    DSU,
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>, alg: Algorithm) -> Vec<i32> {
    let edges = edges
        .into_iter()
        .map(|edge| (edge[0] as usize, edge[1] as usize))
        .collect::<Vec<_>>();

    let (a, b) = match alg {
        Algorithm::Dfs => redundant_connection_dfs(&edges),
        Algorithm::DSU => redundant_connection_dsu(&edges),
    };

    vec![a as i32, b as i32]
}

fn redundant_connection_dfs(edges: &[Edge]) -> Edge {
    fn dfs(
        graph: &[Vec<usize>],
        source: usize,
        target: usize,
        visited: &mut HashSet<usize>,
    ) -> bool {
        if visited.contains(&source) {
            return false;
        }

        visited.insert(source);

        if source == target {
            return true;
        }

        for &neighbour in &graph[source] {
            if dfs(graph, neighbour, target, visited) {
                return true;
            }
        }

        false
    }

    let mut graph = vec![vec![]; MAX_EDGE_VAL + 1];
    let mut visited = HashSet::new();

    for &(a, b) in edges {
        visited.clear();
        if !graph[a].is_empty() && !graph[b].is_empty() && dfs(&graph, a, b, &mut visited) {
            return (a, b);
        }

        graph[a].push(b);
        graph[b].push(a);
    }

    unreachable!()
}

fn redundant_connection_dsu(edges: &[Edge]) -> Edge {
    let mut uf = UnionFind::new(MAX_EDGE_VAL + 1);

    for &(a, b) in edges {
        if !uf.union(a, b) {
            return (a, b);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redundant_connection_dfs_1() {
        assert_eq!(redundant_connection_dfs(&[(1, 2), (1, 3), (2, 3)]), (2, 3));
    }

    #[test]
    fn test_redundant_connection_dfs_2() {
        assert_eq!(
            redundant_connection_dfs(&[(1, 2), (2, 3), (3, 4), (1, 4), (1, 5)]),
            (1, 4)
        );
    }

    #[test]
    fn test_redundant_connection_dsu_1() {
        assert_eq!(redundant_connection_dsu(&[(1, 2), (1, 3), (2, 3)]), (2, 3));
    }

    #[test]
    fn test_redundant_connection_dsu_2() {
        assert_eq!(
            redundant_connection_dsu(&[(1, 2), (2, 3), (3, 4), (1, 4), (1, 5)]),
            (1, 4)
        );
    }
}
