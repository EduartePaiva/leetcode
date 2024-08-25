// 145. Binary Tree Postorder Traversal
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        // recursive
        fn dfs(res: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = root {
                let mut n = n.borrow_mut();
                dfs(res, n.left.take());
                dfs(res, n.right.take());
                res.push(n.val);
            }
        }
        dfs(&mut res, root);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        })));
        assert_eq!(Solution::postorder_traversal(root), vec![3, 2, 1])
    }
}
