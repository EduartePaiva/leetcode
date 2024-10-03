// 1590. Make Sum Divisible by P
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        // prefixsum sliding window?

        let rem = (nums.iter().map(|x| *x as i128).sum::<i128>() % p as i128) as i32;
        if rem == 0 {
            return 0;
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut res = i32::MAX;
        let mut cur_sum = 0;
        for i in 0..nums.len() {
            cur_sum = (cur_sum + nums[i]) % p;
            let req = (cur_sum - rem + p) % p;
            if let Some(&index) = map.get(&req) {
                res = res.min(i as i32 - index);
            }
            map.insert(cur_sum, i as i32);
        }

        if res == i32::MAX || res == nums.len() as i32 {
            -1
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 7), -1);
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_subarray(
                vec![26, 19, 11, 14, 18, 4, 7, 1, 30, 23, 19, 8, 10, 6, 26, 3],
                26
            ),
            3
        );
    }
}
