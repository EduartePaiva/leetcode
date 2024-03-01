//1609. Even Odd Tree
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
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut q = VecDeque::new();
        let mut is_even = false;
        q.push_back(root);

        while !q.is_empty() {
            is_even = !is_even;
            let mut prev_val = if is_even { i32::MIN } else { i32::MAX };

            for _ in 0..q.len() {
                if let Some(Some(n)) = q.pop_front() {
                    let n = n.borrow();
                    let cur_val = n.val;
                    if is_even {
                        if cur_val % 2 == 0 || cur_val <= prev_val {
                            return false;
                        }
                    } else {
                        if cur_val % 2 != 0 || cur_val >= prev_val {
                            return false;
                        }
                    }
                    q.push_back(n.left.clone());
                    q.push_back(n.right.clone());
                    prev_val = cur_val;
                }
            }
        }

        true
    }
}
