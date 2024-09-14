// 2419. Longest Subarray With Maximum Bitwise AND
pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut some_n = nums[0];
        let mut res = 0;
        let mut window = 0;
        for n in nums {
            if n == some_n {
                window += 1;
            } else if n > some_n {
                some_n = n;
                res = 1;
                window = 1;
            } else {
                window = 0;
            }
            res = res.max(window);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);
    }
}
