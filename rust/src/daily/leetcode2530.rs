// 2530. Maximal Score After Applying K Operations
pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_heap = BinaryHeap::from(nums);
        let mut res = 0;
        for _ in 0..k {
            let n = max_heap.pop().unwrap();
            res += n as i64;
            max_heap.push((n as u32).div_ceil(3) as i32);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
    }
}
