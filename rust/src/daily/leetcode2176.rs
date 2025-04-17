// 2176. Count Equal and Divisible Pairs in an Array
pub struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut num_pairs = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] && (i * j) as i32 % k == 0 {
                    num_pairs += 1;
                }
            }
        }

        num_pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4)
    }
}
