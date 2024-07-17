// 2096. Step-By-Step Directions From a Binary Tree Node to Another

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
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        fn find_local(
            root: &Option<Rc<RefCell<TreeNode>>>,
            local_value: i32,
            history: &mut Vec<char>,
        ) -> bool {
            if let Some(n) = root {
                let n = n.borrow();
                if n.val == local_value {
                    return true;
                } else {
                    history.push('L');
                    if find_local(&n.left, local_value, history) {
                        return true;
                    }
                    history.pop();
                    history.push('R');
                    if find_local(&n.right, local_value, history) {
                        return true;
                    }
                    history.pop();
                }
            }
            false
        }
        let mut path_dest = Vec::new();
        let mut path_start = Vec::new();
        find_local(&root, dest_value, &mut path_dest);
        find_local(&root, start_value, &mut path_start);
        let mut path_dest = VecDeque::from(path_dest);
        let mut path_start = VecDeque::from(path_start);
        while !path_dest.is_empty()
            && !path_start.is_empty()
            && path_dest.front().unwrap() == path_start.front().unwrap()
        {
            path_dest.pop_front();
            path_start.pop_front();
        }
        let mut path = (0..path_start.len()).map(|_| 'U').collect::<String>();
        path.extend(path_dest.into_iter());

        return path;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        // Input: root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
        // Output: "UURL"
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        assert_eq!(Solution::get_directions(root, 3, 6), "UURL".to_string());
    }
}
