// 834. Sum of Distances in Tree
pub struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        //build the graph. nodes is from 0 to n-1
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n as usize];
        let mut count: Vec<i32> = vec![0; n as usize];
        let mut res: Vec<i32> = vec![0; n as usize];
        for edge in edges {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            graph[n1].push(n2);
            graph[n2].push(n1);
        }

        fn dfs1(
            cur: usize,
            parent: usize,
            graph: &Vec<Vec<usize>>,
            count: &mut Vec<i32>,
            res: &mut Vec<i32>,
        ) {
            count[cur] = 1;
            let children = &graph[cur];
            for &child in children {
                if child != parent {
                    dfs1(child, cur, graph, count, res);
                    count[cur] += count[child];
                    res[cur] += res[child] + count[child];
                }
            }
        }

        fn dfs2(
            cur: usize,
            parent: usize,
            n: usize,
            count: &Vec<i32>,
            res: &mut Vec<i32>,
            graph: &Vec<Vec<usize>>,
        ) {
            let children = &graph[cur];
            for &child in children {
                if child != parent {
                    res[child] = res[cur] + n as i32 - 2 * count[child];
                    dfs2(child, cur, n, count, res, graph);
                }
            }
        }

        dfs1(0, n as usize, &graph, &mut count, &mut res);
        dfs2(0, n as usize, n as usize, &count, &mut res, &graph);

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
