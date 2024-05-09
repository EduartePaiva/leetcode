// 3075. Maximize Happiness of Selected Children
pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, mut k: i32) -> i64 {
        let mut heap = BinaryHeap::from(happiness);
        let mut decreased_happiness = 0;
        let mut result = 0_i64;

        while let Some(child) = heap.pop() {
            if child - decreased_happiness <= 0 || k <= 0 {
                break;
            }
            result += (child - decreased_happiness) as i64;
            decreased_happiness += 1;
            k -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 2, 3], 2), 4);
    }
}
