use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
    pub fn from_arr(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
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

    pub fn to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut values = vec![];

        if root.is_none() {
            return values;
        }

        let mut root = root.clone();

        let mut queue = VecDeque::new();
        queue.push_back(root.take());

        while !queue.is_empty() {
            let n = queue.len();

            for _ in 0..n {
                let node = queue.pop_front().unwrap();

                match node {
                    None => {
                        values.push(None);
                        continue;
                    }
                    Some(node) => {
                        values.push(Some(node.borrow().val));

                        if node.borrow().left.is_none() && node.borrow().right.is_none() {
                            continue;
                        }
                        queue.push_back(node.borrow_mut().left.take());
                        queue.push_back(node.borrow_mut().right.take());
                    }
                }
            }
        }

        values = values
            .into_iter()
            .rev()
            .skip_while(|x| x.is_none())
            .collect();

        values.reverse();

        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_arr_1() {
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

        let contructed = TreeNode::from_arr(values);
        let inner = contructed.unwrap().borrow().clone();
        assert_eq!(inner, tree);
    }

    #[test]
    fn test_from_arr_2() {
        let tree = TreeNode {
            val: 1,
            left: None,
            right: None,
        };

        let values = &[Some(1)];

        let contructed = TreeNode::from_arr(values);
        let inner = contructed.unwrap().borrow().clone();
        assert_eq!(inner, tree);
    }

    #[test]
    fn test_from_arr_3() {
        let tree = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        };

        let values = &[Some(1), None, Some(2)];

        let contructed = TreeNode::from_arr(values);
        let inner = contructed.unwrap().borrow().clone();
        assert_eq!(inner, tree);
    }

    #[test]
    fn test_to_vec_1() {
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

        let correct = &[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ];

        let vec = TreeNode::to_vec(&Some(Rc::new(RefCell::new(tree))));

        assert_eq!(vec, correct);
    }

    #[test]
    fn test_to_vec_2() {
        let tree = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        };

        let correct = &[Some(1), None, Some(2)];

        let vec = TreeNode::to_vec(&Some(Rc::new(RefCell::new(tree))));

        assert_eq!(vec, correct);
    }
}
