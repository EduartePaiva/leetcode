// 2699. Modify Graph Edge Weights
pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
#[derive(Debug, Clone)]
struct Node {
    to: usize,
    can_modify: bool,
    weight: i32,
}

impl Solution {
    fn shortest_path(from: usize, to: usize, adj: &Vec<Vec<Node>>) -> (i32, Option<Vec<usize>>) {
        // this function will find the shortest path.
        // return what it took to find it.
        // return a path to it if it exists.
        let mut heap: BinaryHeap<(Reverse<i32>, Vec<usize>)> = BinaryHeap::new();
        let mut visit: HashMap<usize, i32> = HashMap::new();
        heap.push((Reverse(0), Vec::from([from])));
        visit.insert(from, 0);

        while let Some((Reverse(cur_wei), path)) = heap.pop() {
            let cur_node = *path.last().unwrap();
            if cur_node == to {
                return (cur_wei, Some(path));
            }
            for n in &adj[cur_node] {
                if let Some(w) = visit.get(&n.to) {
                    if cur_wei + n.weight >= *w {
                        continue;
                    }
                }
                visit.insert(n.to, cur_wei + n.weight);
                let mut new_path = path.clone();
                new_path.push(n.to);
                heap.push((Reverse(cur_wei + n.weight), new_path));
            }
        }

        (0, None)
    }

    fn find_and_modify(have: i32, target: i32, path: Vec<usize>, adj: &mut Vec<Vec<Node>>) -> bool {
        // this function will look for the first node that allow modifying
        // if it don't find, return false.
        // if find modify it back and fourth and return true
        for i in 0..path.len() - 1 {
            let from = path[i];
            let to = path[i + 1];

            let adj_from = &mut adj[from];
            // find index of to
            let mut to_index = 0;
            for i in 0..adj_from.len() {
                if adj_from[i].to == to {
                    to_index = i;
                    break;
                }
            }
            // if to can modify do everything below and return true
            if adj_from[to_index].can_modify {
                // edit the weight of to
                let result = target - (have - adj_from[to_index].weight);
                adj_from[to_index].weight = result;
                // now to the opposite of to to from
                // ---------------------------------
                let adj_to = &mut adj[to];
                // find index of from
                let mut from_index = 0;
                for i in 0..adj_to.len() {
                    if adj_to[i].to == from {
                        from_index = i;
                        break;
                    }
                }
                // edit the weight of to
                let result = target - (have - adj_to[from_index].weight);
                adj_to[from_index].weight = result;
                return true;
            }
        }

        false
    }

    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;
        let mut adj: Vec<Vec<Node>> = vec![vec![]; n];

        for edge in &edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            let w = edge[2];
            adj[a].push(Node {
                to: b,
                can_modify: w == -1,
                weight: if w == -1 { 1 } else { w },
            });
            adj[b].push(Node {
                to: a,
                can_modify: w == -1,
                weight: if w == -1 { 1 } else { w },
            })
        }

        loop {
            let (dist, path) = Solution::shortest_path(source, destination, &adj);
            if dist > target || path.is_none() {
                break vec![];
            }
            if dist == target {
                // convert the adj list back to a edge list and return it.
                // scan edges comparing with the adj and modifying it.
                for i in 0..edges.len() {
                    let a = edges[i][0] as usize;
                    let b = edges[i][1] as usize;
                    edges[i][2] = adj[a].iter().find(|ad| ad.to == b).unwrap().weight;
                }
                break edges;
            }

            if !Solution::find_and_modify(dist, target, path.unwrap(), &mut adj) {
                break vec![];
            }
        }
        // first get the shortest "path"
        // if this path is > target return empty array
        // if it's less search for a "editable" node and modify it.
        // if no editable is found return an empty array.
        // keep doing that until the path is equal to target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::modified_graph_edges(
                5,
                vec![
                    vec![4, 1, -1],
                    vec![2, 0, -1],
                    vec![0, 3, -1],
                    vec![4, 3, -1]
                ],
                0,
                1,
                5
            ),
            vec![vec![4, 1, 1], vec![2, 0, 1], vec![0, 3, 3], vec![4, 3, 1]]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::modified_graph_edges(
                5,
                vec![
                    vec![1, 3, 10],
                    vec![4, 2, -1],
                    vec![0, 3, 7],
                    vec![4, 0, 7],
                    vec![3, 2, -1],
                    vec![1, 4, 5],
                    vec![2, 0, 8],
                    vec![1, 0, 3],
                    vec![1, 2, 5]
                ],
                3,
                4,
                11
            ),
            vec![
                vec![1, 3, 10],
                vec![4, 2, 1],
                vec![0, 3, 7],
                vec![4, 0, 7],
                vec![3, 2, 10],
                vec![1, 4, 5],
                vec![2, 0, 8],
                vec![1, 0, 3],
                vec![1, 2, 5]
            ]
        );
    }
}
