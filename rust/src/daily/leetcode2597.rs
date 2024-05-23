//2597. The Number of Beautiful Subsets
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        fn dfs(prohibited: &mut HashMap<i32, i32>, nums: &Vec<i32>, i: usize, k: i32) -> i32 {
            if i == nums.len() {
                return 1;
            }
            let mut res = 0;
            // don't include current i
            res += dfs(prohibited, nums, i + 1, k);
            // try including current i
            if prohibited.get(&(nums[i] + k)).unwrap_or(&0) == &0
                && prohibited.get(&(nums[i] - k)).unwrap_or(&0) == &0
            {
                *prohibited.entry(nums[i]).or_insert(0) += 1;
                res += dfs(prohibited, nums, i + 1, k);
                let pro = prohibited.entry(nums[i]).or_insert(0);
                *pro -= 1;
                if pro == &0 {
                    prohibited.remove(&nums[i]);
                }
            }

            res
        }
        let mut prohibited: HashMap<i32, i32> = HashMap::new();
        dfs(&mut prohibited, &nums, 0, k) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::beautiful_subsets(vec![2, 4, 6], 2), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::beautiful_subsets(vec![4, 2, 5, 9, 10, 3], 1), 23);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::beautiful_subsets(vec![1, 1, 2, 3], 1), 8);
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::beautiful_subsets(vec![18, 12, 10, 5, 6, 2, 18, 3], 8),
            143
        );
    }
}
