// 2097. Valid Arrangement of Pairs
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut nodes: HashMap<i32, Vec<i32>> = HashMap::new();
        for pair in &pairs {
            nodes.entry(pair[1]).or_insert(Vec::new()).push(pair[0]);
            nodes.entry(pair[0]).or_insert(Vec::new());
        }
        let mut nodes_degree: HashMap<i32, i32> = HashMap::new();
        for v in &pairs {
            *nodes_degree.entry(v[1]).or_insert(0) += 1;
            *nodes_degree.entry(v[0]).or_insert(0) -= 1;
        }
        let last = *nodes_degree
            .iter()
            .find(|&(_, &x)| x == 1)
            .or(nodes_degree.iter().next())
            .unwrap()
            .0;
        let mut res: Vec<Vec<i32>> = vec![];
        fn dfs(n: i32, nodes: &mut HashMap<i32, Vec<i32>>, res: &mut Vec<Vec<i32>>) {
            while let Some(prev) = nodes.get_mut(&n).unwrap().pop() {
                dfs(prev, nodes, res);
                res.push(vec![prev, n]);
            }
        }
        dfs(last, &mut nodes, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::valid_arrangement(vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]]),
            vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1]]
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::valid_arrangement(vec![vec![1, 3], vec![3, 2], vec![2, 1]]),
            vec![vec![1, 3], vec![3, 2], vec![2, 1]]
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::valid_arrangement(vec![vec![1, 2], vec![1, 3], vec![2, 1]]),
            vec![vec![1, 2], vec![2, 1], vec![1, 3]]
        )
    }
}
