// 951. Flip Equivalent Binary Trees
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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if (root1.is_none() && root2.is_some()) || (root1.is_some() && root2.is_none()) {
            return false;
        }
        if let (Some(r1), Some(r2)) = (root1, root2) {
            let (mut r1, mut r2) = (r1.borrow_mut(), r2.borrow_mut());
            if r1.val != r2.val {
                return false;
            }
            let r1_left = r1.left.take();
            let r1_right = r1.right.take();
            let r2_left = r2.left.take();
            let r2_right = r2.right.take();
            // value
            let r1_left_v = if let Some(v) = &r1_left {
                v.borrow().val
            } else {
                -1
            };
            let r2_left_v = if let Some(v) = &r2_left {
                v.borrow().val
            } else {
                -1
            };
            if r1_left_v == r2_left_v {
                return Solution::flip_equiv(r1_left, r2_left)
                    && Solution::flip_equiv(r1_right, r2_right);
            } else {
                return Solution::flip_equiv(r1_left, r2_right)
                    && Solution::flip_equiv(r1_right, r2_left);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
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
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        assert_eq!(Solution::flip_equiv(root1, root2), true);
    }
}
