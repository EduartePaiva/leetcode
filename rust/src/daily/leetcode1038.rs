// 1038. Binary Search Tree to Greater Sum Tree
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
    pub fn bst_to_gst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // grab right, deliver to the right, and return what delivered to the right.
        fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, delivered: i32) -> i32 {
            if let Some(rt) = root {
                let mut rt = rt.borrow_mut();
                let cur_val = rt.val;
                let sum_right = dfs(&mut rt.right, delivered);
                let sum_left = dfs(&mut rt.left, delivered + cur_val + sum_right);

                rt.val = cur_val + sum_right + delivered;
                return sum_left + sum_right + cur_val;
            }
            return 0;
        }
        dfs(&mut root, 0);
        return root;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        //Some(Rc::new(RefCell::new(TreeNode{})))
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                }))),
            }))),
        })));
        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 30,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 36,
                left: Some(Rc::new(RefCell::new(TreeNode::new(36)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 35,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(33)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 21,
                left: Some(Rc::new(RefCell::new(TreeNode::new(26)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                }))),
            }))),
        })));

        assert_eq!(Solution::bst_to_gst(root), result);
    }
}
