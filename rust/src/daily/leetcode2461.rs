// 2461. Maximum Sum of Distinct Subarrays With Length K
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut max_sum = 0;
        let mut cur_sum = 0;
        let mut nums_cnt: HashMap<i32, u32> = HashMap::new();

        for i in 0..k {
            cur_sum += nums[i] as i64;
            *nums_cnt.entry(nums[i]).or_insert(0) += 1;
        }

        if nums_cnt.len() == k {
            max_sum = max_sum.max(cur_sum);
        }

        for i in 1..nums.len() - k + 1 {
            cur_sum -= nums[i - 1] as i64;
            let to_decrease = nums_cnt.entry(nums[i - 1]).or_insert(1);
            *to_decrease -= 1;
            if to_decrease == &0 {
                nums_cnt.remove(&nums[i - 1]);
            }
            cur_sum += nums[i + k - 1] as i64;
            *nums_cnt.entry(nums[i + k - 1]).or_insert(0) += 1;
            if nums_cnt.len() == k {
                max_sum = max_sum.max(cur_sum);
            }
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3),
            15
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![1, 1, 1, 7, 8, 9], 3),
            24
        );
    }
}
