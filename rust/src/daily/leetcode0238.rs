// 238. Product of Array Except Self
pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut prefix = vec![1; length];
        for i in 1..length {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }

        let mut acumulator = 1;
        for i in (0..length).rev() {
            prefix[i] *= acumulator;
            acumulator *= nums[i];
        }

        prefix
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        )
    }
}
