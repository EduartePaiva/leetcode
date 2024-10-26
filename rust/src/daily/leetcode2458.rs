// 2458. Height of Binary Tree After Subtree Removal Queries
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

#[derive(Debug)]
struct Node {
    level: i32,
    height: i32,
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        let mut levels: HashMap<i32, [i32; 2]> = HashMap::new();

        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            cur_level: i32,
            nodes: &mut HashMap<i32, Node>,
            levels: &mut HashMap<i32, [i32; 2]>,
        ) -> i32 {
            if let Some(r) = root {
                let r = r.borrow();
                let cur_h = dfs(&r.left, cur_level + 1, nodes, levels).max(dfs(
                    &r.right,
                    cur_level + 1,
                    nodes,
                    levels,
                )) + 1;

                nodes.insert(
                    r.val,
                    Node {
                        level: cur_level,
                        height: cur_h,
                    },
                );
                let max_2_levels = *levels.get(&cur_level).unwrap_or(&[-1, -1]);
                let mut max_3_levels = Vec::from(max_2_levels);
                max_3_levels.push(cur_h);
                max_3_levels.sort_unstable();
                levels.insert(cur_level, [max_3_levels[2], max_3_levels[1]]);
                return cur_h;
            }
            0
        }
        dfs(&root, 0, &mut nodes, &mut levels);
        // println!("nodes: {:?}", nodes);
        // println!("max 2 levels: {:?}", levels);

        let mut res = vec![];
        for query in queries {
            let Node { level, height } = *nodes.get(&query).unwrap();
            let [l1, l2] = *levels.get(&level).unwrap();
            if height == l1 && l2 != -1 {
                res.push(level + l2 - 1);
            } else if height != l1 {
                res.push(level + l1 - 1);
            } else {
                res.push(level - 1);
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(
            Solution::tree_queries(root, vec![3, 2, 4, 8]),
            vec![3, 2, 3, 2]
        )
    }
}
