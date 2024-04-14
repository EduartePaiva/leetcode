// 404. Sum of Left Leaves

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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // if it's left and it's leaf, return the val.
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            if let Some(n) = node {
                let mut new_n = n.borrow_mut();
                if new_n.left.is_none() && new_n.right.is_none() && is_left {
                    return new_n.val;
                }
                return dfs(new_n.left.take(), true) + dfs(new_n.right.take(), false);
            }
            0
        }
        dfs(root, false)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(Solution::sum_of_left_leaves(tree), 24);
    }
}
