// 2045. Second Minimum Time to Reach Destination
pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    i32,
};
impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        // do the shortest path.
        // if it don't find a
        // it can be 1 extra move or two
        // two will happen if one extra move don't happen

        //ditch dijkstra for bfs

        // let's do normal shortest path.
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            map.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
            map.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
        }

        let mut paths: Vec<i32> = vec![];
        // start from 1 until n
        heap.push((Reverse(0), 1));
        cache.insert(1, 0);

        println!("{:?}", cache);
        println!("{:?}", map);
        while let Some((Reverse(time_vertex), vertex)) = heap.pop() {
            // check if it's green or red now.

            // println!("foi aqui??????????????????, {vertex}");
            if let Some(neighbors) = map.get(&vertex) {
                println!("{:?}", neighbors);
                for &nei in neighbors {
                    let new_time = time_vertex + time;
                    if nei == n {
                        paths.push(new_time);
                        paths.sort();
                        if paths.len() > 2 {
                            paths.pop();
                        }
                    }
                    if *cache.entry(nei).or_insert(i32::MAX) < (time_vertex + time) {
                        continue;
                    }
                    cache.insert(nei, new_time);
                    // 1 signifies that is red.
                    if (new_time / change) % 2 == 1 {
                        heap.push((Reverse(new_time + (change - (new_time % change))), nei));
                        continue;
                    }

                    heap.push((Reverse(new_time), nei));
                }
            }
        }
        println!("cache: {:?}", cache);

        println!("{:?}", paths);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::second_minimum(
                5,
                vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
                3,
                5
            ),
            13
        );
    }
}
