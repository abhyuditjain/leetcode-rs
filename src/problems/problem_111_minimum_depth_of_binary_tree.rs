//! 111. Minimum Depth of Binary Tree
//!
//! Easy
//!
//! Given a binary tree, find its minimum depth.
//! The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
//! Note: A leaf is a node with no children.
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: 2
//!
//! Example 2:
//! Input: root = [2,null,3,null,4,null,5,null,6]
//! Output: 5
//!
//! Constraints:
//! The number of nodes in the tree is in the range [0, 10^5].
//! -1000 <= Node.val <= 1000

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub enum Algorithm {
    Dfs,
    Bfs,
}

pub fn min_depth(root: Node, alg: Algorithm) -> usize {
    match alg {
        Algorithm::Dfs => min_depth_dfs(root),
        Algorithm::Bfs => min_depth_bfs(root),
    }
}

fn min_depth_bfs(root: Node) -> usize {
    if root.is_none() {
        return 0;
    }

    let mut q = VecDeque::new();
    q.push_back((root, 1));

    while !q.is_empty() {
        for _ in 0..q.len() {
            let (node, depth) = q.pop_front().unwrap();

            let node = node.unwrap();

            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return depth;
            }

            if node.borrow().left.is_some() {
                q.push_back((node.borrow().left.clone(), depth + 1));
            }

            if node.borrow().right.is_some() {
                q.push_back((node.borrow().right.clone(), depth + 1));
            }
        }
    }

    unreachable!()
}

pub fn min_depth_dfs(root: Node) -> usize {
    match root {
        Some(root) => {
            match (
                min_depth_dfs(root.borrow().left.clone()),
                min_depth_dfs(root.borrow().right.clone()),
            ) {
                (depth, 0) | (0, depth) => depth + 1,
                (left_depth, right_depth) => left_depth.min(right_depth) + 1,
            }
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_depth_dfs_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        assert_eq!(min_depth_dfs(root), 2);
    }

    #[test]
    fn test_min_depth_dfs_2() {
        let root = TreeNode::from_arr(&[
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);

        assert_eq!(min_depth_dfs(root), 5);
    }

    #[test]
    fn test_min_depth_dfs_3() {
        let root = TreeNode::from_arr(&[]);

        assert_eq!(min_depth_dfs(root), 0);
    }

    #[test]
    fn test_min_depth_bfs_1() {
        let root = TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        assert_eq!(min_depth_bfs(root), 2);
    }

    #[test]
    fn test_min_depth_bfs_2() {
        let root = TreeNode::from_arr(&[
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);

        assert_eq!(min_depth_bfs(root), 5);
    }

    #[test]
    fn test_min_depth_bfs_3() {
        let root = TreeNode::from_arr(&[]);

        assert_eq!(min_depth_bfs(root), 0);
    }
}
