// 988. Smallest String Starting From Leaf

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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        //bfs makes more sense
        let mut deq: VecDeque<(Vec<u8>, _)> = VecDeque::new();
        let mut char = vec![];
        char.push(b'a' + (root.as_ref().unwrap().borrow().val) as u8);
        deq.push_back((char, root.unwrap()));

        let mut maybe_res: Vec<String> = vec![];

        while !deq.is_empty() {
            for _ in 0..deq.len() {
                let (mut letters, node) = deq.pop_front().unwrap();
                let mut node = node.borrow_mut();
                let left = node.left.take();
                let right = node.right.take();
                if left.is_none() && right.is_none() {
                    //it's a leaf node
                    maybe_res.push(letters.iter().rev().map(|c| *c as char).collect());
                    continue;
                }
                if let Some(n) = left {
                    let char_u8 = n.as_ref().borrow().val as u8 + b'a';
                    let mut new_letters = letters.clone();
                    new_letters.push(char_u8);
                    deq.push_back((new_letters, n));
                }

                if let Some(n) = right {
                    let char_u8 = n.as_ref().borrow().val as u8 + b'a';
                    letters.push(char_u8);
                    deq.push_back((letters, n));
                }
            }
        }
        maybe_res.sort();
        maybe_res.into_iter().next().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        //Some(Rc::new(RefCell::new(TreeNode)))

        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        assert_eq!(Solution::smallest_from_leaf(tree), "dba".to_string());
    }
}
