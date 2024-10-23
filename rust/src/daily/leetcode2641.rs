// 2641. Cousins in Binary Tree II
pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = &root {
            n.borrow_mut().val = 0;
        }

        let mut levels = vec![];

        // calculate the levels sum with bfs
        let mut deq = VecDeque::new();
        if let Some(n) = &root {
            deq.push_back(Rc::clone(n));
        }
        while !deq.is_empty() {
            let mut sm = 0;
            for _ in 0..deq.len() {
                let n = deq.pop_front().unwrap();
                let n = n.borrow();
                sm += n.val as i64;
                if let Some(left) = &n.left {
                    deq.push_back(Rc::clone(left));
                }
                if let Some(right) = &n.right {
                    deq.push_back(Rc::clone(right));
                }
            }
            levels.push(sm);
        }

        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, levels: &Vec<i64>, depth: usize) {
            if depth + 1 == levels.len() {
                return;
            }
            if let Some(parent) = root {
                let parent = parent.borrow();
                let children_sm = if let Some(n) = &parent.left {
                    n.borrow().val as i64
                } else {
                    0
                } + if let Some(n) = &parent.right {
                    n.borrow().val as i64
                } else {
                    0
                };
                let cousins = levels[depth + 1] - children_sm;
                if let Some(left) = &parent.left {
                    let mut left = left.borrow_mut();
                    left.val = cousins as i32;
                }
                if let Some(right) = &parent.right {
                    let mut right = right.borrow_mut();
                    right.val = cousins as i32;
                }
                dfs(&parent.left, levels, depth + 1);
                dfs(&parent.right, levels, depth + 1);
            }
        }

        dfs(&root, &levels, 0);

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        let output = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(Solution::replace_value_in_tree(root), output);
    }
}
