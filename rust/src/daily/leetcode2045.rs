// 2045. Second Minimum Time to Reach Destination
pub struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            map.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
            map.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
        }
        let mut res = -1;
        let mut q: VecDeque<i32> = VecDeque::new();
        let mut cur_time = 0;
        let mut visit: HashMap<i32, [i32; 2]> = HashMap::new();
        q.push_back(1);

        while !q.is_empty() {
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                if node == n {
                    if res != -1 {
                        return cur_time;
                    }
                    res = cur_time;
                }
                for &nei in map.get(&node).unwrap() {
                    let nei_times = visit.entry(nei).or_insert([-1, -1]);
                    if nei_times[0] == -1 {
                        nei_times[0] = cur_time;
                        q.push_back(nei);
                        continue;
                    }
                    if nei_times[1] == -1 && nei_times[0] != cur_time {
                        nei_times[1] = cur_time;
                        q.push_back(nei);
                    }
                }
            }
            if (cur_time / change) % 2 == 1 {
                cur_time += change - (cur_time % change);
            }
            cur_time += time;
        }

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
    #[test]
    fn test2() {
        assert_eq!(Solution::second_minimum(2, vec![vec![1, 2]], 3, 2), 11);
    }
}
