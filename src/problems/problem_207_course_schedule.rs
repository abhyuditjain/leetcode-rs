//! 207. Course Schedule
//!
//! Medium
//!
//! There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
//! For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
//! Return true if you can finish all courses. Otherwise, return false.
//!
//! Example 1:
//! Input: numCourses = 2, prerequisites = [[1,0]]
//! Output: true
//! Explanation: There are a total of 2 courses to take.
//! To take course 1 you should have finished course 0. So it is possible.
//!
//! Example 2:
//! Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
//! Output: false
//! Explanation: There are a total of 2 courses to take.
//! To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
//!
//! Constraints:
//! 1 <= numCourses <= 2000
//! 0 <= prerequisites.length <= 5000
//! prerequisites[i].length == 2
//! 0 <= ai, bi < numCourses
//! All the pairs prerequisites[i] are unique.

use std::collections::VecDeque;

pub enum Algorithm {
    Dfs,
    TopologicalSort,
}

pub fn can_finish(num_courses: usize, prerequisites: &[Vec<i32>], alg: Algorithm) -> bool {
    let prerequisites = prerequisites
        .iter()
        .map(|courses| (courses[0] as usize, courses[1] as usize))
        .collect::<Vec<_>>();

    match alg {
        Algorithm::Dfs => can_finish_courses_dfs(num_courses, &prerequisites),
        Algorithm::TopologicalSort => {
            can_finish_courses_topological_sort(num_courses, &prerequisites)
        }
    }
}

pub fn can_finish_courses_topological_sort(
    num_courses: usize,
    prerequisites: &[(usize, usize)],
) -> bool {
    let graph = build_graph(num_courses, prerequisites);
    let mut in_degrees = build_indegrees(num_courses, prerequisites);

    let mut q = in_degrees
        .iter()
        .enumerate()
        .filter_map(|(course, indegree)| match *indegree {
            0 => Some(course),
            _ => None,
        })
        .collect::<VecDeque<_>>();

    let mut courses = vec![];

    while !q.is_empty() {
        if let Some(first) = q.pop_front() {
            courses.push(first);

            for &second in &graph[first] {
                in_degrees[second] -= 1;

                if in_degrees[second] == 0 {
                    q.push_back(second);
                }
            }
        }
    }

    courses.len() == num_courses
}

#[derive(Clone)]
enum Status {
    Red,    // Unvisited
    Yellow, // In progress
    Green,  // Completed
}

pub fn can_finish_courses_dfs(num_courses: usize, prerequisites: &[(usize, usize)]) -> bool {
    let graph = build_graph(num_courses, prerequisites);
    let mut statuses = vec![Status::Red; num_courses];

    (0..num_courses).all(|course| !has_cycle(course, &mut statuses, &graph))
}

fn has_cycle(course: usize, statuses: &mut [Status], graph: &[Vec<usize>]) -> bool {
    match statuses[course] {
        Status::Green => false,
        Status::Yellow => true,
        Status::Red => {
            statuses[course] = Status::Yellow;

            if graph[course]
                .iter()
                .any(|&next_course| has_cycle(next_course, statuses, graph))
            {
                return true;
            }

            statuses[course] = Status::Green;

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
    fn test_can_finish_courses_topological_sort_1() {
        assert!(can_finish_courses_topological_sort(2, &[(1, 0)]));
    }

    #[test]
    fn test_can_finish_courses_topological_sort_2() {
        assert!(!can_finish_courses_topological_sort(2, &[(1, 0), (0, 1)]));
    }

    #[test]
    fn test_can_finish_courses_topological_sort_3() {
        assert!(can_finish_courses_topological_sort(3, &[(0, 1), (1, 2)]));
    }

    #[test]
    fn test_can_finish_courses_dfs_1() {
        assert!(can_finish_courses_dfs(2, &[(1, 0)]));
    }

    #[test]
    fn test_can_finish_courses_dfs_2() {
        assert!(!can_finish_courses_dfs(2, &[(1, 0), (0, 1)]));
    }

    #[test]
    fn test_can_finish_courses_dfs_3() {
        assert!(can_finish_courses_dfs(3, &[(0, 1), (1, 2)]));
    }
}
