//! 1697. Checking Existence of Edge Length Limited Paths
//!
//! Hard
//!
//! An undirected graph of n nodes is defined by edgeList, where edgeList[i] = [ui, vi, disi] denotes an edge between nodes ui and vi with distance disi. Note that there may be multiple edges between two nodes.
//! Given an array queries, where queries[j] = [pj, qj, limitj], your task is to determine for each queries[j] whether there is a path between pj and qj such that each edge on the path has a distance strictly less than limitj .
//! Return a boolean array answer, where answer.length == queries.length and the jth value of answer is true if there is a path for queries[j] is true, and false otherwise.
//!
//! Example 1:
//! Input: n = 3, edgeList = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]], queries = [[0,1,2],[0,2,5]]
//! Output: [false,true]
//! Explanation: The above figure shows the given graph. Note that there are two overlapping edges between 0 and 1 with distances 2 and 16.
//! For the first query, between 0 and 1 there is no path where each distance is less than 2, thus we return false for this query.
//! For the second query, there is a path (0 -> 1 -> 2) of two edges with distances less than 5, thus we return true for this query.
//!
//! Example 2:
//! Input: n = 5, edgeList = [[0,1,10],[1,2,5],[2,3,9],[3,4,13]], queries = [[0,4,14],[1,4,13]]
//! Output: [true,false]
//! Exaplanation: The above figure shows the given graph.
//!
//! Constraints:
//! 2 <= n <= 10^5
//! 1 <= edgeList.length, queries.length <= 10^5
//! edgeList[i].length == 3
//! queries[j].length == 3
//! 0 <= ui, vi, pj, qj <= n - 1
//! ui != vi
//! pj != qj
//! 1 <= disi, limitj <= 10^9
//! There may be multiple edges between two nodes.

use crate::utils::union_find::UnionFind;

type Edge = (usize, usize, usize);
type Query = (usize, usize, usize);

pub fn distance_limited_paths_exist(n: i32, edges: &[Vec<i32>], queries: &[Vec<i32>]) -> Vec<bool> {
    let edges = edges
        .iter()
        .map(|edge| (edge[0] as usize, edge[1] as usize, edge[2] as usize))
        .collect::<Vec<_>>();

    let queries = queries
        .iter()
        .map(|query| (query[0] as usize, query[1] as usize, query[2] as usize))
        .collect::<Vec<_>>();

    distance_limited_paths(n as usize, edges, queries)
}

fn distance_limited_paths(n: usize, mut edges: Vec<Edge>, queries: Vec<Query>) -> Vec<bool> {
    edges.sort_unstable_by_key(|edge| edge.2);

    let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
    queries.sort_unstable_by_key(|q| q.1 .2);

    let mut uf = UnionFind::new(n);

    let mut edges = edges.into_iter().peekable();

    let mut answer = vec![false; queries.len()];

    for (query_idx, query) in queries {
        let node_a = query.0;
        let node_b = query.1;
        let limit = query.2;

        while let Some(edge) = edges.peek() {
            if edge.2 >= limit {
                break;
            }

            uf.union(edge.0, edge.1);
            edges.next();
        }

        if uf.find(node_a) == uf.find(node_b) {
            answer[query_idx] = true;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_limited_paths_1() {
        let edges = vec![(0, 1, 2), (1, 2, 4), (2, 0, 8), (1, 0, 16)];
        let queries = vec![(0, 1, 2), (0, 2, 5)];

        assert_eq!(distance_limited_paths(3, edges, queries), vec![false, true]);
    }

    #[test]
    fn test_distance_limited_paths_2() {
        let edges = vec![(0, 1, 10), (1, 2, 5), (2, 3, 9), (3, 4, 13)];
        let queries = vec![(0, 4, 14), (1, 4, 13)];

        assert_eq!(distance_limited_paths(5, edges, queries), vec![true, false]);
    }
}
