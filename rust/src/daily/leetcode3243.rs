// 3243. Shortest Distance After Road Addition Queries I
pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];

        let mut nodes: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for i in 0..(n - 1) as usize {
            nodes[i].push(i + 1);
        }

        let mut dp = vec![usize::MAX; nodes.len()];
        dp[nodes.len() - 1] = 0;
        fn shortest_dist(nodes: &Vec<Vec<usize>>, dp: &mut Vec<usize>, end: usize) -> i32 {
            for i in (0..=end).rev() {
                for &to in &nodes[i] {
                    dp[i] = dp[i].min(dp[to] + 1);
                }
            }
            dp[0] as i32
        }
        shortest_dist(&nodes, &mut dp, nodes.len() - 2);
        for query in queries {
            let from = query[0] as usize;
            let to = query[1] as usize;
            nodes[from].push(to);
            res.push(shortest_dist(&nodes, &mut dp, from));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
            vec![3, 2, 1]
        )
    }
}
