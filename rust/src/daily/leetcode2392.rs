// 2392. Build a Matrix With Conditions
pub struct Solution;

#[derive(Clone)]
struct Place {
    row: usize,
    col: usize,
}

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;
        fn dfs(
            node: usize,
            visit: &mut Vec<bool>,
            path: &mut Vec<bool>,
            result: &mut Vec<usize>,
            adj: &Vec<Vec<usize>>,
        ) -> bool {
            if path[node] {
                return false;
            }
            if visit[node] {
                return true;
            }
            path[node] = true;
            visit[node] = true;

            for &n in &adj[node] {
                if !dfs(n, visit, path, result, adj) {
                    return false;
                }
            }

            path[node] = false;
            result.push(node);

            true
        }

        fn topo_sort(conditions: Vec<Vec<i32>>, k: usize) -> Vec<usize> {
            let mut visit: Vec<bool> = vec![false; k + 1];
            let mut path: Vec<bool> = vec![false; k + 1];
            let mut result: Vec<usize> = vec![];
            let mut adj: Vec<Vec<usize>> = vec![vec![]; k + 1];
            for val in conditions {
                adj[val[0] as usize].push(val[1] as usize);
            }
            for node in 1..=k {
                if !dfs(node, &mut visit, &mut path, &mut result, &mut adj) {
                    return vec![];
                }
            }

            result.reverse();
            return result;
        }

        let mut place_of_k: Vec<Place> = vec![Place { row: 0, col: 0 }; k + 1];

        let sorted_col = topo_sort(col_conditions, k);
        if sorted_col.is_empty() {
            return vec![];
        }
        for (col, key) in sorted_col.into_iter().enumerate() {
            place_of_k[key].col = col;
        }
        let sorted_row = topo_sort(row_conditions, k);
        if sorted_row.is_empty() {
            return vec![];
        }
        for (row, key) in sorted_row.into_iter().enumerate() {
            place_of_k[key].row = row;
        }

        let mut res = vec![vec![0; k as usize]; k as usize];

        for (num, Place { row, col }) in place_of_k.into_iter().enumerate().skip(1) {
            res[row][col] = num as i32;
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
            Solution::build_matrix(
                3,
                vec![vec![1, 2], vec![3, 2]],
                vec![vec![2, 1], vec![3, 2]]
            ),
            vec![vec![3, 0, 0], vec![0, 0, 1], vec![0, 2, 0]]
        )
    }
}
