// 1248. Count Number of Nice Subarrays
pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        // sliding window
        let mut res = 0;
        let mut odd = 0;
        let mut m = 0;
        let mut l = 0;
        for r in 0..nums.len() {
            odd += nums[r] % 2;
            while odd > k {
                odd -= nums[l] % 2;
                l += 1;
                m = l;
            }
            if odd == k {
                while nums[m] % 2 == 0 {
                    m += 1;
                }
                res += (m - l) as i32 + 1;
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
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 1, 1, 1], 1), 5);
    }
}
