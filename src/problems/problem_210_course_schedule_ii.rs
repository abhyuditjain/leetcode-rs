//! 210. Course Schedule II
//!
//! Medium
//!
//! There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
//! For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
//! Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.
//!
//! Example 1:
//! Input: numCourses = 2, prerequisites = [[1,0]]
//! Output: [0,1]
//! Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
//!
//! Example 2:
//! Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
//! Output: [0,2,1,3]
//! Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
//! So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
//!
//! Example 3:
//! Input: numCourses = 1, prerequisites = []
//! Output: [0]
//!
//! Constraints:
//! 1 <= numCourses <= 2000
//! 0 <= prerequisites.length <= numCourses * (numCourses - 1)
//! prerequisites[i].length == 2
//! 0 <= ai, bi < numCourses
//! ai != bi
//! All the pairs [ai, bi] are distinct.

use std::collections::VecDeque;

pub enum Algorithm {
    Dfs,
    TopologicalSort,
}

#[derive(Clone, PartialEq, Eq)]
enum Status {
    Red,    // Unvisited
    Yellow, // In progress
    Green,  // Completed
}

pub fn get_order(vertices: usize, edges: &[Vec<i32>], alg: Algorithm) -> Vec<i32> {
    let edges = edges
        .iter()
        .map(|courses| (courses[0] as usize, courses[1] as usize))
        .collect::<Vec<_>>();

    let order = match alg {
        Algorithm::Dfs => get_order_dfs(vertices, &edges),
        Algorithm::TopologicalSort => get_order_topological_sort(vertices, &edges),
    };

    order.into_iter().map(|x| x as i32).collect()
}

pub fn get_order_dfs(vertices: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let graph = build_graph(vertices, edges);
    let mut statuses = vec![Status::Red; vertices];

    let mut order = vec![];

    for v in 0..vertices {
        if statuses[v] == Status::Red && has_cycle(v, &mut statuses, &graph, &mut order) {
            return vec![];
        }
    }

    order.reverse();
    order
}

pub fn get_order_topological_sort(vertices: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let graph = build_graph(vertices, edges);
    let mut indegrees = build_indegrees(vertices, edges);

    let mut q = indegrees
        .iter()
        .enumerate()
        .filter_map(|(course, indegree)| match *indegree {
            0 => Some(course),
            _ => None,
        })
        .collect::<VecDeque<_>>();

    let mut order = vec![];

    while !q.is_empty() {
        if let Some(first) = q.pop_front() {
            order.push(first);

            for &second in &graph[first] {
                indegrees[second] -= 1;

                if indegrees[second] == 0 {
                    q.push_back(second);
                }
            }
        }
    }

    match order.len() == vertices {
        true => order,
        false => vec![],
    }
}

fn has_cycle(
    course: usize,
    statuses: &mut [Status],
    graph: &[Vec<usize>],
    order: &mut Vec<usize>,
) -> bool {
    match statuses[course] {
        Status::Green => false,
        Status::Yellow => true,
        Status::Red => {
            statuses[course] = Status::Yellow;

            if graph[course]
                .iter()
                .any(|&next_course| has_cycle(next_course, statuses, graph, order))
            {
                return true;
            }

            statuses[course] = Status::Green;
            order.push(course);

            false
        }
    }
}

fn build_graph(vertices: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; vertices];

    for &(dst, src) in edges {
        graph[src].push(dst);
    }

    graph
}

fn build_indegrees(vertices: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut indegrees = vec![0; vertices];

    for &(dst, _) in edges {
        indegrees[dst] += 1;
    }

    indegrees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_dfs_1() {
        assert_eq!(get_order_dfs(2, &[(1, 0)]), vec![0, 1]);
    }

    #[test]
    fn test_get_order_dfs_2() {
        assert_eq!(
            get_order_dfs(4, &[(1, 0), (2, 0), (3, 1), (3, 2)]),
            vec![0, 2, 1, 3]
        );
    }

    #[test]
    fn test_get_order_dfs_3() {
        assert_eq!(get_order_dfs(1, &[]), vec![0]);
    }

    #[test]
    fn test_get_order_dfs_4() {
        assert_eq!(get_order_dfs(2, &[(1, 0), (0, 1)]), vec![]);
    }

    #[test]
    fn test_get_order_topological_sort_1() {
        assert_eq!(get_order_topological_sort(2, &[(1, 0)]), vec![0, 1]);
    }

    #[test]
    fn test_get_order_topological_sort_2() {
        assert_eq!(
            get_order_topological_sort(4, &[(1, 0), (2, 0), (3, 1), (3, 2)]),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn test_get_order_topological_sort_3() {
        assert_eq!(get_order_topological_sort(1, &[]), vec![0]);
    }

    #[test]
    fn test_get_order_topological_sort_4() {
        assert_eq!(get_order_topological_sort(2, &[(1, 0), (0, 1)]), vec![]);
    }
}
