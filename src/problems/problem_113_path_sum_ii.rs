//! 113. Path Sum II
//!
//! Medium
//!
//! Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths where the sum of the node values in the path equals targetSum. Each path should be returned as a list of the node values, not node references.
//! A root-to-leaf path is a path starting from the root and ending at any leaf node. A leaf is a node with no children.
//!
//! Example 1:
//! Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
//! Output: [[5,4,11,2],[5,8,4,5]]
//! Explanation: There are two paths whose sum equals targetSum:
//! 5 + 4 + 11 + 2 = 22
//! 5 + 8 + 4 + 5 = 22
//!
//! Example 2:
//! Input: root = [1,2,3], targetSum = 5
//! Output: []
//!
//! Example 3:
//! Input: root = [1,2], targetSum = 0
//! Output: []
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 5000].
//! -1000 <= Node.val <= 1000
//! -1000 <= targetSum <= 1000

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::treenode::TreeNode;

pub enum Algorithm {
    Dfs,
    Bfs,
}

pub fn path_sum(
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
    alg: Algorithm,
) -> Vec<Vec<i32>> {
    match alg {
        Algorithm::Dfs => path_sum_dfs(root, target_sum),
        Algorithm::Bfs => path_sum_bfs(root, target_sum),
    }
}

fn path_sum_bfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut paths = vec![];
    let mut q = VecDeque::new();

    q.push_back((root, vec![], target_sum));

    while let Some((Some(node), mut path, mut target)) = q.pop_front() {
        let node_val = node.borrow().val;
        path.push(node_val);
        target -= node_val;

        let left_child = node.borrow().left.clone();
        let right_child = node.borrow().right.clone();

        if left_child.is_none() && right_child.is_none() && target == 0 {
            paths.push(path.clone());
        }

        if left_child.is_some() {
            q.push_back((left_child, path.clone(), target));
        }

        if right_child.is_some() {
            q.push_back((right_child, path, target));
        }
    }

    paths
}

pub fn path_sum_dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut paths = vec![];

    path_sum_helper(root, target_sum, &mut paths, &mut vec![]);

    paths
}

fn path_sum_helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    mut target: i32,
    paths: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
) {
    match node {
        None => (),
        Some(node) => {
            let node_val = node.borrow().val;

            target -= node_val;

            path.push(node_val);

            if node.borrow().left.is_none() && node.borrow().right.is_none() && target == 0 {
                paths.push(path.clone());
            }

            if node.borrow().left.is_some() {
                path_sum_helper(node.borrow().left.clone(), target, paths, path);
            }

            if node.borrow().right.is_some() {
                path_sum_helper(node.borrow().right.clone(), target, paths, path);
            }

            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_sum_dfs_1() {
        let root = TreeNode::from_arr(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            Some(5),
            Some(1),
        ]);

        assert_eq!(
            path_sum_dfs(root, 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }

    #[test]
    fn test_path_sum_dfs_2() {
        let root = TreeNode::from_arr(&[Some(1), Some(2), Some(3)]);

        assert_eq!(path_sum_dfs(root, 5), vec![vec![]; 0]);
    }

    #[test]
    fn test_path_sum_dfs_3() {
        let root = TreeNode::from_arr(&[Some(1), Some(2)]);

        assert_eq!(path_sum_dfs(root, 0), vec![vec![]; 0]);
    }

    #[test]
    fn test_path_sum_bfs_1() {
        let root = TreeNode::from_arr(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            Some(5),
            Some(1),
        ]);

        assert_eq!(
            path_sum_bfs(root, 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }

    #[test]
    fn test_path_sum_bfs_2() {
        let root = TreeNode::from_arr(&[Some(1), Some(2), Some(3)]);

        assert_eq!(path_sum_bfs(root, 5), vec![vec![]; 0]);
    }

    #[test]
    fn test_path_sum_bfs_3() {
        let root = TreeNode::from_arr(&[Some(1), Some(2)]);

        assert_eq!(path_sum_bfs(root, 0), vec![vec![]; 0]);
    }
}
