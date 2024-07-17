// 1110. Delete Nodes And Return Forest
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
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = VecDeque::new();
        result.push_back(root);

        fn delete_one_from_tree(
            cur: &mut Option<Rc<RefCell<TreeNode>>>,
            result: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
            val: i32,
        ) -> bool {
            if let Some(n) = cur {
                let mut n = n.borrow_mut();
                if let Some(left) = n.left.clone() {
                    let mut left = left.borrow_mut();
                    if left.val == val {
                        if left.left.is_some() {
                            result.push_back(left.left.take());
                        }
                        if left.right.is_some() {
                            result.push_back(left.right.take());
                        }
                        n.left = None;
                        return true;
                    }
                }
                if let Some(right) = n.right.clone() {
                    let mut right = right.borrow_mut();
                    if right.val == val {
                        if right.left.is_some() {
                            result.push_back(right.left.take());
                        }
                        if right.right.is_some() {
                            result.push_back(right.right.take());
                        }
                        n.right = None;
                        return true;
                    }
                }

                return delete_one_from_tree(&mut n.left, result, val)
                    || delete_one_from_tree(&mut n.right, result, val);
            }

            false
        }

        for del in to_delete {
            for _ in 0..result.len() {
                let cur_node = result.pop_front().unwrap();
                if let Some(node) = cur_node.clone() {
                    let mut node = node.borrow_mut();
                    if node.val == del {
                        if node.left.is_some() {
                            result.push_back(node.left.take());
                        }
                        if node.right.is_some() {
                            result.push_back(node.right.take());
                        }
                        break;
                    }
                }
                if delete_one_from_tree(&mut cur_node.clone(), &mut result, del) {
                    result.push_back(cur_node);
                    break;
                }
                result.push_back(cur_node);
            }
        }

        Vec::from(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        let res = vec![
            Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: None,
                }))),
                right: None,
            }))),
        ];

        assert_eq!(Solution::del_nodes(root, vec![3, 5]), res)
    }
    #[test]
    fn test2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: None,
        })));

        let res = vec![
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        ];

        assert_eq!(Solution::del_nodes(root, vec![2, 3]), res)
    }
}
