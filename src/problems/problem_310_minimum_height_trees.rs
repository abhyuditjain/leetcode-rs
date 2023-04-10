//! 310. Minimum Height Trees
//!
//! Medium
//!
//! A tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.
//! Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges where edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in the tree, you can choose any node of the tree as the root. When you select a node x as the root, the result tree has height h. Among all possible rooted trees, those with minimum height (i.e. min(h))  are called minimum height trees (MHTs).
//! Return a list of all MHTs' root labels. You can return the answer in any order.
//! The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
//!
//! Example 1:
//! Input: n = 4, edges = [[1,0],[1,2],[1,3]]
//! Output: [1]
//! Explanation: As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
//!
//! Example 2:
//! Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
//! Output: [3,4]
//!
//! Constraints:
//! 1 <= n <= 2 * 10^4
//! edges.length == n - 1
//! 0 <= ai, bi < n
//! ai != bi
//! All the pairs (ai, bi) are distinct.
//! The given input is guaranteed to be a tree and there will be no repeated edges.

use std::collections::HashSet;

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let edges = edges
        .into_iter()
        .map(|edge| (edge[0] as usize, edge[1] as usize))
        .collect::<Vec<_>>();

    let answer = min_height_trees(n as usize, &edges);

    answer.into_iter().map(|x| x as i32).collect()
}

pub fn min_height_trees(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    if n < 2 {
        return (0..n).collect();
    }

    let mut graph = build_adjacency_list(n, edges);
    let mut leaves = graph
        .iter()
        .enumerate()
        .filter_map(|(i, set)| if set.len() == 1 { Some(i) } else { None })
        .collect::<Vec<_>>();

    // Trim the leaves until reaching the centroids

    let mut remaining_nodes = n;

    while remaining_nodes > 2 {
        remaining_nodes -= leaves.len();

        let mut new_leaves = vec![];

        for leaf in leaves {
            let neighbour = *graph[leaf].iter().next().unwrap();
            graph[neighbour].remove(&leaf);

            if graph[neighbour].len() == 1 {
                new_leaves.push(neighbour);
            }
        }

        leaves = new_leaves;
    }
    leaves
}

fn build_adjacency_list(n: usize, edges: &[(usize, usize)]) -> Vec<HashSet<usize>> {
    let mut graph = vec![HashSet::new(); n];

    for &(a, b) in edges {
        graph[a].insert(b);
        graph[b].insert(a);
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_height_trees_1() {
        assert_eq!(min_height_trees(4, &[(1, 0), (1, 2), (1, 3)]), vec![1]);
    }

    #[test]
    fn test_min_height_trees_2() {
        assert_eq!(
            min_height_trees(6, &[(3, 0), (3, 1), (3, 2), (3, 4), (5, 4)]),
            vec![3, 4]
        );
    }
}
