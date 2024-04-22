// 1971. Find if Path Exists in Graph

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination {
            return true;
        }
        let mut nodes: HashMap<i32, Vec<i32>> = (0..n).map(|x| (x, vec![])).collect();
        for edge in edges {
            let edg1 = edge[0];
            let edg2 = edge[1];
            nodes.get_mut(&edg1).unwrap().push(edg2);
            nodes.get_mut(&edg2).unwrap().push(edg1);
        }

        let mut visit = vec![false; n as usize];
        fn dfs(
            current: i32,
            nodes: &HashMap<i32, Vec<i32>>,
            destination: &i32,
            visit: &mut Vec<bool>,
        ) -> bool {
            if visit[current as usize] {
                return false;
            }
            visit[current as usize] = true;

            let edges = nodes.get(&current).unwrap();
            for edge in edges {
                if edge == destination {
                    return true;
                }
                let possible_path = dfs(*edge, nodes, destination, visit);
                if possible_path == true {
                    return true;
                }
            }

            false
        }

        dfs(source, &nodes, &destination, &mut visit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2),
            true
        );
    }
}
