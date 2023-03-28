//! 95. Unique Binary Search Trees II
//!
//! Medium
//!
//! Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n.
//! Return the answer in any order.
//!
//! Example 1:
//! Input: n = 3
//! Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
//!
//! Example 2:
//! Input: n = 1
//! Output: [[1]]
//!
//! Constraints:
//! 1 <= n <= 8

use std::{cell::RefCell, rc::Rc};

use crate::utils::treenode::TreeNode;

type OptionalNode = Option<Rc<RefCell<TreeNode>>>;

pub fn generate_trees(n: usize) -> Vec<OptionalNode> {
    if n == 0 {
        return vec![];
    }

    generate_trees_range(1, n)
}

fn generate_trees_range(start: usize, end: usize) -> Vec<OptionalNode> {
    let mut trees = vec![];

    if start > end {
        trees.push(None);
        return trees;
    }

    for i in start..=end {
        let left_trees = generate_trees_range(start, i - 1);
        let right_trees = generate_trees_range(i + 1, end);

        for left_tree in &left_trees {
            for right_tree in &right_trees {
                let mut curr_node = TreeNode::new(i as i32);
                curr_node.left = left_tree.clone();
                curr_node.right = right_tree.clone();

                trees.push(Some(Rc::new(RefCell::new(curr_node))));
            }
        }
    }

    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_trees_1() {
        let mut correct = vec![vec![Some(1)]];

        let result = generate_trees(1);

        let mut result_tree_vecs = result.iter().map(TreeNode::to_vec).collect::<Vec<_>>();

        result_tree_vecs.sort_unstable();
        correct.sort_unstable();

        assert_eq!(result.len(), correct.len());
        assert_eq!(result_tree_vecs, correct);
    }

    #[test]
    fn test_generate_trees_2() {
        let mut correct = vec![
            vec![Some(1), None, Some(2), None, Some(3)],
            vec![Some(1), None, Some(3), Some(2)],
            vec![Some(2), Some(1), Some(3)],
            vec![Some(3), Some(1), None, None, Some(2)],
            vec![Some(3), Some(2), None, Some(1)],
        ];

        let result = generate_trees(3);

        let mut result_tree_vecs = result.iter().map(TreeNode::to_vec).collect::<Vec<_>>();

        result_tree_vecs.sort_unstable();
        correct.sort_unstable();

        assert_eq!(result.len(), correct.len());
        assert_eq!(result_tree_vecs, correct);
    }
}
