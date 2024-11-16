// 3254. Find the Power of K-Size Subarrays I
pub struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // brute force solution n*k
        let k = k as usize;
        fn is_sorted(nums: &[i32]) -> i32 {
            for i in 1..nums.len() {
                if nums[i - 1] + 1 != nums[i] {
                    return -1;
                }
            }
            nums[nums.len() - 1]
        }
        let mut res = vec![];
        for i in 0..=nums.len() - k {
            res.push(is_sorted(&nums[i..i + k]));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::results_array(vec![2, 2, 2, 2, 2], 4),
            vec![-1, -1]
        );
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::results_array(vec![1, 3, 4], 2), vec![-1, 4]);
    }
}
