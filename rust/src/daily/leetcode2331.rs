// 2331. Evaluate Boolean Binary Tree

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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            match node.borrow().val {
                0 => false,
                1 => true,
                2 => {
                    Solution::evaluate_tree(node.borrow().left.clone())
                        || Solution::evaluate_tree(node.borrow().right.clone())
                }
                3 => {
                    Solution::evaluate_tree(node.borrow().left.clone())
                        && Solution::evaluate_tree(node.borrow().right.clone())
                }
                _ => panic!("invalid tree value"),
            }
        } else {
            panic!("invalid tree structure");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        })));

        assert_eq!(Solution::evaluate_tree(root), true);
    }
}
