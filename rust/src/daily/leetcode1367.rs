// 1367. Linked List in Binary Tree
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // two functions.
        // one traverse root.
        // other check if from node X a path can be formed.
        fn travel_root(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if downward_path(head, root) {
                return true;
            }
            if let Some(r) = root {
                let r = r.borrow();
                return travel_root(head, &r.left) || travel_root(head, &r.right);
            }
            false
        }
        fn downward_path(
            head: &Option<Box<ListNode>>,
            root: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if head.is_none() {
                return true;
            }
            if let (Some(h), Some(r)) = (head, root) {
                let r = r.borrow();
                if h.val == r.val {
                    return downward_path(&h.next, &r.left) || downward_path(&h.next, &r.right);
                }
            }
            false
        }
        travel_root(&head, &root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        //head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
        let head = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    }))),
                }))),
                right: None,
            }))),
        })));

        assert_eq!(Solution::is_sub_path(head, root), true);
    }

    #[test]
    fn test2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 10,
                next: None,
            })),
        }));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        })));

        assert_eq!(Solution::is_sub_path(head, root), true);
    }
}
