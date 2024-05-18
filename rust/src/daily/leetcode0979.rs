// 979. Distribute Coins in Binary Tree
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
use std::rc::Rc;
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //                                                              size,coins
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                let left = dfs(n.left.take(), res);
                let right = dfs(n.right.take(), res);
                let extra_coins = right + left + n.val - 1;
                *res += extra_coins.abs();
                return extra_coins;
            }
            0
        }
        let mut res = 0;
        dfs(root, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        })));
        assert_eq!(Solution::distribute_coins(root), 2);
    }
    #[test]
    fn test2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        })));
        assert_eq!(Solution::distribute_coins(root), 3);
    }
    #[test]
    fn test3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        right: None,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                    right: None,
                }))),
            }))),
            right: None,
        })));
        assert_eq!(Solution::distribute_coins(root), 4);
    }
}
