// 834. Sum of Distances in Tree
pub struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        //build the graph. nodes is from 0 to n-1
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for edge in edges {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            graph[n1].push(n2);
            graph[n2].push(n1);
        }
        //brute force dfs
        fn dfs(graph: &Vec<Vec<usize>>, n: usize, prev: usize, depth: i32) -> i32 {
            if graph[n].len() == 1 && graph[n][0] == prev {
                // this is a leave
                return 0;
            }

            let next_nodes = &graph[n];
            let mut acc = 0;
            for node in next_nodes {
                if *node == prev {
                    continue;
                }
                acc += depth + dfs(graph, *node, n, depth + 1);
            }

            acc
        }

        let mut res = vec![0; n as usize];
        for i in 0..n as usize {
            res[i] = dfs(&graph, i, n as usize, 1);
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
            Solution::sum_of_distances_in_tree(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]
            ),
            vec![8, 12, 6, 10, 10, 10]
        );
    }
}
