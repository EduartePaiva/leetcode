// 1863. Sum of All Subset XOR Totals

pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn backtrack(nums: &Vec<i32>, i: usize, acc: i32) -> i32 {
            if i == nums.len() {
                return acc;
            }
            return backtrack(nums, i + 1, acc) + backtrack(nums, i + 1, acc ^ nums[i]);
        }
        backtrack(&nums, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
    }
}
