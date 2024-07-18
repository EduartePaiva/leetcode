// 1530. Number of Good Leaf Nodes Pairs
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
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        //get leaf right and left, then combine them.
        // I just need to get their distance
        let mut res = 0;

        fn backtrack(
            root: &Option<Rc<RefCell<TreeNode>>>,
            res: &mut i32,
            distance: i32,
        ) -> Vec<i32> {
            if let Some(n) = root {
                let n = n.borrow();
                if n.right.is_none() && n.left.is_none() {
                    // this is a leaf node
                    return vec![1];
                }

                let left_nodes = backtrack(&n.left, res, distance);
                let right_nodes = backtrack(&n.right, res, distance);

                for l in &left_nodes {
                    for r in &right_nodes {
                        if l + r <= distance {
                            *res += 1;
                        }
                    }
                }

                let mut nodes = left_nodes
                    .into_iter()
                    .map(|val| val + 1)
                    .filter(|val| val < &distance)
                    .collect::<Vec<i32>>();
                nodes.extend(
                    right_nodes
                        .into_iter()
                        .map(|val| val + 1)
                        .filter(|val| val < &distance),
                );
                return nodes;
            }
            return vec![];
        }

        backtrack(&root, &mut res, distance);

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
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(Solution::count_pairs(root, 3), 1);
    }
}
