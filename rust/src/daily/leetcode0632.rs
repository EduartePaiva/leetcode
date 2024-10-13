// 632. Smallest Range Covering Elements from K Lists
pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut left = i32::MAX;
        let mut right = i32::MIN;
        // the value, list index, num index
        let mut min_heap: BinaryHeap<(Reverse<i32>, (usize, usize))> = BinaryHeap::new();
        for (i, num) in nums.iter().enumerate() {
            min_heap.push((Reverse(num[0]), (i, 0)));
            left = left.min(num[0]);
            right = right.max(num[0]);
        }

        let mut res_r = right;
        let mut res_l = left;
        loop {
            let (_, (lst_idx, n_idx)) = min_heap.pop().unwrap();
            if n_idx + 1 == nums[lst_idx].len() {
                break vec![res_l, res_r];
            }
            let nxt_val = nums[lst_idx][n_idx + 1];
            min_heap.push((Reverse(nxt_val), (lst_idx, n_idx + 1)));
            right = right.max(nxt_val);
            left = min_heap.peek().unwrap().0 .0;
            if right - left < res_r - res_l {
                res_r = right;
                res_l = left;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::smallest_range(vec![
                vec![4, 10, 15, 24, 26],
                vec![0, 9, 12, 20],
                vec![5, 18, 22, 30]
            ]),
            vec![20, 24]
        )
    }
}
