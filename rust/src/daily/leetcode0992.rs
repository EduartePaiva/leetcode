// 992. Subarrays with K Different Integers
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut count = HashMap::new();

        let mut res = 0;
        let mut l_near = 0;
        let mut l_far = 0;
        for r in 0..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;

            while count.len() > k {
                *count.entry(nums[l_near]).or_insert(0) -= 1;
                if count.get(&nums[l_near]).unwrap() == &0 {
                    count.remove(&nums[l_near]);
                }
                l_near += 1;
                l_far = l_near;
            }

            if count.get(&nums[l_near]).unwrap() > &1 {
                *count.entry(nums[l_near]).or_insert(0) -= 1;
                l_near += 1;
            }

            if count.len() == k {
                res += l_near - l_far + 1;
            }
        }

        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
            7
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
            3
        );
    }
}
