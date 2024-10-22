// 2583. Kth Largest Sum in a Binary Tree
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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        // bfs solution
        let mut levels: Vec<i64> = Vec::new();
        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        while !nodes.is_empty() {
            let mut level_sum = 0;
            for _ in 0..nodes.len() {
                if let Some(n) = nodes.pop_front().unwrap() {
                    let mut n = n.borrow_mut();
                    level_sum += n.val as i64;
                    if n.left.is_some() {
                        nodes.push_back(n.left.take());
                    }
                    if n.right.is_some() {
                        nodes.push_back(n.right.take());
                    }
                }
            }
            levels.push(level_sum);
        }

        if levels.len() < k as usize {
            return -1;
        }
        levels.sort_unstable();
        levels.reverse();
        levels[(k - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(Solution::kth_largest_level_sum(tree, 2), 13)
    }
}
