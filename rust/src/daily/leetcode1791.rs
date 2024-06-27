// 1791. Find Center of Star Graph
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut set = HashSet::new();
        for i in 0..2 {
            for j in 0..2 {
                if !set.insert(edges[i][j]) {
                    return edges[i][j];
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
            2
        );
    }
}
