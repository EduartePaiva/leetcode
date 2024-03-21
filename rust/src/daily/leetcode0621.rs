// 621. Task Scheduler

pub struct Solution;

use std::collections::{BinaryHeap, HashMap, VecDeque};
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count = HashMap::new();
        for task in tasks {
            *count.entry(task).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::from_iter(count.into_values());
        let mut deq = VecDeque::new();
        let mut time = 0;

        while !heap.is_empty() || !deq.is_empty() {
            time += 1;
            if let Some(cnt) = heap.pop() {
                if cnt > 1 {
                    deq.push_back((time + n, cnt - 1))
                }
            }
            if !deq.is_empty() && deq[0].0 == time {
                if let Some((_, cnt)) = deq.pop_front() {
                    heap.push(cnt);
                }
            }
        }
        time
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
    }
}
