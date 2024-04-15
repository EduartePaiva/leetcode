// 129. Sum Root to Leaf Numbers
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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // travel down the tree,
        // accumulate the digits until a leaf is reached
        // if leaf return the accumulated digits
        // if not leaf return leaf left + leaf right

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut accumulator: i32) -> i32 {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                accumulator += n.val;
                // check if it's leaf
                if n.left.is_none() && n.right.is_none() {
                    return accumulator;
                }
                // continue traveling down
                accumulator *= 10;
                return dfs(n.left.take(), accumulator) + dfs(n.right.take(), accumulator);
            }
            0
        }

        dfs(root, 0)
    }
}
#[cfg(test)]
mod tests {
    //Some(Rc::new(RefCell::new(TreeNode)))
    use super::*;
    #[test]
    fn test1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(Solution::sum_numbers(tree), 25);
    }
}
