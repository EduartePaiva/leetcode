// 992. Subarrays with K Different Integers
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut count = HashMap::new();
        let mut prefix_count = vec![0; nums.len()];
        let mut prefix_un_count = vec![0; nums.len()];

        for i in 0..nums.len() {
            *count.entry(nums[i]).or_insert(0) += 1;
            prefix_count[i] = count.len();
        }
        let max_cnt = count.len();
        for i in 0..nums.len() {
            *count.entry(nums[i]).or_insert(0) -= 1;
            prefix_un_count[i] = max_cnt - count.len();
            if count.get(&nums[i]).unwrap() == &0 {
                count.remove(&nums[i]);
            }
        }
        println!("{:?}", nums);
        println!("{:?}", prefix_count);
        println!("{:?}", prefix_un_count);

        let mut l = 0;
        let mut res = 0;
        for r in 0..nums.len() {
            if prefix_count[r] - prefix_un_count[l] == k {
                res += 1;
                while prefix_count[r] - prefix_un_count[l] == k {
                    res += 1;
                    l += 1;
                }
            }

            println!("r: {r}, l: {l}");
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
