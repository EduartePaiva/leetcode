// 525. Contiguous Array

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, -1);

        let mut res = 0;
        let mut current = 0;
        for (i, val) in nums.iter().enumerate() {
            current += if *val == 0 { -1 } else { 1 };

            match map.get(&current) {
                Some(val) => res = res.max(i as i32 - val),
                None => {
                    map.insert(current, i as i32);
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_max_length(vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1]),
            10
        );
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }
}
