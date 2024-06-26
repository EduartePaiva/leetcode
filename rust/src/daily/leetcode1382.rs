// 1382. Balance a Binary Search Tree
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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sorted_nodes: Vec<i32> = vec![];

        fn disassemble_bst(root: &Option<Rc<RefCell<TreeNode>>>, sorted_nodes: &mut Vec<i32>) {
            if let Some(n) = root {
                let n = n.borrow();
                disassemble_bst(&n.left, sorted_nodes);
                sorted_nodes.push(n.val);
                disassemble_bst(&n.right, sorted_nodes);
            }
        }

        disassemble_bst(&root, &mut sorted_nodes);

        fn build_bst(sorted_nodes: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if sorted_nodes.len() > 0 {
                let half = sorted_nodes.len() / 2;
                let node;
                if sorted_nodes.len() % 2 == 0 {
                    node = TreeNode {
                        left: build_bst(&sorted_nodes[..half - 1]),
                        right: build_bst(&sorted_nodes[half..]),
                        val: sorted_nodes[half - 1],
                    };
                } else {
                    node = TreeNode {
                        left: build_bst(&sorted_nodes[0..half]),
                        right: build_bst(&sorted_nodes[half + 1..]),
                        val: sorted_nodes[half],
                    };
                }
                return Some(Rc::new(RefCell::new(node)));
            }
            None
        }

        build_bst(&sorted_nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // #[ignore = "reason"]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            }))),
        })));

        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        assert_eq!(Solution::balance_bst(root), result);
    }
    #[test]
    fn test2() {
        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(Solution::balance_bst(root), result);
    }
}
