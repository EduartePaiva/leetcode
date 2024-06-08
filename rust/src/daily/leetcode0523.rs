// 523. Continuous Subarray Sum
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        seen.insert(0, -1);
        let mut sum = 0;
        for (i, num) in nums.into_iter().enumerate() {
            sum += num;
            if seen.contains_key(&(sum % k)) {
                if i as i32 - *seen.get(&&(sum % k)).unwrap() >= 2 {
                    return true;
                }
            } else {
                seen.insert(sum % k, i as i32);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 6], 7), true);
    }
}
