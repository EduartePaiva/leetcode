// 1509. Minimum Difference Between Largest and Smallest Value in Three Moves
pub struct Solution;

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }
        nums.sort_unstable();
        let max = nums.len() - 1;
        let mut res = i32::MAX;
        for i in 0..=3 {
            res = res.min(nums[max - 3 + i] - nums[i]);
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_difference(vec![5, 3, 2, 4]), 0);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::min_difference(vec![1, 5, 0, 10, 14]), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::min_difference(vec![6, 6, 0, 1, 1, 4, 6]), 2);
    }
}
