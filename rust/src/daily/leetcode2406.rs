// 2406. Divide Intervals Into Minimum Number of Groups
pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|x| x[0]);
        let mut groups = BinaryHeap::new();
        groups.push(Reverse(0));

        for inter in intervals {
            let left = inter[0];
            let right = inter[1];
            let smallest = groups.pop().unwrap().0;
            if smallest < left {
                groups.push(Reverse(right));
            } else {
                groups.push(Reverse(right));
                groups.push(Reverse(smallest));
            }
        }

        groups.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_groups(vec![
                vec![5, 10],
                vec![6, 8],
                vec![1, 5],
                vec![2, 3],
                vec![1, 10]
            ]),
            3
        );
    }
}
