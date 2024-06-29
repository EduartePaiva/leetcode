// 2192. All Ancestors of a Node in a Directed Acyclic Graph
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n as usize];
        //what if I just invert here?
        for edge in edges {
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        // now it turns into a backtracking problem
        // if res already have something  don't need to go down
        fn travel_down(
            cur_node: usize,
            res: &mut Vec<HashSet<usize>>,
            graph: &Vec<Vec<usize>>,
        ) -> HashSet<usize> {
            if graph[cur_node].len() == 0 {
                return HashSet::from([cur_node]);
            }

            let mut cur_answer: HashSet<usize> = HashSet::new();
            for &val in &graph[cur_node] {
                if res[val].len() > 0 {
                    let res_cp = res[val].clone();
                    cur_answer.extend(res_cp);
                    cur_answer.insert(val);
                } else {
                    let res_cp = travel_down(val, res, graph);
                    cur_answer.extend(res_cp);
                }
            }
            res[cur_node] = cur_answer;

            return res[cur_node].clone();
        }
        let mut res: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];

        for i in 0..n as usize {
            travel_down(i, &mut res, &graph);
        }

        return res
            .into_iter()
            .map(|cur| {
                let mut new_set = cur.into_iter().map(|v| v as i32).collect::<Vec<i32>>();
                new_set.sort_unstable();
                new_set
            })
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_ancestors(
                8,
                vec![
                    vec![0, 3],
                    vec![0, 4],
                    vec![1, 3],
                    vec![2, 4],
                    vec![2, 7],
                    vec![3, 5],
                    vec![3, 6],
                    vec![3, 7],
                    vec![4, 6]
                ]
            ),
            vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3]
            ]
        )
    }
}
