// 310. Minimum Height Trees
pub struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // I can check all nodes, but I can have memoization.
        //                   key     (n,depth of n)
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::with_capacity(n as usize);
        for edge in edges {
            let n1 = edge[0];
            let n2 = edge[1];
            adj.entry(n1).or_insert(vec![]).push(n2);
            adj.entry(n2).or_insert(vec![]).push(n1);
        }

        let mut leaves = VecDeque::new();
        let mut edge_cnt = HashMap::new();
        for (src, neighbors) in adj.iter() {
            if neighbors.len() == 1 {
                leaves.push_back(*src);
            }
            edge_cnt.insert(*src, leaves.len());
        }

        // cut leaves until the center node
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
            vec![1]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            ),
            vec![3, 4]
        );
    }
}
