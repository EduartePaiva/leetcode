//623. Add One Row to Tree
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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, cur_dep: i32, depth: i32, val: i32) {
            if let Some(new_n) = node {
                if cur_dep == (depth - 1) {
                    let mut n = new_n.borrow_mut();
                    let old_left = n.left.take();
                    let old_right = n.right.take();
                    n.left.replace(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: old_left,
                        right: None,
                    })));

                    n.right.replace(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: None,
                        right: old_right,
                    })));
                } else {
                    let n = new_n.borrow();
                    dfs(n.left.clone(), cur_dep + 1, depth, val);
                    dfs(n.right.clone(), cur_dep + 1, depth, val);
                }
            }
        }
        dfs(root.clone(), 1, depth, val);
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: None,
            }))),
        })));
        let tree2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None,
                }))),
                left: None,
            }))),
        })));

        assert_eq!(Solution::add_one_row(tree1, 1, 2), tree2);
    }
}
