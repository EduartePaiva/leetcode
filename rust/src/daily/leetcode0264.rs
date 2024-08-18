// 264. Ugly Number II
pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
impl Solution {
    pub fn nth_ugly_number(mut n: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let mut visit = HashSet::new();
        heap.push(Reverse(1));
        visit.insert(1);

        loop {
            let min = heap.pop().unwrap().0;
            n -= 1;
            if n == 0 {
                break min as i32;
            }
            if !visit.contains(&(min * 2)) {
                heap.push(Reverse(min * 2));
                visit.insert(min * 2);
            }
            if !visit.contains(&(min * 3)) {
                heap.push(Reverse(min * 3));
                visit.insert(min * 3);
            }
            if !visit.contains(&(min * 5)) {
                heap.push(Reverse(min * 5));
                visit.insert(min * 5);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::nth_ugly_number(10), 12)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::nth_ugly_number(1352), 402653184)
    }
}
