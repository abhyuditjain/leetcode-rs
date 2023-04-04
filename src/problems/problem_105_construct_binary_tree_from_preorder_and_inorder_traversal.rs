//! 105. Construct Binary Tree from Preorder and Inorder Traversal
//!
//! Medium
//!
//! Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
//!
//! Example 1:
//! Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
//! Output: [3,9,20,null,null,15,7]
//!
//! Example 2:
//! Input: preorder = [-1], inorder = [-1]
//! Output: [-1]
//!
//! Constraints:
//! 1 <= preorder.length <= 3000
//! inorder.length == preorder.length
//! -3000 <= preorder[i], inorder[i] <= 3000
//! preorder and inorder consist of unique values.
//! Each value of inorder also appears in preorder.
//! preorder is guaranteed to be the preorder traversal of the tree.
//! inorder is guaranteed to be the inorder traversal of the tree.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
    let hm = inorder
        .iter()
        .enumerate()
        .map(|(i, &val)| (val, i))
        .collect::<HashMap<_, _>>();
    build_tree_helper(&mut preorder.iter(), &hm, (0, preorder.len() as isize - 1))
}

fn build_tree_helper(
    preorder: &mut std::slice::Iter<i32>,
    index_map: &HashMap<i32, usize>,
    range: (isize, isize),
) -> Node {
    if range.0 <= range.1 {
        if let Some(&val) = preorder.next() {
            if let Some(&i) = index_map.get(&val) {
                return Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: build_tree_helper(preorder, index_map, (range.0, i as isize - 1)),
                    right: build_tree_helper(preorder, index_map, (i as isize + 1, range.1)),
                })));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree_1() {
        let correct =
            TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        assert_eq!(
            build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            correct
        );
    }

    #[test]
    fn test_build_tree_2() {
        let correct = TreeNode::from_arr(&[Some(-1)]);

        assert_eq!(build_tree(vec![-1], vec![-1]), correct);
    }
}
