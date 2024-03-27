// 713. Subarray Product Less Than K

pub struct Solution;
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut l = 0;
        let mut accumulator = 1;

        for r in 0..nums.len() {
            accumulator *= nums[r];
            while accumulator >= k && l <= r {
                accumulator /= nums[l];
                l += 1;
            }
            res += r + 1 - l;
        }

        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }
}
