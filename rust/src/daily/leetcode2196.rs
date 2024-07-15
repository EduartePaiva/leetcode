// 2196. Create Binary Tree From Descriptions
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        //descriptions[i] = [parenti, childi, isLefti]
        //[[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
        let mut map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        for desc in descriptions.iter() {
            if !map.contains_key(&desc[1]) {
                let child_node = Rc::new(RefCell::new(TreeNode::new(desc[1])));
                map.insert(desc[1], child_node);
            }
            let child_node = map.get(&desc[1]).unwrap().clone();
            let node = map
                .entry(desc[0])
                .or_insert(Rc::new(RefCell::new(TreeNode::new(desc[0]))));

            if desc[2] == 1 {
                node.borrow_mut().left.replace(child_node);
            } else {
                node.borrow_mut().right.replace(child_node);
            }
        }
        // remove child nodes
        for desc in descriptions.iter() {
            map.remove(&desc[1]);
        }
        Some(map.into_values().next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 50,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 80,
                left: Some(Rc::new(RefCell::new(TreeNode::new(19)))),
                right: None,
            }))),
        })));

        assert_eq!(
            Solution::create_binary_tree(vec![
                vec![20, 15, 1],
                vec![20, 17, 0],
                vec![50, 20, 1],
                vec![50, 80, 0],
                vec![80, 19, 1]
            ]),
            result
        );
    }
}
