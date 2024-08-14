// 719. Find K-th Smallest Pair Distance
pub struct Solution;

impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut start = 0;
        let mut end = nums[nums.len() - 1] - nums[0];
        while start < end {
            let half = (start + end) / 2;
            let mut num_diff = 0;
            let mut r = 0;
            for l in 0..nums.len() {
                let num_to_find = nums[l] + half;
                while r < nums.len() && nums[r] <= num_to_find {
                    r += 1;
                }
                num_diff += (r - l - 1) as i32;
            }
            if num_diff >= k {
                end = half;
            } else {
                start = half + 1;
            }
        }
        end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::smallest_distance_pair(vec![4, 62, 100], 2), 58)
        // 62-100 = 38
        // 4-62 = 58
        // 4-100 = 96
        //
        //
        //
    }
}
