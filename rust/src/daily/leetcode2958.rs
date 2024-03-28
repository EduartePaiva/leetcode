// 2958. Length of Longest Subarray With at Most K Frequency
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = HashMap::new();
        let mut res = 0;
        let mut l = 0;

        for r in 0..nums.len() {
            *freq.entry(nums[r]).or_insert(0) += 1;

            while freq.get(&nums[r]).unwrap() > &k && l <= r {
                freq.entry(nums[l]).and_modify(|x| *x -= 1);
                l += 1;
            }

            res = res.max(r + 1 - l);
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
            Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2),
            6
        );
    }
}
