//! 1319. Number of Operations to Make Network Connected
//!
//! Medium
//!
//! There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where connections[i] = [ai, bi] represents a connection between computers ai and bi. Any computer can reach any other computer directly or indirectly through the network.
//! You are given an initial computer network connections. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.
//! Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.
//!
//! Example 1:
//! Input: n = 4, connections = [[0,1],[0,2],[1,2]]
//! Output: 1
//! Explanation: Remove cable between computer 1 and 2 and place between computers 1 and 3.
//!
//! Example 2:
//! Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
//! Output: 2
//!
//! Example 3:
//! Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
//! Output: -1
//! Explanation: There are not enough cables.
//!
//! Constraints:
//! 1 <= n <= 10^5
//! 1 <= connections.length <= min(n * (n - 1) / 2, 10^5)
//! connections[i].length == 2
//! 0 <= ai, bi < n
//! ai != bi
//! There are no repeated connections.
//! No two computers are connected by more than one cable.

use std::collections::VecDeque;

use crate::utils::union_find::UnionFind;

pub enum Algorithm {
    Dfs,
    Bfs,
    DSU,
}

type Edge = (usize, usize);

pub fn make_connected(n: i32, edges: Vec<Vec<i32>>, alg: Algorithm) -> i32 {
    let edges = edges
        .into_iter()
        .map(|edge| (edge[0] as usize, edge[1] as usize))
        .collect::<Vec<Edge>>();

    if let Some(operations) = match alg {
        Algorithm::Dfs => num_operations_dfs(n as usize, &edges),
        Algorithm::Bfs => num_operations_bfs(n as usize, &edges),
        Algorithm::DSU => num_operations_dsu(n as usize, &edges),
    } {
        operations as i32
    } else {
        -1
    }
}

fn num_operations_dfs(n: usize, edges: &[Edge]) -> Option<usize> {
    fn dfs(graph: &[Vec<usize>], node: usize, visited: &mut [bool]) {
        visited[node] = true;

        for &neighbour in &graph[node] {
            if !visited[neighbour] {
                dfs(graph, neighbour, visited)
            }
        }
    }

    if edges.len() < n - 1 {
        return None;
    }

    let graph = make_graph(n, edges);
    let mut connected_components = 0;
    let mut visited = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            connected_components += 1;
            dfs(&graph, i, &mut visited);
        }
    }

    Some(connected_components - 1)
}

fn num_operations_bfs(n: usize, edges: &[Edge]) -> Option<usize> {
    fn bfs(graph: &[Vec<usize>], node: usize, visited: &mut [bool]) {
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

    if edges.len() < n - 1 {
        return None;
    }

    let graph = make_graph(n, edges);
    let mut connected_components = 0;
    let mut visited = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            connected_components += 1;
            bfs(&graph, i, &mut visited);
        }
    }

    Some(connected_components - 1)
}

fn num_operations_dsu(n: usize, edges: &[Edge]) -> Option<usize> {
    if edges.len() < n - 1 {
        return None;
    }

    let mut connected_components = n;
    let mut uf = UnionFind::new(n);

    for &(a, b) in edges {
        if uf.union(a, b) {
            connected_components -= 1;
        }
    }

    Some(connected_components - 1)
}

fn make_graph(n: usize, edges: &[Edge]) -> Vec<Vec<usize>> {
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
    fn test_num_operations_dfs() {
        assert_eq!(num_operations_dfs(4, &[(0, 1), (0, 2), (1, 2)]), Some(1));

        assert_eq!(
            num_operations_dfs(6, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3)]),
            Some(2)
        );

        assert_eq!(
            num_operations_dfs(6, &[(0, 1), (0, 2), (0, 3), (1, 2)]),
            None
        );
    }

    #[test]
    fn test_num_operations_bfs() {
        assert_eq!(num_operations_bfs(4, &[(0, 1), (0, 2), (1, 2)]), Some(1));

        assert_eq!(
            num_operations_bfs(6, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3)]),
            Some(2)
        );

        assert_eq!(
            num_operations_bfs(6, &[(0, 1), (0, 2), (0, 3), (1, 2)]),
            None
        );
    }

    #[test]
    fn test_num_operations_dsu() {
        assert_eq!(num_operations_dsu(4, &[(0, 1), (0, 2), (1, 2)]), Some(1));

        assert_eq!(
            num_operations_dsu(6, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3)]),
            Some(2)
        );

        assert_eq!(
            num_operations_dsu(6, &[(0, 1), (0, 2), (0, 3), (1, 2)]),
            None
        );
    }
}
