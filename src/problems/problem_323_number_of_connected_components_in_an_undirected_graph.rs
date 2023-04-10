//! 323. Number of Connected Components in an Undirected Graph
//!
//! Medium
//!
//! You have a graph of n nodes. You are given an integer n and an array edges where edges[i] = [ai, bi] indicates that there is an edge between ai and bi in the graph.
//! Return the number of connected components in the graph.
//!
//! Example 1:
//! Input: n = 5, edges = [[0,1],[1,2],[3,4]]
//! Output: 2
//!
//! Example 2:
//! Input: n = 5, edges = [[0,1],[1,2],[2,3],[3,4]]
//! Output: 1
//!
//! Constraints:
//! 1 <= n <= 2000
//! 1 <= edges.length <= 5000
//! edges[i].length == 2
//! 0 <= ai <= bi < n
//! ai != bi
//! There are no repeated edges.

use crate::utils::union_find::UnionFind;

type Edge = (usize, usize);

pub enum Algorithm {
    Dfs,
    DSU,
}

pub fn count_connected_components(n: usize, edges: &[Edge], alg: Algorithm) -> usize {
    match alg {
        Algorithm::Dfs => count_connected_components_dfs(n, edges),
        Algorithm::DSU => count_connected_components_dsu(n, edges),
    }
}

fn count_connected_components_dsu(n: usize, edges: &[Edge]) -> usize {
    let mut uf = UnionFind::new(n);

    let mut total_components = n;

    for &(a, b) in edges {
        if uf.union(a, b) {
            total_components -= 1;
        }
    }

    total_components
}

fn count_connected_components_dfs(n: usize, edges: &[Edge]) -> usize {
    fn dfs(start: usize, graph: &[Vec<usize>], visited: &mut [bool]) {
        visited[start] = true;

        for &neighbour in &graph[start] {
            if !visited[neighbour] {
                dfs(neighbour, graph, visited);
            }
        }
    }
    let mut components = 0;
    let graph = build_adjacency_list(n, edges);
    let mut visited = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            components += 1;
            dfs(i, &graph, &mut visited);
        }
    }

    components
}

fn build_adjacency_list(n: usize, edges: &[Edge]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n];

    for &(a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_connected_components_dfs_1() {
        assert_eq!(
            count_connected_components_dfs(5, &[(0, 1), (1, 2), (3, 4)]),
            2
        );
    }

    #[test]
    fn test_count_connected_components_dfs_2() {
        assert_eq!(
            count_connected_components_dfs(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]),
            1
        );
    }

    #[test]
    fn test_count_connected_components_dsu_1() {
        assert_eq!(
            count_connected_components_dsu(5, &[(0, 1), (1, 2), (3, 4)]),
            2
        );
    }

    #[test]
    fn test_count_connected_components_dsu_2() {
        assert_eq!(
            count_connected_components_dsu(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]),
            1
        );
    }
}
