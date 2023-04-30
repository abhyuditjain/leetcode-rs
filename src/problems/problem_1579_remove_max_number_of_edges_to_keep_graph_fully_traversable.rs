//! 1579. Remove Max Number of Edges to Keep Graph Fully Traversable
//!
//! Hard
//!
//! Alice and Bob have an undirected graph of n nodes and three types of edges:
//! Type 1: Can be traversed by Alice only.
//! Type 2: Can be traversed by Bob only.
//! Type 3: Can be traversed by both Alice and Bob.
//! Given an array edges where edges[i] = [typei, ui, vi] represents a bidirectional edge of type typei between nodes ui and vi, find the maximum number of edges you can remove so that after removing the edges, the graph can still be fully traversed by both Alice and Bob. The graph is fully traversed by Alice and Bob if starting from any node, they can reach all other nodes.
//! Return the maximum number of edges you can remove, or return -1 if Alice and Bob cannot fully traverse the graph.
//!
//! Example 1:
//! Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
//! Output: 2
//! Explanation: If we remove the 2 edges [1,1,2] and [1,1,3]. The graph will still be fully traversable by Alice and Bob. Removing any additional edge will not make it so. So the maximum number of edges we can remove is 2.
//!
//! Example 2:
//! Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
//! Output: 0
//! Explanation: Notice that removing any edge will not make the graph fully traversable by Alice and Bob.
//!
//! Example 3:
//! Input: n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
//! Output: -1
//! Explanation: In the current graph, Alice cannot reach node 4 from the other nodes. Likewise, Bob cannot reach 1. Therefore it's impossible to make the graph fully traversable.
//!
//! Constraints:
//! 1 <= n <= 10^5
//! 1 <= edges.length <= min(105, 3 * n * (n - 1) / 2)
//! edges[i].length == 3
//! 1 <= typei <= 3
//! 1 <= ui < vi <= n
//! All tuples (typei, ui, vi) are distinct.

use crate::utils::union_find::UnionFind;

#[derive(PartialEq, Eq)]
enum EdgeType {
    Alice,
    Bob,
    Both,
}

impl From<i32> for EdgeType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Alice,
            2 => Self::Bob,
            3 => Self::Both,
            _ => unimplemented!(),
        }
    }
}

type Edge = (EdgeType, usize, usize);

pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;

    let edges = edges
        .into_iter()
        .map(|edge| (edge[0].into(), edge[1] as usize - 1, edge[2] as usize - 1))
        .collect::<Vec<_>>();

    match num_edges_to_remove(n, &edges) {
        Some(x) => x as i32,
        None => -1,
    }
}

fn num_edges_to_remove(n: usize, edges: &[Edge]) -> Option<usize> {
    let mut alice_uf = UnionFind::new(n);
    let mut bob_uf = UnionFind::new(n);

    let mut edges_required = 0;

    for (_, a, b) in edges.iter().filter(|edge| edge.0 == EdgeType::Both) {
        match (alice_uf.union(*a, *b), bob_uf.union(*a, *b)) {
            (false, false) => {}
            _ => edges_required += 1,
        }
    }

    for (edge_type, a, b) in edges.iter().filter(|edge| edge.0 != EdgeType::Both) {
        match edge_type {
            EdgeType::Alice => {
                if alice_uf.union(*a, *b) {
                    edges_required += 1
                }
            }
            EdgeType::Bob => {
                if bob_uf.union(*a, *b) {
                    edges_required += 1
                }
            }
            EdgeType::Both => unreachable!(),
        }
    }

    if alice_uf.is_graph_connected() && bob_uf.is_graph_connected() {
        return Some(edges.len() - edges_required);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{EdgeType::*, *};

    #[test]
    fn test_num_edges_to_remove_1() {
        assert_eq!(
            num_edges_to_remove(
                4,
                &[
                    (Both, 0, 1),
                    (Both, 1, 2),
                    (Alice, 0, 2),
                    (Alice, 1, 3),
                    (Alice, 0, 1),
                    (Bob, 2, 3)
                ]
            ),
            Some(2)
        );
    }

    #[test]
    fn test_num_edges_to_remove_2() {
        assert_eq!(
            num_edges_to_remove(4, &[(Both, 0, 1), (Both, 1, 2), (Alice, 0, 3), (Bob, 0, 3)]),
            Some(0)
        );
    }

    #[test]
    fn test_num_edges_to_remove_3() {
        assert_eq!(
            num_edges_to_remove(4, &[(Both, 1, 2), (Alice, 0, 1), (Bob, 2, 3)]),
            None
        );
    }
}
