// 947. Most Stones Removed with Same Row or Column
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut connections = (0..stones.len()).collect::<Vec<usize>>();
        let mut rows: HashMap<i32, usize> = HashMap::new();
        let mut cols: HashMap<i32, usize> = HashMap::new();

        fn find(mut x: usize, connections: &mut Vec<usize>) -> usize {
            while connections[x] != x {
                connections[x] = connections[connections[x]];
                x = connections[x];
            }
            x
        }

        fn union(i: usize, j: usize, connections: &mut Vec<usize>) -> i32 {
            let root_i = find(i, connections);
            let root_j = find(j, connections);
            connections[root_i] = root_j;
            if root_i != root_j {
                1
            } else {
                0
            }
        }
        let mut res = 0;
        for (i, stone) in stones.into_iter().enumerate() {
            let [row, col]: [i32; 2] = stone.try_into().unwrap();
            match rows.get(&row) {
                Some(j) => res += union(i, *j, &mut connections),
                None => {
                    rows.insert(row, i);
                }
            }
            match cols.get(&col) {
                Some(j) => res += union(i, *j, &mut connections),
                None => {
                    cols.insert(col, i);
                }
            }
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
            Solution::remove_stones(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![2, 2]
            ]),
            5
        )
    }
}
