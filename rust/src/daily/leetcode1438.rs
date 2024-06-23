// 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as u32;
        // [8,2,4,7]
        //simple sliding window
        let mut res = 0;
        for l in 0..nums.len() {
            let mut min_val = i32::MAX;
            let mut max_val = i32::MIN;
            for r in l..nums.len() {
                min_val = min_val.min(nums[r]);
                max_val = max_val.max(nums[r]);

                if min_val.abs_diff(max_val) > limit {
                    break;
                }
                res = res.max((r - l) as i32 + 1);
            }
            if res >= (nums.len() - l) as i32 {
                break;
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
        assert_eq!(Solution::longest_subarray(vec![8, 2, 4, 7], 4), 2);
    }
}
