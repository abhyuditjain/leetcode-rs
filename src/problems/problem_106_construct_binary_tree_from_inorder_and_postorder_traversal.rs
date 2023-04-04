//! 106. Construct Binary Tree from Inorder and Postorder Traversal
//!
//! Medium
//!
//! Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.
//!
//! Example 1:
//! Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
//! Output: [3,9,20,null,null,15,7]
//!
//! Example 2:
//! Input: inorder = [-1], postorder = [-1]
//! Output: [-1]
//!
//! Constraints:
//! 1 <= inorder.length <= 3000
//! postorder.length == inorder.length
//! -3000 <= inorder[i], postorder[i] <= 3000
//! inorder and postorder consist of unique values.
//! Each value of postorder also appears in inorder.
//! inorder is guaranteed to be the inorder traversal of the tree.
//! postorder is guaranteed to be the postorder traversal of the tree.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::utils::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn build_tree(inorder: &[i32], postorder: &[i32]) -> Node {
    let inorder_indices = inorder
        .iter()
        .enumerate()
        .map(|(i, &val)| (val, i))
        .collect::<HashMap<_, _>>();

    let mut postorder_idx = postorder.len() as isize - 1;

    build_tree_helper(
        postorder,
        &inorder_indices,
        0,
        inorder.len() as isize - 1,
        &mut postorder_idx,
    )
}

fn build_tree_helper(
    postorder: &[i32],
    inorder_indices: &HashMap<i32, usize>,
    inorder_start: isize,
    inorder_end: isize,
    postorder_idx: &mut isize,
) -> Node {
    if inorder_start > inorder_end {
        return Node::None;
    }

    let root_val = postorder[*postorder_idx as usize];
    let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

    *postorder_idx -= 1;

    let root_idx_in_inorder = *inorder_indices.get(&root_val).unwrap();

    root.borrow_mut().right = build_tree_helper(
        postorder,
        inorder_indices,
        root_idx_in_inorder as isize + 1,
        inorder_end,
        postorder_idx,
    );

    root.borrow_mut().left = build_tree_helper(
        postorder,
        inorder_indices,
        inorder_start,
        root_idx_in_inorder as isize - 1,
        postorder_idx,
    );

    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree_1() {
        let correct =
            TreeNode::from_arr(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        assert_eq!(build_tree(&[9, 3, 15, 20, 7], &[9, 15, 7, 20, 3]), correct);
    }

    #[test]
    fn test_build_tree_2() {
        let correct = TreeNode::from_arr(&[Some(-1)]);

        assert_eq!(build_tree(&[-1], &[-1]), correct);
    }
}
