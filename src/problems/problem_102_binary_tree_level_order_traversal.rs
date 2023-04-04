//! 102. Binary Tree Level Order Traversal
//!
//! Medium
//!
//! Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [[3],[9,20],[15,7]]
//!
//! Example 2:
//! Input: root = [1]
//! Output: [[1]]
//!
//! Example 3:
//! Input: root = []
//! Output: []
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 2000].
//! -1000 <= Node.val <= 1000

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub enum Algorithm {
    Dfs,
    Bfs,
}

pub fn level_order(root: Node, alg: Algorithm) -> Vec<Vec<i32>> {
    match alg {
        Algorithm::Bfs => level_order_bfs(root),
        Algorithm::Dfs => level_order_dfs(root),
    }
}

fn level_order_bfs(root: Node) -> Vec<Vec<i32>> {
    let mut queue = VecDeque::new();

    if let Some(node) = root {
        queue.push_back(node);
    }

    let mut levels = vec![];

    while !queue.is_empty() {
        let mut level = vec![];

        for _ in 0..queue.len() {
            if let Some(node) = queue.pop_front() {
                level.push(node.borrow().val);

                if let Some(left) = node.borrow_mut().left.take() {
                    queue.push_back(left);
                }

                if let Some(right) = node.borrow_mut().right.take() {
                    queue.push_back(right);
                }
            }
        }

        levels.push(level)
    }

    levels
}

fn level_order_dfs(root: Node) -> Vec<Vec<i32>> {
    let mut levels = vec![];

    if let Some(root) = root {
        level_order_dfs_helper(root, 0, &mut levels);
    }

    levels
}

fn level_order_dfs_helper(node: Rc<RefCell<TreeNode>>, level: usize, levels: &mut Vec<Vec<i32>>) {
    if levels.len() == level {
        levels.push(vec![]);
    }

    levels.get_mut(level).unwrap().push(node.borrow().val);

    if let Some(left) = node.borrow().left.clone() {
        level_order_dfs_helper(left, level + 1, levels);
    }

    if let Some(right) = node.borrow().right.clone() {
        level_order_dfs_helper(right, level + 1, levels);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order_bfs_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(
            level_order_bfs(root),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn test_level_order_bfs_2() {
        let root = TreeNode::from_arr(&[Some(1)]);
        assert_eq!(level_order_bfs(root), vec![vec![1]]);
    }

    #[test]
    fn test_level_order_bfs_3() {
        let root = TreeNode::from_arr(&[]);
        assert_eq!(level_order_bfs(root), vec![vec![]; 0]);
    }

    #[test]
    fn test_level_order_dfs_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(
            level_order_dfs(root),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn test_level_order_dfs_2() {
        let root = TreeNode::from_arr(&[Some(1)]);
        assert_eq!(level_order_dfs(root), vec![vec![1]]);
    }

    #[test]
    fn test_level_order_dfs_3() {
        let root = TreeNode::from_arr(&[]);
        assert_eq!(level_order_dfs(root), vec![vec![]; 0]);
    }

    #[test]
    fn test_level_order_dfs_4() {
        let root = TreeNode::from_arr(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(level_order_dfs(root), vec![vec![1], vec![2, 3], vec![4, 5]]);
    }
}
