// 2044. Count Number of Maximum Bitwise-OR Subsets
pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max_or = nums.iter().fold(0, |prev, cur| prev | cur);
        fn dfs(i: usize, max_or: i32, cur: i32, nums: &Vec<i32>) -> i32 {
            if i == nums.len() {
                return if cur == max_or { 1 } else { 0 };
            }
            return dfs(i + 1, max_or, cur, &nums) + dfs(i + 1, max_or, cur | nums[i], &nums);
        }
        dfs(0, max_or, 0, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
    }
}
