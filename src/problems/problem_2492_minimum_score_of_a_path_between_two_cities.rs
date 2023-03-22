//! 2492. Minimum Score of a Path Between Two Cities
//!
//! Medium
//!
//! You are given a positive integer n representing n cities numbered from 1 to n.
//! You are also given a 2D array roads where roads[i] = [ai, bi, distancei] indicates that there is a
//! bidirectional road between cities ai and bi with a distance equal to distancei.
//! The cities graph is not necessarily connected.
//! The score of a path between two cities is defined as the minimum distance of a road in this path.
//! Return the minimum possible score of a path between cities 1 and n.
//! Note: A path is a sequence of roads between two cities.
//! It is allowed for a path to contain the same road multiple times, and you can visit cities 1 and n multiple times along the path.
//! The test cases are generated such that there is at least one path between 1 and n.
//!
//! Example 1:
//! Input: n = 4, roads = [[1,2,9],[2,3,6],[2,4,5],[1,4,7]]
//! Output: 5
//! Explanation: The path from city 1 to 4 with the minimum score is: 1 -> 2 -> 4. The score of this path is min(9,5) = 5.
//! It can be shown that no other path has less score.
//!
//! Example 2:
//! Input: n = 4, roads = [[1,2,2],[1,3,4],[3,4,7]]
//! Output: 2
//! Explanation: The path from city 1 to 4 with the minimum score is: 1 -> 2 -> 1 -> 3 -> 4. The score of this path is min(2,2,4,7) = 2.
//!
//! Constraints:
//! 2 <= n <= 105
//! 1 <= roads.length <= 10^5
//! roads[i].length == 3
//! 1 <= ai, bi <= n
//! ai != bi
//! 1 <= distancei <= 10^4
//! There are no repeated edges.
//! There is at least one path between 1 and n.

use std::collections::{HashMap, VecDeque};

use crate::utils::union_find::UnionFind;

pub enum Algorithm {
    Bfs,
    Dfs,
    UnionFind,
}

type Road = (usize, usize, usize);

pub fn min_score_path(n: usize, roads: &[Road], algorithm: Algorithm) -> usize {
    match algorithm {
        Algorithm::Bfs => bfs(n, roads),
        Algorithm::Dfs => dfs(n, roads),
        Algorithm::UnionFind => union_find(n, roads),
    }
}

fn adjacency_list(n: usize, roads: &[Road]) -> HashMap<usize, Vec<(usize, usize)>> {
    let mut graph = HashMap::new();

    for i in 1..=n {
        graph.insert(i, vec![]);
    }

    for &(from, to, distance) in roads {
        assert!(from >= 1 && to >= 1 && from <= n && to <= n);

        graph.get_mut(&from).unwrap().push((to, distance));
        graph.get_mut(&to).unwrap().push((from, distance));
    }

    graph
}

fn union_find(n: usize, roads: &[Road]) -> usize {
    let mut uf = UnionFind::new(n + 1); // UnionFind implements from [0, n-1]

    for &(from, to, _) in roads {
        uf.union(from, to);
    }

    let mut min_cost = usize::MAX;

    for &(a, _b, distance) in roads {
        if uf.is_connected(1, a) {
            // We don't have to check b as a and b are connected.
            min_cost = min_cost.min(distance);
        }
    }

    min_cost
}

fn dfs(n: usize, roads: &[Road]) -> usize {
    let graph = adjacency_list(n, roads);
    let mut visited = vec![false; n + 1];

    let mut min_cost = usize::MAX;

    dfs_helper(1, &graph, &mut visited, &mut min_cost);

    min_cost
}

fn dfs_helper(
    node: usize,
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    visited: &mut [bool],
    min_cost: &mut usize,
) {
    visited[node] = true;

    if let Some(neighbours) = graph.get(&node) {
        for &(neighbour, distance) in neighbours {
            *min_cost = (*min_cost).min(distance);

            if !visited[neighbour] {
                dfs_helper(neighbour, graph, visited, min_cost);
            }
        }
    }
}

fn bfs(n: usize, roads: &[Road]) -> usize {
    let graph = adjacency_list(n, roads);
    let mut visited = vec![false; n + 1];

    let mut min_cost = usize::MAX;

    let mut queue = VecDeque::new();
    queue.push_back(1);
    visited[1] = true;

    while let Some(node) = queue.pop_front() {
        if let Some(neighbours) = graph.get(&node) {
            for &(neighbour, distance) in neighbours {
                min_cost = min_cost.min(distance);

                if !visited[neighbour] {
                    queue.push_back(neighbour);
                    visited[neighbour] = true;
                }
            }
        }
    }

    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find_1() {
        assert_eq!(
            union_find(4, &[(1, 2, 9), (2, 3, 6), (2, 4, 5), (1, 4, 7)]),
            5
        );
    }

    #[test]
    fn test_union_find_2() {
        assert_eq!(union_find(4, &[(1, 2, 2), (1, 3, 4), (3, 4, 7)]), 2);
    }

    #[test]
    fn test_bfs_1() {
        assert_eq!(bfs(4, &[(1, 2, 9), (2, 3, 6), (2, 4, 5), (1, 4, 7)]), 5);
    }

    #[test]
    fn test_bfs_2() {
        assert_eq!(bfs(4, &[(1, 2, 2), (1, 3, 4), (3, 4, 7)]), 2);
    }

    #[test]
    fn test_dfs_1() {
        assert_eq!(dfs(4, &[(1, 2, 9), (2, 3, 6), (2, 4, 5), (1, 4, 7)]), 5);
    }

    #[test]
    fn test_dfs_2() {
        assert_eq!(dfs(4, &[(1, 2, 2), (1, 3, 4), (3, 4, 7)]), 2);
    }

    #[test]
    fn test_adjacency_list_1() {
        let correct = [
            (1, vec![(2, 10)]),
            (3, vec![(4, 20)]),
            (2, vec![(1, 10)]),
            (4, vec![(3, 20)]),
        ]
        .into_iter()
        .collect::<HashMap<usize, Vec<(usize, usize)>>>();

        assert_eq!(adjacency_list(4, &[(1, 2, 10), (3, 4, 20)]), correct);
    }

    #[test]
    fn test_adjacency_list_2() {
        let correct = [
            (1, vec![(2, 10), (3, 20)]),
            (3, vec![(1, 20)]),
            (2, vec![(1, 10)]),
        ]
        .into_iter()
        .collect::<HashMap<usize, Vec<(usize, usize)>>>();

        assert_eq!(adjacency_list(3, &[(1, 2, 10), (1, 3, 20)]), correct);
    }
}
