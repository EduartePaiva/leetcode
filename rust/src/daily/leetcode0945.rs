// 945. Minimum Increment to Make Array Unique
pub struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut res = 0;
        for i in 1..nums.len() {
            if nums[i - 1] >= nums[i] {
                res += (nums[i - 1] - nums[i]) + 1;
                nums[i] = nums[i - 1] + 1;
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
        assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]),
            6
        );
    }
}
