use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl TreeNode {
    pub fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let nodes = values
            .iter()
            .map(|x| {
                if x.is_none() {
                    None
                } else {
                    Some(Rc::new(RefCell::new(TreeNode::new(x.unwrap()))))
                }
            })
            .collect::<Vec<_>>();

        let mut slow = 0;
        let mut fast = 1;
        let n = nodes.len();

        while fast < n {
            let m = fast;
            while slow < m {
                if let Some(node_rc) = &nodes[slow] {
                    if fast < values.len() {
                        node_rc.borrow_mut().left = nodes[fast].clone();
                    }
                    if fast + 1 < values.len() {
                        node_rc.borrow_mut().right = nodes[fast + 1].clone();
                    }
                    fast += 2;
                }
                slow += 1;
            }
        }

        nodes[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree() {
        let tree = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        };

        let values = &[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ];

        let contructed = TreeNode::build_tree(values);
        let inner = contructed.unwrap().borrow().clone();
        assert_eq!(inner, tree);
    }
}
