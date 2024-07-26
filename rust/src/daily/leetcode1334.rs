use std::i32;

// 1334. Find the City With the Smallest Number of Neighbors at a Threshold Distance
pub struct Solution;

#[derive(Clone)]
struct Node {
    dest: usize,
    weight: i32,
}

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        // I have to run a dfs from each edge
        // let's trust the input
        let mut map: Vec<Vec<Node>> = vec![vec![]; n as usize];
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            let weight = edge[2];
            map[from].push(Node { dest: to, weight });
            map[to].push(Node { dest: from, weight });
        }

        fn get_num_neighbors(
            visited: &mut Vec<bool>,
            dist_left: i32,
            node: usize,
            map: &Vec<Vec<Node>>,
        ) -> i32 {
            if dist_left < 0 || visited[node] {
                return 0;
            }
            visited[node] = true;
            // println!("city: {node}");
            let neis = &map[node];
            let mut res = 0;
            for n in neis {
                if visited[n.dest] {
                    continue;
                }
                res += get_num_neighbors(visited, dist_left - n.weight, n.dest, map);
            }

            res + 1
        }

        let mut nodes_neighbors = vec![n; n as usize];

        for i in 0..n as usize {
            // println!("----------------");
            let mut visited: Vec<bool> = vec![false; n as usize];
            nodes_neighbors[i] = get_num_neighbors(&mut visited, distance_threshold, i, &map);
        }

        let mut res = n - 1;
        let mut min_nei = i32::MAX;
        // println!("{:?}", nodes_neighbors);

        for i in (0..n as usize).rev() {
            if nodes_neighbors[i] < min_nei {
                min_nei = nodes_neighbors[i];
                res = i as i32;
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
            Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            ),
            3
        )
    }
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            ),
            3
        )
    }
}
