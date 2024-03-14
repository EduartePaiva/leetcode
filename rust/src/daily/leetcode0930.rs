// 930. Binary Subarrays With Sum
pub struct Solution;
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        //count num of subarray where curSum <= x
        fn helper(goal: i32, nums: &[i32]) -> i32 {
            if goal < 0 {
                return 0;
            }
            let mut res = 0;
            let mut l = 0;
            let mut cur = 0;
            for r in 0..nums.len() {
                cur += nums[r];

                while cur > goal {
                    cur -= nums[l];
                    l += 1;
                }
                res += r - l + 1;
            }
            return res as i32;
        }
        helper(goal, &nums) - helper(goal - 1, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }
}
